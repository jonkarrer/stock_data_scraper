use alpaca_api_client::{market_data::stocks::StockBar, TimeFrame, Trend};
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
    pub bar_trend: String,
    pub buy_or_sell: i32,
    pub next_frame_price: f32,
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
    pub bar_trend: String,
    pub buy_or_sell: i32,
    pub next_frame_price: f32,
    pub next_frame_trend: String,
    pub next_frame_unix_timestamp: i64,
    pub next_frame_event_datetime: String,
}

impl StockBarModelEntry {
    pub fn new(
        stock_bar: &StockBar,
        stock_symbol: &str,
        timeframe: TimeFrame,
        sector: &str,
        bar_trend: Trend,
        buy_or_sell: i32,
        next_frame_price: f32,
        next_frame_trend: Trend,
        next_frame_event_datetime: &str,
    ) -> Result<Self> {
        let event_datetime = stock_bar.t.to_string();
        let open_price = stock_bar.o;
        let close_price = stock_bar.c;
        let high_price = stock_bar.h;
        let low_price = stock_bar.l;
        let volume = stock_bar.v;
        let volume_weighted_price = stock_bar.vw;

        let (event_datetime, event_unix_timestamp) = Self::format_timestamp(&event_datetime)?;
        let (next_frame_event_datetime, next_frame_unix_timestamp) =
            Self::format_timestamp(next_frame_event_datetime)?;

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
            timeframe: timeframe.to_string(),
            sector: sector.to_string(),
            bar_trend: bar_trend.to_string(),
            buy_or_sell,
            next_frame_price,
            next_frame_trend: next_frame_trend.to_string(),
            next_frame_unix_timestamp,
            next_frame_event_datetime,
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
