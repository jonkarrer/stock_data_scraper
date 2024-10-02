#![allow(async_fn_in_trait)]
use anyhow::Result;
use sqlx::{migrate::MigrateDatabase, SqlitePool};

mod market_data;
pub use market_data::*;

pub struct SqliteDb {
    pub uri: String,
    pub pool: SqlitePool,
}

impl SqliteDb {
    pub async fn connect(uri: &str) -> Result<Self> {
        println!("Connecting to {}...", uri);
        let pool = SqlitePool::connect(uri).await?;
        println!("Connected!");
        Ok(Self {
            uri: uri.to_string(),
            pool,
        })
    }

    pub async fn create_new(uri: &str) -> Result<Self> {
        let does_exist = sqlx::Sqlite::database_exists(uri).await.unwrap_or(false);

        if does_exist {
            println!("Database already exists, connecting...");
            let connection = Self::connect(uri).await?;
            return Ok(connection);
        }

        println!("Creating database at {}", uri);
        sqlx::Sqlite::create_database(uri).await?;

        println!("Database created, connecting...");
        let connection = Self::connect(uri).await?;
        Ok(connection)
    }

    pub async fn test_connection(&self) {
        match sqlx::query("SELECT 1").execute(&self.pool).await {
            Ok(_) => println!("Connection successful!"),
            Err(e) => println!("Error: {}", e),
        }
    }

    pub async fn reset_database(uri: &str) -> Result<()> {
        println!("Resetting database at {}...", uri);
        sqlx::Sqlite::drop_database(uri).await?;
        println!("Database reset!");
        Self::create_new(uri).await?;
        Ok(())
    }
}
