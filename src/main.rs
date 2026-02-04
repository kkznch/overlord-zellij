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
#[command(about = "Overlord CLI - 魔王軍をZellij上で指揮せよ")]
#[command(version)]
#[command(after_help = "魔王軍があなたの命令を待っています...")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// summon - 魔王軍を召喚（カレントディレクトリで展開）
    Summon,

    /// slay - 魔王軍を撃滅（セッション終了）
    Slay {
        /// 確認なしで強制撃滅
        #[arg(short, long)]
        force: bool,
    },

    /// status - 魔王軍の状態を確認
    Status,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Summon => summon::execute(),
        Commands::Slay { force } => slay::execute(force),
        Commands::Status => status::execute(),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
