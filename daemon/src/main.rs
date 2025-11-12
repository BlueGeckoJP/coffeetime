mod data_processing;
mod database;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "coffeetime-daemon")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long)]
    database_url: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    ExecStart,
    ExecStop,
    BeforeSleep,
    AfterSleep,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let share_directory = dirs::home_dir().unwrap().join(".local/share/coffeetime");
    let default_db_path = share_directory.join("coffeetime.db");
    std::fs::create_dir_all(share_directory)?;

    let database_url = match cli.database_url {
        Some(url) => url,
        None => format!("sqlite://{}?mode=rwc", default_db_path.to_string_lossy()),
    };

    match cli.command {
        Commands::ExecStart => data_processing::exec_start(&database_url).await?,
        Commands::ExecStop => data_processing::exec_stop(&database_url).await?,
        Commands::BeforeSleep => data_processing::before_sleep(&database_url).await?,
        Commands::AfterSleep => data_processing::after_sleep(&database_url).await?,
    }

    Ok(())
}
