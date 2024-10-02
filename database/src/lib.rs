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
}

impl StockBarRepository for SqliteDb {
    async fn insert_stock_bar(&self, model_entry: &StockBarModelEntry) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO stock_bars (event_datetime, event_unix_timestamp, open_price, close_price, high_price, low_price, volume, volume_weighted_price, stock_symbol, timeframe, sector)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(&model_entry.event_datetime)
        .bind(&model_entry.event_unix_timestamp)
        .bind(&model_entry.open_price)
        .bind(&model_entry.close_price)
        .bind(&model_entry.high_price)
        .bind(&model_entry.low_price)
        .bind(&model_entry.volume)
        .bind(&model_entry.volume_weighted_price)
        .bind(&model_entry.stock_symbol)
        .bind(&model_entry.timeframe)
        .bind(&model_entry.sector)
        .execute(&self.pool).await?;

        Ok(())
    }

    async fn insert_batch_of_stock_bars(&self, model_entries: &[StockBarModelEntry]) -> Result<()> {
        let transaction = self.pool.begin().await?;
        for model in model_entries {
            self.insert_stock_bar(model).await?;
        }
        transaction.commit().await?;
        Ok(())
    }
}
