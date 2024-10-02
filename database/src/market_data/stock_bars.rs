use alpaca_api_client::market_data::stocks::StockBar;
use anyhow::Result;
use chrono::DateTime;

pub struct StockBarModel {
    pub id: i32,
    pub event_datetime: String,
    pub event_unix_timestamp: i64,
    pub open_price: f32,
    pub close_price: f32,
    pub high_price: f32,
    pub low_price: f32,
    pub volume: f32,
    pub volume_weighted_price: f32,
    pub stock_symbol: String,
    pub timeframe: String,
    pub sector: String,
}

pub struct StockBarModelEntry {
    pub event_datetime: String,
    pub event_unix_timestamp: i64,
    pub open_price: f32,
    pub close_price: f32,
    pub high_price: f32,
    pub low_price: f32,
    pub volume: f32,
    pub volume_weighted_price: f32,
    pub stock_symbol: String,
    pub timeframe: String,
    pub sector: String,
}

impl StockBarModelEntry {
    pub fn new(
        stock_bar: StockBar,
        stock_symbol: &str,
        timeframe: String,
        sector: &str,
    ) -> Result<Self> {
        let event_datetime = stock_bar.t;
        let open_price = stock_bar.o;
        let close_price = stock_bar.c;
        let high_price = stock_bar.h;
        let low_price = stock_bar.l;
        let volume = stock_bar.v;
        let volume_weighted_price = stock_bar.vw;

        let (event_datetime, event_unix_timestamp) = Self::format_timestamp(&event_datetime)?;

        Ok(Self {
            event_datetime,
            event_unix_timestamp,
            open_price,
            close_price,
            high_price,
            low_price,
            volume,
            volume_weighted_price,
            stock_symbol: stock_symbol.to_string(),
            timeframe,
            sector: sector.to_string(),
        })
    }

    fn format_timestamp(datetime: &str) -> Result<(String, i64)> {
        let dt = DateTime::parse_from_rfc3339(datetime)?;

        // UTC
        let dt_utc = dt.with_timezone(&chrono::Utc);

        // Format for SQLite
        let dt_sqlite = dt_utc.format("%Y-%m-%d %H:%M:%S").to_string();

        // Unix timestamp
        let unix_timestamp = dt_utc.timestamp_millis();

        Ok((dt_sqlite, unix_timestamp))
    }
}

pub trait StockBarRepository {
    async fn insert_stock_bar(&self, model_entry: &StockBarModelEntry) -> Result<()>;
    async fn insert_batch_of_stock_bars(&self, model_entries: &[StockBarModelEntry]) -> Result<()>;
}
