mod database;
mod entities;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "coffeetime-daemon")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long, default_value = "sqlite://test.db?mode=rwc")]
    database_url: String,
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

    match cli.command {
        Commands::ExecStart => database::exec_start(&cli.database_url).await?,
        Commands::ExecStop => database::exec_stop(&cli.database_url).await?,
        Commands::BeforeSleep => database::before_sleep(&cli.database_url).await?,
        Commands::AfterSleep => database::after_sleep(&cli.database_url).await?,
    }

    Ok(())
}
