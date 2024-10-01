use anyhow::Result;

use clap::{Parser, Subcommand};
use database::DB;

#[derive(Subcommand)]
pub enum DatabaseCommands {
    CreateDatabase { uri: String },
}

#[derive(Parser)]
pub struct DatabaseArgs {
    #[command(subcommand)]
    pub subcommand: DatabaseCommands,
}

pub async fn run(args: &DatabaseArgs) -> Result<()> {
    match &args.subcommand {
        DatabaseCommands::CreateDatabase { uri } => {
            DB::new(uri).await?;
            Ok(())
        }
    }
}
