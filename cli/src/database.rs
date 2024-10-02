use anyhow::Result;

use clap::{Parser, Subcommand};
use database::SqliteDb;
// use database::DB;

#[derive(Subcommand)]
pub enum DatabaseCommands {
    CreateDatabase { uri: String },
    TestConnection { uri: String },
}

#[derive(Parser)]
pub struct DatabaseArgs {
    #[command(subcommand)]
    pub subcommand: DatabaseCommands,
}

pub async fn run(args: &DatabaseArgs) -> Result<()> {
    match &args.subcommand {
        DatabaseCommands::CreateDatabase { uri } => {
            SqliteDb::create_new(uri).await?;
            Ok(())
        }
        DatabaseCommands::TestConnection { uri } => {
            let db = SqliteDb::connect(uri).await?;
            db.test_connection().await;
            Ok(())
        }
    }
}
