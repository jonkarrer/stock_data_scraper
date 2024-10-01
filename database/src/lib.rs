use anyhow::{Context, Result};
use sqlx::{migrate::MigrateDatabase, SqlitePool};

pub struct DB {
    pub uri: String,
    pub pool: SqlitePool,
}

impl DB {
    pub async fn connect(uri: &str) -> Result<Self> {
        let pool = SqlitePool::connect(uri).await?;
        Ok(Self {
            uri: uri.to_string(),
            pool,
        })
    }

    pub async fn new(uri: &str) -> Result<Self> {
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
}
