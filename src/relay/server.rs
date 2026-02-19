use std::env;
use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Context;

use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::*;
use rmcp::schemars::JsonSchema;
use rmcp::{tool, tool_handler, tool_router, ErrorData as McpError, ServerHandler, ServiceExt};
use serde::Deserialize;

use super::notify;
use super::store::MessageStore;
use super::types::{Priority, Status};
use crate::army::roles::Role;
use crate::logging;

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

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ShareInsightRequest {
    /// Category: architecture, debugging, pattern, gotcha, performance
    pub category: String,
    /// Brief title (max 80 chars)
    pub title: String,
    /// What was learned, why it matters, and how to apply it
    pub content: String,
    /// Optional tags for searchability
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct QueryInsightsRequest {
    /// Filter by category: architecture, debugging, pattern, gotcha, performance
    pub category: Option<String>,
    /// Search keyword (matches title, content, and tags)
    pub keyword: Option<String>,
    /// Max results to return. Default: 10
    pub limit: Option<usize>,
}

// --- Service ---

#[derive(Clone)]
pub struct RelayService {
    role: Role,
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
    pub fn new(role: Role, store: Arc<MessageStore>, session_name: String, plugin_path: String) -> Self {
        Self {
            role,
            store,
            session_name,
            plugin_path,
            tool_router: Self::tool_router(),
        }
    }

    #[tool(
        description = "Send a message to an allowed role. Routes: overlord <-> strategist <-> shitennoh. The target will be auto-notified."
    )]
    async fn send_message(
        &self,
        Parameters(req): Parameters<SendMessageRequest>,
    ) -> Result<CallToolResult, McpError> {
        let target: Role = req.to.parse().map_err(|e: String| {
            McpError::invalid_params(e, None)
        })?;
        if target == self.role {
            return Err(McpError::invalid_params(
                "Cannot send message to yourself",
                None,
            ));
        }
        if !self.role.can_send_to(target) {
            return Err(McpError::invalid_params(
                format!(
                    "Route {} -> {} is not allowed. You can send to: {:?}",
                    self.role, target,
                    self.role.allowed_targets().iter().map(|r| r.as_str()).collect::<Vec<_>>()
                ),
                None,
            ));
        }

        logging::debug(&format!("send_message: from={} to={} subject={}", self.role, target, req.subject));

        let priority = parse_priority(req.priority.as_deref());
        self.store
            .send_message(self.role, target, &req.subject, &req.body, priority)
            .map_err(|e| McpError::internal_error(format!("Failed to send: {}", e), None))?;

        // Auto-trigger: set pending flag and inject notification if needed
        let should_notify = self
            .store
            .set_pending(target.as_str())
            .map_err(|e| {
                McpError::internal_error(format!("Failed to set pending: {}", e), None)
            })?;

        if should_notify
            && let Err(e) = notify::notify_pane(&self.session_name, target, self.role, &self.plugin_path) {
                logging::error(&format!("notify failed: from={} to={} err={}", self.role, target, e));
                return Ok(CallToolResult::success(vec![Content::text(format!(
                    "Message sent to {} (auto-notification failed: {}). Target should check_inbox manually.",
                    target, e
                ))]));
            }

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Message sent to {}: {}",
            target, req.subject
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
            .check_inbox(self.role.as_str(), mark_read)
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

        let role: Role = req.role.parse().map_err(|e: String| {
            McpError::invalid_params(
                format!("{}. Or use 'all'.", e),
                None,
            )
        })?;

        let text = match self.store.get_status(role.as_str()).map_err(|e| {
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
            None => format!("{}: unknown (no status recorded)", role),
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
            .update_status(self.role, status, req.task.as_deref())
            .map_err(|e| {
                McpError::internal_error(format!("Failed to update status: {}", e), None)
            })?;
        Ok(CallToolResult::success(vec![Content::text(format!(
            "Status updated to: {}",
            req.status
        ))]))
    }

    #[tool(
        description = "Broadcast a message to all roles you are allowed to contact. Routes: overlord <-> strategist <-> shitennoh."
    )]
    async fn broadcast(
        &self,
        Parameters(req): Parameters<BroadcastRequest>,
    ) -> Result<CallToolResult, McpError> {
        let priority = parse_priority(req.priority.as_deref());
        let mut sent_to = Vec::new();

        for target in self.role.allowed_targets() {
            self.store
                .send_message(self.role, target, &req.subject, &req.body, priority.clone())
                .map_err(|e| {
                    McpError::internal_error(format!("Failed to send to {}: {}", target, e), None)
                })?;

            let should_notify = self.store.set_pending(target.as_str()).map_err(|e| {
                McpError::internal_error(
                    format!("Failed to set pending for {}: {}", target, e),
                    None,
                )
            })?;

            if should_notify
                && let Err(e) = notify::notify_pane(&self.session_name, target, self.role, &self.plugin_path) {
                    logging::error(&format!("broadcast notify failed: to={} err={}", target, e));
                }
            sent_to.push(target.as_str());
        }

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Broadcast sent to {}: {}",
            sent_to.join(", "),
            req.subject
        ))]))
    }

    #[tool(
        description = "Record a learning or discovery to the army's shared knowledge base. Persists across sessions. Use when you discover a non-obvious pattern, debug a tricky issue, or learn something about the codebase architecture."
    )]
    async fn share_insight(
        &self,
        Parameters(req): Parameters<ShareInsightRequest>,
    ) -> Result<CallToolResult, McpError> {
        let tags = req.tags.unwrap_or_default();

        logging::debug(&format!(
            "share_insight: from={} category={} title={}",
            self.role, req.category, req.title
        ));

        let insight = self
            .store
            .store_insight(self.role, &req.category, &req.title, &req.content, tags)
            .map_err(|e| {
                McpError::internal_error(format!("Failed to store insight: {}", e), None)
            })?;

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Insight recorded [{}]: {} (category: {}, from: {})",
            insight.id, insight.title, insight.category, insight.from
        ))]))
    }

    #[tool(
        description = "Search the army's shared knowledge base. Use at session start to load relevant context, or when facing a familiar problem. Knowledge persists across sessions."
    )]
    async fn query_insights(
        &self,
        Parameters(req): Parameters<QueryInsightsRequest>,
    ) -> Result<CallToolResult, McpError> {
        let limit = req.limit.unwrap_or(10);

        let insights = self
            .store
            .query_insights(req.category.as_deref(), req.keyword.as_deref(), limit)
            .map_err(|e| {
                McpError::internal_error(format!("Failed to query insights: {}", e), None)
            })?;

        if insights.is_empty() {
            return Ok(CallToolResult::success(vec![Content::text(
                "No insights found.",
            )]));
        }

        let mut output = format!("{} insight(s) found:\n", insights.len());
        for (i, insight) in insights.iter().enumerate() {
            output.push_str(&format!(
                "\n[{}] ({}) {} — by {}\n    {}\n    tags: {}\n",
                i + 1,
                insight.category,
                insight.title,
                insight.from,
                insight.content,
                if insight.tags.is_empty() {
                    "none".to_string()
                } else {
                    insight.tags.join(", ")
                }
            ));
        }
        Ok(CallToolResult::success(vec![Content::text(output)]))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_priority_urgent() {
        assert!(matches!(parse_priority(Some("urgent")), Priority::Urgent));
    }

    #[test]
    fn test_parse_priority_default() {
        assert!(matches!(parse_priority(None), Priority::Normal));
        assert!(matches!(parse_priority(Some("normal")), Priority::Normal));
        assert!(matches!(parse_priority(Some("invalid")), Priority::Normal));
    }

    #[test]
    fn test_parse_status_valid() {
        assert!(matches!(parse_status("idle"), Ok(Status::Idle)));
        assert!(matches!(parse_status("working"), Ok(Status::Working)));
        assert!(matches!(parse_status("blocked"), Ok(Status::Blocked)));
        assert!(matches!(parse_status("done"), Ok(Status::Done)));
    }

    #[test]
    fn test_parse_status_invalid() {
        assert!(parse_status("invalid").is_err());
        assert!(parse_status("").is_err());
        assert!(parse_status("IDLE").is_err()); // case-sensitive
    }
}

/// Run the MCP relay server (entry point for `ovld relay`)
pub async fn serve() -> anyhow::Result<()> {
    let role_str = env::var("OVLD_ROLE")
        .context("OVLD_ROLE environment variable must be set")?;

    if env::var("OVLD_DEBUG").is_ok() {
        logging::init(&format!("relay-{}", role_str));
    }

    let role: Role = role_str.parse().map_err(|e: String| anyhow::anyhow!(e))?;

    let relay_dir = match env::var("OVLD_RELAY_DIR") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => {
            let home = env::var("HOME").context("HOME environment variable not set")?;
            PathBuf::from(home).join(".config").join("ovld").join("relay")
        }
    };

    let session_name = env::var("OVLD_SESSION")
        .context("OVLD_SESSION environment variable must be set")?;

    let plugin_path = env::var("OVLD_PLUGIN_PATH")
        .unwrap_or_else(|_| String::new());

    let knowledge_dir = match env::var("OVLD_KNOWLEDGE_DIR") {
        Ok(dir) => PathBuf::from(dir),
        Err(_) => {
            let home = env::var("HOME").context("HOME environment variable not set")?;
            PathBuf::from(home).join(".config").join("ovld").join("knowledge")
        }
    };

    let store = Arc::new(
        MessageStore::new(relay_dir)
            .with_knowledge_dir(knowledge_dir)
    );

    logging::info(&format!("MCP relay started: role={} session={}", role, session_name));

    let service = RelayService::new(role, store, session_name, plugin_path);

    let server = service
        .serve(rmcp::transport::stdio())
        .await
        .context("Failed to start MCP server")?;

    server.waiting().await?;

    Ok(())
}
