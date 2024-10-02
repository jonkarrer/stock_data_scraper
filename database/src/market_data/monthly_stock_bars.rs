use alpaca_api_client::{market_data::stocks::StockBar, TimeFrame, Trend};
use anyhow::Result;
use chrono::DateTime;

use crate::SqliteDb;

pub struct MonthlyStockBarModel {
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
    pub bar_trend: String,
    pub buy_or_sell: i32,
    pub next_frame_price: f32,
    pub next_frame_trend: String,
    pub next_frame_unix_timestamp: i64,
    pub next_frame_event_datetime: String,
    pub ten_week_sma: f32,
    pub ten_week_ema: f32,
    pub ten_week_rsi: f32,
    pub ten_week_high: f32,
    pub ten_week_low: f32,
    pub five_week_high: f32,
    pub five_week_low: f32,
}

pub struct MonthlyStockBarModelEntry {
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
    pub bar_trend: String,
    pub buy_or_sell: i32,
    pub next_frame_price: f32,
    pub next_frame_trend: String,
    pub next_frame_unix_timestamp: i64,
    pub next_frame_event_datetime: String,
    pub ten_week_sma: f32,
    pub ten_week_ema: f32,
    pub ten_week_rsi: f32,
    pub ten_week_high: f32,
    pub ten_week_low: f32,
    pub five_week_high: f32,
    pub five_week_low: f32,
}

impl MonthlyStockBarModelEntry {
    pub fn new(
        stock_bar: &StockBar,
        stock_symbol: &str,
        timeframe: TimeFrame,
        bar_trend: Trend,
        buy_or_sell: i32,
        next_frame_price: f32,
        next_frame_trend: Trend,
        next_frame_event_datetime: &str,
        ten_week_sma: f32,
        ten_week_ema: f32,
        ten_week_rsi: f32,
        ten_week_high: f32,
        ten_week_low: f32,
        five_week_high: f32,
        five_week_low: f32,
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
            bar_trend: bar_trend.to_string(),
            buy_or_sell,
            next_frame_price,
            next_frame_trend: next_frame_trend.to_string(),
            next_frame_unix_timestamp,
            next_frame_event_datetime,
            ten_week_sma,
            ten_week_ema,
            ten_week_rsi,
            ten_week_high,
            ten_week_low,
            five_week_high,
            five_week_low,
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

pub trait MonthlyStockBarRepository {
    async fn insert_stock_bar(&self, model_entry: &MonthlyStockBarModelEntry) -> Result<()>;
    async fn insert_batch_of_stock_bars(
        &self,
        model_entries: &[MonthlyStockBarModelEntry],
    ) -> Result<()>;
}

impl MonthlyStockBarRepository for SqliteDb {
    async fn insert_stock_bar(&self, model_entry: &MonthlyStockBarModelEntry) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO monthly_stock_bars (event_datetime, event_unix_timestamp, open_price, close_price, high_price, low_price, volume, volume_weighted_price, stock_symbol, timeframe, bar_trend, buy_or_sell, next_frame_price, next_frame_trend, next_frame_unix_timestamp, next_frame_event_datetime, ten_week_moving_avg, ten_week_ema, ten_week_rsi, ten_week_high, ten_week_low, five_week_high, five_week_low)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
        .bind(&model_entry.bar_trend)
        .bind(&model_entry.buy_or_sell)
        .bind(&model_entry.next_frame_price)
        .bind(&model_entry.next_frame_trend)
        .bind(&model_entry.next_frame_unix_timestamp)
        .bind(&model_entry.next_frame_event_datetime)
        .bind(&model_entry.ten_week_sma)
        .bind(&model_entry.ten_week_ema)
        .bind(&model_entry.ten_week_rsi)
        .bind(&model_entry.ten_week_high)
        .bind(&model_entry.ten_week_low)
        .bind(&model_entry.five_week_high)
        .bind(&model_entry.five_week_low)
        .execute(&self.pool).await?;

        Ok(())
    }

    async fn insert_batch_of_stock_bars(
        &self,
        model_entries: &[MonthlyStockBarModelEntry],
    ) -> Result<()> {
        let transaction = self.pool.begin().await?;
        for model in model_entries {
            self.insert_stock_bar(model).await?;
        }
        transaction.commit().await?;
        Ok(())
    }
}
