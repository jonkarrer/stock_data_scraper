mod database;
use anyhow::Result;
use clap::{Parser, Subcommand};
use database::DatabaseArgs;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Database(DatabaseArgs),
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Database(args) => database::run(args).await?,
    }

    Ok(())
}
