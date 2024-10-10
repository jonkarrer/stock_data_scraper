use alpaca_api_client::{market_data::stocks::StockBar, TimeFrame, Trend};
use anyhow::Result;
use chrono::DateTime;

use crate::SqliteDb;

pub struct HourlyStockBarModel {
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
    pub bar_trend: String,
    pub buy_or_sell: i32,
    pub next_frame_price: f32,
    pub next_frame_trend: String,
    pub next_frame_unix_timestamp: i64,
    pub next_frame_event_datetime: String,
    pub five_period_sma: f32,
    pub eight_period_sma: f32,
    pub thirteen_period_sma: f32,
    pub nine_period_rsi: f32,
    pub bottom_bollinger_band: f32,
    pub middle_bollinger_band: f32,
    pub top_bollinger_band: f32,
    pub twenty_period_high: f32,
    pub twenty_period_low: f32,
    pub eight_period_high: f32,
    pub eight_period_low: f32,
    pub five_period_high: f32,
    pub five_period_low: f32,
}

pub struct HourlyStockBarModelEntry {
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
    pub five_period_sma: f32,
    pub eight_period_sma: f32,
    pub thirteen_period_sma: f32,
    pub nine_period_rsi: f32,
    pub bottom_bollinger_band: f32,
    pub middle_bollinger_band: f32,
    pub top_bollinger_band: f32,
    pub twenty_period_high: f32,
    pub twenty_period_low: f32,
    pub eight_period_high: f32,
    pub eight_period_low: f32,
    pub five_period_high: f32,
    pub five_period_low: f32,
}

impl HourlyStockBarModelEntry {
    pub fn new(
        stock_bar: &StockBar,
        stock_symbol: &str,
        timeframe: TimeFrame,
        bar_trend: Trend,
        buy_or_sell: i32,
        next_frame_price: f32,
        next_frame_trend: Trend,
        next_frame_event_datetime: &str,
        five_period_sma: f32,
        eight_period_sma: f32,
        thirteen_period_sma: f32,
        nine_period_rsi: f32,
        bottom_bollinger_band: f32,
        middle_bollinger_band: f32,
        top_bollinger_band: f32,
        twenty_period_high: f32,
        twenty_period_low: f32,
        eight_period_high: f32,
        eight_period_low: f32,
        five_period_high: f32,
        five_period_low: f32,
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
            five_period_sma,
            eight_period_sma,
            thirteen_period_sma,
            nine_period_rsi,
            bottom_bollinger_band,
            middle_bollinger_band,
            top_bollinger_band,
            twenty_period_high,
            twenty_period_low,
            eight_period_high,
            eight_period_low,
            five_period_high,
            five_period_low,
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

pub trait HourlyStockBarRepository {
    async fn insert_hourly_stock_bar(&self, model_entry: &HourlyStockBarModelEntry) -> Result<()>;
    async fn insert_batch_of_hourly_stock_bars(
        &self,
        model_entries: &[HourlyStockBarModelEntry],
    ) -> Result<()>;
}

impl HourlyStockBarRepository for SqliteDb {
    async fn insert_hourly_stock_bar(&self, model_entry: &HourlyStockBarModelEntry) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO hourly_stock_bars (event_datetime, event_unix_timestamp, open_price, close_price, high_price, low_price, volume, volume_weighted_price, stock_symbol, timeframe, bar_trend, buy_or_sell, next_frame_price, next_frame_trend, next_frame_unix_timestamp, next_frame_event_datetime, five_period_sma, eight_period_sma, thirteen_period_sma nine_period_rsi, bottom_bollinger_band, middle_bollinger_band, top_bollinger_band, twenty_period_high, twenty_period_low, eight_period_high, eight_period_low, five_period_high, five_period_low)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
        .bind(&model_entry.five_period_sma)
        .bind(&model_entry.eight_period_sma)
        .bind(&model_entry.thirteen_period_sma)
        .bind(&model_entry.nine_period_rsi)
        .bind(&model_entry.bottom_bollinger_band)
        .bind(&model_entry.middle_bollinger_band)
        .bind(&model_entry.top_bollinger_band)
        .bind(&model_entry.twenty_period_high)
        .bind(&model_entry.twenty_period_low)
        .bind(&model_entry.eight_period_high)
        .bind(&model_entry.eight_period_low)
        .bind(&model_entry.five_period_high)
        .bind(&model_entry.five_period_low)
        .execute(&self.pool).await?;

        Ok(())
    }

    async fn insert_batch_of_hourly_stock_bars(
        &self,
        model_entries: &[HourlyStockBarModelEntry],
    ) -> Result<()> {
        let transaction = self.pool.begin().await?;
        for model in model_entries {
            self.insert_hourly_stock_bar(model).await?;
        }
        transaction.commit().await?;
        Ok(())
    }
}
