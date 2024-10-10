use alpaca_api_client::{market_data::stocks::StockBar, TimeFrame, Trend};
use anyhow::Result;
use chrono::DateTime;

use crate::SqliteDb;

pub struct DailyStockBarModel {
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
    pub next_period_price: f32,
    pub next_period_trend: String,
    pub next_period_unix_timestamp: i64,
    pub next_period_event_datetime: String,
    pub previous_period_trend: String,
    pub hundred_day_sma: f32,
    pub hundred_day_ema: f32,
    pub fifty_day_sma: f32,
    pub fifty_day_ema: f32,
    pub twenty_day_sma: f32,
    pub twenty_day_ema: f32,
    pub nine_day_ema: f32,
    pub nine_day_sma: f32,
    pub hundred_day_high: f32,
    pub hundred_day_low: f32,
    pub fifty_day_high: f32,
    pub fifty_day_low: f32,
    pub ten_day_high: f32,
    pub ten_day_low: f32,
    pub fourteen_day_rsi: f32,
    pub top_bollinger_band: f32,
    pub middle_bollinger_band: f32,
    pub bottom_bollinger_band: f32,
    pub macd_signal: f32,
}

pub struct DailyStockBarModelEntry {
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
    pub next_period_price: f32,
    pub next_period_trend: String,
    pub next_period_unix_timestamp: i64,
    pub next_period_event_datetime: String,
    pub previous_period_trend: String,
    pub hundred_day_sma: f32,
    pub hundred_day_ema: f32,
    pub fifty_day_sma: f32,
    pub fifty_day_ema: f32,
    pub twenty_day_sma: f32,
    pub twenty_day_ema: f32,
    pub nine_day_ema: f32,
    pub nine_day_sma: f32,
    pub hundred_day_high: f32,
    pub hundred_day_low: f32,
    pub fifty_day_high: f32,
    pub fifty_day_low: f32,
    pub ten_day_high: f32,
    pub ten_day_low: f32,
    pub fourteen_day_rsi: f32,
    pub top_bollinger_band: f32,
    pub middle_bollinger_band: f32,
    pub bottom_bollinger_band: f32,
    pub macd_signal: f32,
}

impl DailyStockBarModelEntry {
    pub fn new(
        stock_bar: &StockBar,
        stock_symbol: &str,
        timeframe: TimeFrame,
        bar_trend: Trend,
        buy_or_sell: i32,
        next_period_price: f32,
        next_period_trend: Trend,
        next_period_event_datetime: &str,
        previous_period_trend: Trend,
        hundred_day_sma: f32,
        hundred_day_ema: f32,
        fifty_day_sma: f32,
        fifty_day_ema: f32,
        twenty_day_sma: f32,
        twenty_day_ema: f32,
        nine_day_ema: f32,
        nine_day_sma: f32,
        hundred_day_high: f32,
        hundred_day_low: f32,
        fifty_day_high: f32,
        fifty_day_low: f32,
        ten_day_high: f32,
        ten_day_low: f32,
        fourteen_day_rsi: f32,
        top_bollinger_band: f32,
        middle_bollinger_band: f32,
        bottom_bollinger_band: f32,
        macd_signal: f32,
    ) -> Result<Self> {
        let event_datetime = stock_bar.t.to_string();
        let open_price = stock_bar.o;
        let close_price = stock_bar.c;
        let high_price = stock_bar.h;
        let low_price = stock_bar.l;
        let volume = stock_bar.v;
        let volume_weighted_price = stock_bar.vw;

        let (event_datetime, event_unix_timestamp) = Self::format_timestamp(&event_datetime)?;
        let (next_period_event_datetime, next_period_unix_timestamp) =
            Self::format_timestamp(next_period_event_datetime)?;

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
            next_period_price,
            next_period_trend: next_period_trend.to_string(),
            next_period_unix_timestamp,
            next_period_event_datetime,
            previous_period_trend: previous_period_trend.to_string(),
            hundred_day_sma,
            hundred_day_ema,
            fifty_day_sma,
            fifty_day_ema,
            twenty_day_sma,
            twenty_day_ema,
            nine_day_ema,
            nine_day_sma,
            hundred_day_high,
            hundred_day_low,
            fifty_day_high,
            fifty_day_low,
            ten_day_high,
            ten_day_low,
            fourteen_day_rsi,
            top_bollinger_band,
            middle_bollinger_band,
            bottom_bollinger_band,
            macd_signal,
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

pub trait DailyStockBarRepository {
    async fn insert_daily_stock_bar(&self, model_entry: &DailyStockBarModelEntry) -> Result<()>;
    async fn insert_batch_of_daily_stock_bars(
        &self,
        model_entries: &[DailyStockBarModelEntry],
    ) -> Result<()>;
}

impl DailyStockBarRepository for SqliteDb {
    async fn insert_daily_stock_bar(&self, model_entry: &DailyStockBarModelEntry) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO daily_stock_bars (event_datetime, event_unix_timestamp, open_price, close_price, high_price, low_price, volume, volume_weighted_price, stock_symbol, timeframe, bar_trend, buy_or_sell, next_period_price, next_period_trend, next_period_unix_timestamp, next_period_event_datetime, previous_period_trend, hundred_day_sma, hundred_day_ema, fifty_day_sma, fifty_day_ema, twenty_day_sma, twenty_day_ema, nine_day_sma, nine_day_ema, hundred_day_high, hundred_day_low, fifty_day_high, fifty_day_low, ten_day_high, ten_day_low, fourteen_day_rsi, top_bollinger_band, middle_bollinger_band, bottom_bollinger_band, macd_signal)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
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
        .bind(&model_entry.next_period_price)
        .bind(&model_entry.next_period_trend)
        .bind(&model_entry.next_period_unix_timestamp)
        .bind(&model_entry.next_period_event_datetime)
        .bind(&model_entry.previous_period_trend)
        .bind(&model_entry.hundred_day_sma)
        .bind(&model_entry.hundred_day_ema)
        .bind(&model_entry.fifty_day_sma)
        .bind(&model_entry.fifty_day_ema)
        .bind(&model_entry.twenty_day_sma)
        .bind(&model_entry.twenty_day_ema)
        .bind(&model_entry.nine_day_sma)
        .bind(&model_entry.nine_day_ema)
        .bind(&model_entry.hundred_day_high)
        .bind(&model_entry.hundred_day_low)
        .bind(&model_entry.fifty_day_high)
        .bind(&model_entry.fifty_day_low)
        .bind(&model_entry.ten_day_high)
        .bind(&model_entry.ten_day_low)
        .bind(&model_entry.fourteen_day_rsi)
        .bind(&model_entry.top_bollinger_band)
        .bind(&model_entry.middle_bollinger_band)
        .bind(&model_entry.bottom_bollinger_band)
        .bind(&model_entry.macd_signal)
        .execute(&self.pool).await?;

        Ok(())
    }

    async fn insert_batch_of_daily_stock_bars(
        &self,
        model_entries: &[DailyStockBarModelEntry],
    ) -> Result<()> {
        let transaction = self.pool.begin().await?;
        for model in model_entries {
            self.insert_daily_stock_bar(model).await?;
        }
        transaction.commit().await?;
        Ok(())
    }
}
