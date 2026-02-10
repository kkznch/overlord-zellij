use std::env;
use std::path::PathBuf;
use std::sync::Arc;

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::*;
use rmcp::schemars::JsonSchema;
use rmcp::{tool, tool_handler, tool_router, ErrorData as McpError, ServerHandler, ServiceExt};
use serde::Deserialize;

use super::notify;
use super::store::MessageStore;
use super::types::{is_valid_role, Priority, Status, ALL_ROLES};
use crate::logging;

const SESSION_NAME: &str = "overlord";

// --- Tool request types ---

#[derive(Debug, Deserialize, JsonSchema)]
pub struct SendMessageRequest {
    /// Target role: overlord, strategist, inferno, glacier, shadow, storm
    pub to: String,
    /// Brief subject (max 80 chars)
    pub subject: String,
    /// Message body. Keep compact: task name, changed files, summary, next action. Reference file paths, not full code.
    pub body: String,
    /// Priority: normal or urgent. Default: normal
    pub priority: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct CheckInboxRequest {
    /// Whether to mark messages as read. Default: true
    pub mark_read: Option<bool>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct GetStatusRequest {
    /// Role to check: overlord, strategist, inferno, glacier, shadow, storm, or 'all'
    pub role: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct UpdateStatusRequest {
    /// Status: idle, working, blocked, done
    pub status: String,
    /// Brief description of current task (max 120 chars)
    pub task: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct BroadcastRequest {
    /// Brief subject (max 80 chars)
    pub subject: String,
    /// Message body. Keep compact.
    pub body: String,
    /// Priority: normal or urgent. Default: normal
    pub priority: Option<String>,
}

// --- Service ---

#[derive(Clone)]
pub struct RelayService {
    role: String,
    store: Arc<MessageStore>,
    session_name: String,
    plugin_path: String,
    tool_router: ToolRouter<Self>,
}

fn parse_priority(s: Option<&str>) -> Priority {
    match s {
        Some("urgent") => Priority::Urgent,
        _ => Priority::Normal,
    }
}

fn parse_status(s: &str) -> Result<Status, McpError> {
    match s {
        "idle" => Ok(Status::Idle),
        "working" => Ok(Status::Working),
        "blocked" => Ok(Status::Blocked),
        "done" => Ok(Status::Done),
        _ => Err(McpError::invalid_params(
            format!("Invalid status '{}'. Use: idle, working, blocked, done", s),
            None,
        )),
    }
}

#[tool_router]
impl RelayService {
    pub fn new(role: String, store: Arc<MessageStore>, session_name: String, plugin_path: String) -> Self {
        Self {
            role,
            store,
            session_name,
            plugin_path,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Send a message to another role. The target role will be automatically notified via their Zellij pane."
    )]
    async fn send_message(
        &self,
        Parameters(req): Parameters<SendMessageRequest>,
    ) -> Result<CallToolResult, McpError> {
        if !is_valid_role(&req.to) {
            return Err(McpError::invalid_params(
                format!("Invalid role '{}'. Valid: {:?}", req.to, ALL_ROLES),
                None,
            ));
        }
        if req.to == self.role {
            return Err(McpError::invalid_params(
                "Cannot send message to yourself",
                None,
            ));
        }

        logging::debug(&format!("send_message: from={} to={} subject={}", self.role, req.to, req.subject));

        let priority = parse_priority(req.priority.as_deref());
        self.store
            .send_message(&self.role, &req.to, &req.subject, &req.body, priority)
            .map_err(|e| McpError::internal_error(format!("Failed to send: {}", e), None))?;

        // Auto-trigger: set pending flag and inject notification if needed
        let should_notify = self
            .store
            .set_pending(&req.to)
            .map_err(|e| {
                McpError::internal_error(format!("Failed to set pending: {}", e), None)
            })?;

        if should_notify {
            if let Err(e) = notify::notify_pane(&self.session_name, &req.to, &self.role, &self.plugin_path) {
                logging::error(&format!("notify failed: from={} to={} err={}", self.role, req.to, e));
                return Ok(CallToolResult::success(vec![Content::text(format!(
                    "Message sent to {} (auto-notification failed: {}). Target should check_inbox manually.",
                    req.to, e
                ))]));
            }
        }

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Message sent to {}: {}",
            req.to, req.subject
        ))]))
    }

    #[tool(
        description = "Check inbox for unread messages. Call this when you receive a [MESSAGE from ...] notification, or at the start of each turn."
    )]
    async fn check_inbox(
        &self,
        Parameters(req): Parameters<CheckInboxRequest>,
    ) -> Result<CallToolResult, McpError> {
        let mark_read = req.mark_read.unwrap_or(true);
        let messages = self
            .store
            .check_inbox(&self.role, mark_read)
            .map_err(|e| {
                McpError::internal_error(format!("Failed to check inbox: {}", e), None)
            })?;

        if messages.is_empty() {
            return Ok(CallToolResult::success(vec![Content::text(
                "No unread messages.",
            )]));
        }

        let mut output = format!("{} unread message(s):\n", messages.len());
        for (i, msg) in messages.iter().enumerate() {
            output.push_str(&format!(
                "\n[{}] From: {} | Priority: {} | Subject: {}\nBody: {}\n",
                i + 1,
                msg.from,
                msg.priority,
                msg.subject,
                msg.body,
            ));
        }
        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    #[tool(description = "Get the current status of a role, or 'all' for everyone.")]
    async fn get_status(
        &self,
        Parameters(req): Parameters<GetStatusRequest>,
    ) -> Result<CallToolResult, McpError> {
        if req.role == "all" {
            let statuses = self.store.get_all_statuses().map_err(|e| {
                McpError::internal_error(format!("Failed to get statuses: {}", e), None)
            })?;
            let mut output = String::from("Army Status:\n");
            for s in statuses {
                output.push_str(&format!(
                    "  {}: {} {}\n",
                    s.role,
                    s.status,
                    s.task
                        .as_deref()
                        .map(|t| format!("({})", t))
                        .unwrap_or_default()
                ));
            }
            return Ok(CallToolResult::success(vec![Content::text(output)]));
        }

        if !is_valid_role(&req.role) {
            return Err(McpError::invalid_params(
                format!("Invalid role '{}'. Valid: {:?} or 'all'", req.role, ALL_ROLES),
                None,
            ));
        }

        let text = match self.store.get_status(&req.role).map_err(|e| {
            McpError::internal_error(format!("Failed to get status: {}", e), None)
        })? {
            Some(s) => format!(
                "{}: {} {}",
                s.role,
                s.status,
                s.task
                    .as_deref()
                    .map(|t| format!("({})", t))
                    .unwrap_or_default()
            ),
            None => format!("{}: unknown (no status recorded)", req.role),
        };
        Ok(CallToolResult::success(vec![Content::text(text)]))
    }

    #[tool(description = "Update your own status so other roles can see what you are doing.")]
    async fn update_status(
        &self,
        Parameters(req): Parameters<UpdateStatusRequest>,
    ) -> Result<CallToolResult, McpError> {
        let status = parse_status(&req.status)?;
        self.store
            .update_status(&self.role, status, req.task.as_deref())
            .map_err(|e| {
                McpError::internal_error(format!("Failed to update status: {}", e), None)
            })?;
        Ok(CallToolResult::success(vec![Content::text(format!(
            "Status updated to: {}",
            req.status
        ))]))
    }

    #[tool(
        description = "Broadcast a message to all other roles. Use sparingly - typically only for Overlord announcements or Strategist task assignments."
    )]
    async fn broadcast(
        &self,
        Parameters(req): Parameters<BroadcastRequest>,
    ) -> Result<CallToolResult, McpError> {
        let priority = parse_priority(req.priority.as_deref());
        let mut sent_to = Vec::new();

        for role in ALL_ROLES {
            if *role == self.role {
                continue;
            }
            self.store
                .send_message(&self.role, role, &req.subject, &req.body, priority.clone())
                .map_err(|e| {
                    McpError::internal_error(format!("Failed to send to {}: {}", role, e), None)
                })?;

            let should_notify = self.store.set_pending(role).map_err(|e| {
                McpError::internal_error(
                    format!("Failed to set pending for {}: {}", role, e),
                    None,
                )
            })?;

            if should_notify {
                if let Err(e) = notify::notify_pane(&self.session_name, role, &self.role, &self.plugin_path) {
                    logging::error(&format!("broadcast notify failed: to={} err={}", role, e));
                }
            }
            sent_to.push(*role);
        }

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Broadcast sent to {}: {}",
            sent_to.join(", "),
            req.subject
        ))]))
    }
}

#[tool_handler]
impl ServerHandler for RelayService {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation::from_build_env(),
            instructions: Some(format!(
                "Overlord Army Relay for role: {}. Use tools to communicate with other roles.",
                self.role
            )),
        }
    }
}

/// Run the MCP relay server (entry point for `ovld relay`)
pub async fn serve() -> anyhow::Result<()> {
    let role = env::var("OVLD_ROLE")
        .unwrap_or_else(|_| panic!("OVLD_ROLE environment variable must be set"));

    if env::var("OVLD_DEBUG").is_ok() {
        logging::init(&format!("relay-{}", role));
    }

    if !is_valid_role(&role) {
        anyhow::bail!("Invalid OVLD_ROLE '{}'. Valid: {:?}", role, ALL_ROLES);
    }

    let relay_dir = env::var("OVLD_RELAY_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = env::var("HOME").expect("HOME not set");
            PathBuf::from(home)
                .join(".config")
                .join("ovld")
                .join("relay")
        });

    let session_name = env::var("OVLD_SESSION").unwrap_or_else(|_| SESSION_NAME.to_string());

    let plugin_path = env::var("OVLD_PLUGIN_PATH")
        .unwrap_or_else(|_| String::new());

    let store = Arc::new(MessageStore::new(relay_dir));

    logging::info(&format!("MCP relay started: role={} session={}", role, session_name));

    let service = RelayService::new(role, store, session_name, plugin_path);

    let server = service
        .serve(rmcp::transport::stdio())
        .await
        .expect("Failed to start MCP server");

    server.waiting().await?;

    Ok(())
}
