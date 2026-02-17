use std::io;
use std::time::Duration;

use anyhow::Result;
use chrono::Utc;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::ExecutableCommand;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Cell, Paragraph, Row, Table};

use crate::army::roles::ALL;
use crate::config;
use crate::i18n::{self, Lang};
use crate::relay::store::MessageStore;
use crate::relay::types::Status;

const POLL_INTERVAL: Duration = Duration::from_secs(2);

fn status_symbol(status: &Status) -> &'static str {
    match status {
        Status::Idle => "[-]",
        Status::Working => "[*]",
        Status::Blocked => "[!]",
        Status::Done => "[v]",
    }
}

fn format_elapsed(secs: i64) -> String {
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m", secs / 60)
    } else {
        format!("{}h", secs / 3600)
    }
}

pub fn execute() -> Result<()> {
    let relay_dir = config::relay_dir()?;
    let store = MessageStore::new(relay_dir);
    let lang = config::load_config().lang;

    enable_raw_mode()?;
    io::stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;

    let result = run_loop(&mut terminal, &store, lang);

    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    result
}

fn run_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, store: &MessageStore, lang: Lang) -> Result<()> {
    let mut log_scroll: u16 = 0;
    loop {
        let statuses = store.get_all_statuses().unwrap_or_default();
        let recent = store.recent_messages(usize::MAX).unwrap_or_default();
        let now = Utc::now();

        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Length((ALL.len() as u16) + 2),
                    Constraint::Length(3),
                    Constraint::Min(0),
                    Constraint::Length(1),
                ])
                .split(frame.area());

            // Title
            let title = Paragraph::new(" ARMY STATUS ")
                .style(Style::default().fg(Color::Yellow).bold())
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::BOTTOM));
            frame.render_widget(title, chunks[0]);

            // Status table
            let header = Row::new(vec![
                Cell::from("Role").style(Style::default().bold()),
                Cell::from("Status").style(Style::default().bold()),
                Cell::from("Task").style(Style::default().bold()),
                Cell::from("Ago").style(Style::default().bold()),
            ]);

            let rows: Vec<Row> = statuses
                .iter()
                .map(|s| {
                    let elapsed = (now - s.updated_at).num_seconds().max(0);
                    let status_style = match s.status {
                        Status::Working => Style::default().fg(Color::Green),
                        Status::Blocked => Style::default().fg(Color::Red),
                        Status::Done => Style::default().fg(Color::Cyan),
                        Status::Idle => Style::default().fg(Color::DarkGray),
                    };
                    let pending = if store.has_pending(s.role.as_str()) { " *" } else { "" };
                    let role_key = format!("role.{}", s.role);
                    let role_name = i18n::t(&role_key, lang);
                    Row::new(vec![
                        Cell::from(format!("{} {}{}", s.role.icon(), role_name, pending)),
                        Cell::from(format!("{} {}", status_symbol(&s.status), s.status))
                            .style(status_style),
                        Cell::from(s.task.clone().unwrap_or_default()),
                        Cell::from(format_elapsed(elapsed)),
                    ])
                })
                .collect();

            let widths = [
                Constraint::Length(16),
                Constraint::Length(14),
                Constraint::Fill(1),
                Constraint::Length(6),
            ];

            let table = Table::new(rows, widths)
                .header(header)
                .block(Block::default().borders(Borders::BOTTOM));
            frame.render_widget(table, chunks[1]);

            // Summary
            let working_count = statuses.iter().filter(|s| matches!(s.status, Status::Working)).count();
            let pending_roles: Vec<&str> = ALL
                .iter()
                .filter(|r| store.has_pending(r.as_str()))
                .map(|r| r.as_str())
                .collect();
            let pending_text = if pending_roles.is_empty() {
                String::new()
            } else {
                format!("  Pending: {}", pending_roles.join(" "))
            };
            let summary = Paragraph::new(format!(
                " Workers: {}/{}{}",
                working_count,
                ALL.len(),
                pending_text,
            ))
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::BOTTOM));
            frame.render_widget(summary, chunks[2]);

            // Recent messages
            if !recent.is_empty() {
                let msg_lines: Vec<Line> = recent
                    .iter()
                    .map(|m| {
                        let ts = m.timestamp.with_timezone(&chrono::Local).format("%H:%M:%S");
                        let from_name = i18n::t(&format!("role.{}", m.from), lang);
                        let to_name = i18n::t(&format!("role.{}", m.to), lang);
                        Line::from(vec![
                            Span::styled(
                                format!(" [{}] ", ts),
                                Style::default().fg(Color::DarkGray),
                            ),
                            Span::styled(
                                format!("{} {} -> {} {}: ", m.from.icon(), from_name, m.to.icon(), to_name),
                                Style::default().fg(Color::DarkGray),
                            ),
                            Span::raw(&m.subject),
                        ])
                    })
                    .collect();
                let max_scroll = (msg_lines.len() as u16).saturating_sub(chunks[3].height.saturating_sub(1));
                log_scroll = log_scroll.min(max_scroll);
                let msgs = Paragraph::new(msg_lines)
                    .scroll((log_scroll, 0))
                    .block(Block::default().title(" Log ").borders(Borders::TOP));
                frame.render_widget(msgs, chunks[3]);
            }

            // Footer
            let footer = Paragraph::new(" q: quit | j/k: scroll log | data refreshes every 2s")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);
            frame.render_widget(footer, chunks[4]);
        })?;

        if event::poll(POLL_INTERVAL)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => return Ok(()),
                        KeyCode::Char('j') | KeyCode::Down => log_scroll = log_scroll.saturating_add(1),
                        KeyCode::Char('k') | KeyCode::Up => log_scroll = log_scroll.saturating_sub(1),
                        _ => {}
                    }
                }
            }
        }
    }
}
