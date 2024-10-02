mod watchlist;

use alpaca_api_client::{market_data::stocks::HistoricalBarsQuery, TimeFrame, Trend};
use database::{MonthlyStockBarModelEntry, MonthlyStockBarRepository, SqliteDb};

#[tokio::main]
async fn main() {
    let symbols = watchlist::get_all_unique_stock_symbols();
    insert_monthly_stock_bars(symbols).await;
}

pub async fn insert_monthly_stock_bars(symbols: Vec<&str>) {
    println!("Fetching historical stock data");
    let bars_map = HistoricalBarsQuery::new(symbols, TimeFrame::OneWeek)
        .start("2016-01-01")
        .send()
        .unwrap();

    println!("Finished fetching historical stock data");

    let db =
        SqliteDb::connect("sqlite:///Volumes/karrer_ssd/datastores/sqlite/market_data/stocks.db")
            .await
            .unwrap();

    println!("Inserting stock data");
    for (symbol, bars) in bars_map {
        let mut stock_bar_entries = Vec::new();
        for (index, bar) in bars.iter().enumerate() {
            if index + 1 == bars.len() {
                break;
            }

            let ten_week_sma = if index < 10 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 10)..index].iter().map(|bar| bar.c).collect();
                tindi::simple_moving_average(&prices)
            };

            let ten_week_ema = if index < 10 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 10)..index].iter().map(|bar| bar.c).collect();
                tindi::exponential_moving_average(&prices, 10).expect("EMA failed")
            };

            let ten_week_rsi = if index < 10 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 10)..index].iter().map(|bar| bar.c).collect();
                tindi::relative_strength_index(&prices)
            };

            let bar_trend = if bar.o > bar.c {
                Trend::Bearish
            } else {
                Trend::Bullish
            };

            let ten_week_high = if index < 10 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 10)..index].iter().map(|bar| bar.h).collect();
                tindi::find_high(&prices)
            };

            let ten_week_low = if index < 10 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 10)..index].iter().map(|bar| bar.l).collect();
                tindi::find_low(&prices)
            };

            let five_week_high = if index < 5 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 5)..index].iter().map(|bar| bar.h).collect();
                tindi::find_high(&prices)
            };

            let five_week_low = if index < 5 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 5)..index].iter().map(|bar| bar.l).collect();
                tindi::find_low(&prices)
            };

            let next_bar = &bars[index + 1];
            let price_diff = next_bar.c - bar.c;
            let buy_or_sell = if price_diff > 0.0 { 1 } else { 0 };
            let next_frame_price = next_bar.c;
            let next_frame_trend = if next_bar.o > next_bar.c {
                Trend::Bearish
            } else {
                Trend::Bullish
            };
            let next_frame_event_datetime = &next_bar.t;

            let entry = MonthlyStockBarModelEntry::new(
                bar,
                &symbol,
                TimeFrame::OneWeek,
                bar_trend,
                buy_or_sell,
                next_frame_price,
                next_frame_trend,
                &next_frame_event_datetime,
                ten_week_sma,
                ten_week_ema,
                ten_week_rsi,
                ten_week_high,
                ten_week_low,
                five_week_high,
                five_week_low,
            )
            .unwrap();

            stock_bar_entries.push(entry);
        }
        db.insert_batch_of_stock_bars(&stock_bar_entries)
            .await
            .unwrap();

        println!("Insertion completed for stock: {}", symbol);
    }
}
