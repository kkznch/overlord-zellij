use clap::{Parser, Subcommand};
use colored::Colorize;

mod army;
mod commands;
mod config;
mod error;
mod layout;
mod zellij;

use commands::{slay, status, summon};

#[derive(Parser)]
#[command(name = "ovld")]
#[command(about = "Overlord CLI - Command your Claude army on Zellij")]
#[command(version)]
#[command(after_help = "The demons await your command...")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Summon the army - Start Zellij session with army layout
    Summon {
        /// Layout file to use (default: army)
        #[arg(short, long, default_value = "army")]
        layout: String,

        /// Session name
        #[arg(short, long, default_value = "overlord")]
        session: String,

        /// Skip ritual injection
        #[arg(long)]
        no_rituals: bool,
    },

    /// Slay the army - Close session and cleanup all processes
    Slay {
        /// Session name to kill
        #[arg(short, long, default_value = "overlord")]
        session: String,

        /// Force kill without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Check army status
    Status {
        /// Session name to check
        #[arg(short, long, default_value = "overlord")]
        session: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Summon {
            layout,
            session,
            no_rituals,
        } => summon::execute(&layout, &session, no_rituals),

        Commands::Slay { session, force } => slay::execute(&session, force),

        Commands::Status { session } => status::execute(&session),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
