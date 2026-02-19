use clap::{Parser, Subcommand};
use colored::Colorize;

use ovld::commands::{dashboard, init, status, summon, unsummon};
use ovld::{config, logging, relay};

#[derive(Parser)]
#[command(name = "ovld")]
#[command(about = "Overlord CLI - 魔王軍をZellij上で指揮せよ")]
#[command(version)]
#[command(after_help = "魔王軍があなたの命令を待っています...")]
struct Cli {
    /// デバッグログを ~/.config/ovld/logs/ に出力
    #[arg(long, global = true)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// summon - 魔王軍を召喚（カレントディレクトリで展開）
    Summon {
        /// サンドボックスを無効化（プロジェクトディレクトリ外への書き込みを許可）
        #[arg(long)]
        no_sandbox: bool,
    },

    /// unsummon - 魔王軍を還送（セッション終了）
    Unsummon {
        /// 還送するセッション名（省略時はカレントディレクトリから検索）
        name: Option<String>,

        /// 確認なしで強制還送
        #[arg(short, long)]
        force: bool,

        /// 全セッションを一括還送
        #[arg(long)]
        all: bool,
    },

    /// status - 魔王軍の状態を確認
    Status {
        /// 全セッションの一覧を表示
        #[arg(long)]
        all: bool,
    },

    /// init - グローバル設定を（再）展開
    Init {
        /// 既存ファイルを強制上書き
        #[arg(short, long)]
        force: bool,
    },

    /// dashboard - リアルタイム魔王軍ステータスダッシュボード (TUI)
    Dashboard {
        /// セッション名（レイアウトから自動設定）
        #[arg(long)]
        session: Option<String>,

        /// relay ディレクトリパス（レイアウトから自動設定）
        #[arg(long)]
        relay_dir: Option<String>,
    },

    /// relay - MCP relay server (internal, spawned by Claude instances)
    #[command(hide = true)]
    Relay,
}

fn main() {
    let cli = Cli::parse();

    if cli.debug {
        logging::init("ovld");
    }

    let config = config::load_config();

    let result = match cli.command {
        Commands::Summon { no_sandbox } => summon::execute(&config, cli.debug, !no_sandbox),
        Commands::Unsummon { name, force, all } => unsummon::execute(name, all, force, &config),
        Commands::Status { all } => status::execute(all, &config),
        Commands::Init { force } => init::execute(force, &config),
        Commands::Dashboard { session, relay_dir } => dashboard::execute(session, relay_dir),
        Commands::Relay => match tokio::runtime::Runtime::new() {
            Ok(rt) => rt.block_on(relay::serve()),
            Err(e) => Err(anyhow::anyhow!("Failed to create tokio runtime: {}", e)),
        },
    };

    if let Err(e) = result {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
