use clap::{Parser, Subcommand};
use colored::Colorize;

mod army;
mod commands;
mod config;
mod i18n;
mod layout;
mod relay;
mod zellij;

use commands::{init, status, summon, unsummon};

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

    /// unsummon - 魔王軍を還送（セッション終了）
    Unsummon {
        /// 確認なしで強制還送
        #[arg(short, long)]
        force: bool,
    },

    /// status - 魔王軍の状態を確認
    Status,

    /// init - グローバル設定を（再）展開
    Init {
        /// 既存ファイルを強制上書き
        #[arg(short, long)]
        force: bool,
    },

    /// relay - MCP relay server (internal, spawned by Claude instances)
    #[command(hide = true)]
    Relay,
}

fn main() {
    let cli = Cli::parse();

    let config = config::load_config();

    let result = match cli.command {
        Commands::Summon => summon::execute(&config),
        Commands::Unsummon { force } => unsummon::execute(force, &config),
        Commands::Status => status::execute(&config),
        Commands::Init { force } => init::execute(force, &config),
        Commands::Relay => {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
            rt.block_on(relay::serve())
        }
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
