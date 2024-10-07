#![allow(dead_code)]
mod watchlist;

use std::{thread, time::Duration};

use alpaca_api_client::{market_data::stocks::HistoricalBarsQuery, TimeFrame, Trend};
use database::{
    DailyStockBarModelEntry, DailyStockBarRepository, FifteenMinStockBarModelEntry,
    FifteenMinStockBarRepository, HourlyStockBarModelEntry, HourlyStockBarRepository,
    MonthlyStockBarModelEntry, MonthlyStockBarRepository, SqliteDb,
};
use tindi::BollingerBands;

#[tokio::main]
async fn main() {
    // let symbols = watchlist::get_all_unique_stock_symbols();
    // insert_monthly_stock_bars(symbols).await;
    // insert_daily_stock_bars(BATCH_SEVEN.to_vec()).await;
    // insert_fifteen_min_stock_bars(vec!["SO"]).await;
    // for chunk in BATCH_ALL.chunks(4) {
    //     insert_hourly_stock_bars(chunk.to_vec()).await;
    //     println!("Going to sleep for 1 minute...");
    //     thread::sleep(Duration::from_secs_f64(60.0)); // 1.2 minutes = 72 seconds
    //     println!("Woke up!");
    // }
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
        db.insert_batch_of_monthly_stock_bars(&stock_bar_entries)
            .await
            .unwrap();

        println!("Insertion completed for stock: {}", symbol);
    }
}

pub async fn insert_daily_stock_bars(symbols: Vec<&str>) {
    println!("Fetching historical stock data");
    let bars_map = HistoricalBarsQuery::new(symbols, TimeFrame::OneDay)
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

            let hundred_day_sma = if index < 100 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 100)..index].iter().map(|bar| bar.c).collect();
                tindi::simple_moving_average(&prices)
            };

            let hundred_day_ema = if index < 100 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 100)..index].iter().map(|bar| bar.c).collect();
                tindi::exponential_moving_average(&prices, 100).expect("EMA failed")
            };

            let fifty_day_sma = if index < 50 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 50)..index].iter().map(|bar| bar.c).collect();
                tindi::simple_moving_average(&prices)
            };

            let fifty_day_ema = if index < 50 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 50)..index].iter().map(|bar| bar.c).collect();
                tindi::exponential_moving_average(&prices, 50).expect("EMA failed")
            };

            let twenty_day_sma = if index < 20 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 20)..index].iter().map(|bar| bar.c).collect();
                tindi::simple_moving_average(&prices)
            };

            let twenty_day_ema = if index < 20 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 20)..index].iter().map(|bar| bar.c).collect();
                tindi::exponential_moving_average(&prices, 20).expect("EMA failed")
            };

            let nine_day_sma = if index < 9 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 9)..index].iter().map(|bar| bar.c).collect();
                tindi::simple_moving_average(&prices)
            };

            let nine_day_ema = if index < 9 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 9)..index].iter().map(|bar| bar.c).collect();
                tindi::exponential_moving_average(&prices, 9).expect("EMA failed")
            };

            let hundred_day_high = if index < 100 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 100)..index].iter().map(|bar| bar.h).collect();
                tindi::find_high(&prices)
            };

            let hundred_day_low = if index < 100 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 100)..index].iter().map(|bar| bar.l).collect();
                tindi::find_low(&prices)
            };

            let fifty_day_high = if index < 50 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 50)..index].iter().map(|bar| bar.h).collect();
                tindi::find_high(&prices)
            };

            let fifty_day_low = if index < 50 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 50)..index].iter().map(|bar| bar.l).collect();
                tindi::find_low(&prices)
            };

            let ten_day_high = if index < 10 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 10)..index].iter().map(|bar| bar.h).collect();
                tindi::find_high(&prices)
            };

            let ten_day_low = if index < 10 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 10)..index].iter().map(|bar| bar.l).collect();
                tindi::find_low(&prices)
            };

            let fourteen_day_rsi = if index < 14 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 14)..index].iter().map(|bar| bar.c).collect();
                tindi::relative_strength_index(&prices)
            };

            let bollinger_bands = if index < 20 {
                BollingerBands {
                    top_band: 0.0,
                    mid_band: 0.0,
                    bottom_band: 0.0,
                }
            } else {
                let prices: Vec<f32> = bars[(index - 20)..index].iter().map(|bar| bar.c).collect();
                tindi::BollingerBands::new(&prices, 20, 2.0).expect("Bollinger Bands failed")
            };

            let top_bollinger_band = bollinger_bands.top_band;
            let mid_bollinger_band = bollinger_bands.mid_band;
            let bottom_bollinger_band = bollinger_bands.bottom_band;

            let bar_trend = if bar.o > bar.c {
                Trend::Bearish
            } else {
                Trend::Bullish
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

            let entry = DailyStockBarModelEntry::new(
                bar,
                &symbol,
                TimeFrame::OneDay,
                bar_trend,
                buy_or_sell,
                next_frame_price,
                next_frame_trend,
                &next_frame_event_datetime,
                hundred_day_sma,
                hundred_day_ema,
                fifty_day_sma,
                fifty_day_ema,
                twenty_day_sma,
                twenty_day_ema,
                nine_day_sma,
                nine_day_ema,
                hundred_day_high,
                hundred_day_low,
                fifty_day_high,
                fifty_day_low,
                ten_day_high,
                ten_day_low,
                fourteen_day_rsi,
                top_bollinger_band,
                mid_bollinger_band,
                bottom_bollinger_band,
            )
            .unwrap();

            stock_bar_entries.push(entry);
        }
        db.insert_batch_of_daily_stock_bars(&stock_bar_entries)
            .await
            .unwrap();

        println!("Insertion completed for stock: {}", symbol);
    }
}

pub async fn insert_hourly_stock_bars(symbols: Vec<&str>) {
    println!("Fetching historical stock data");
    let bars_map = HistoricalBarsQuery::new(symbols, TimeFrame::OneHour)
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

            let five_period_sma = if index < 5 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 5)..index].iter().map(|bar| bar.c).collect();
                tindi::simple_moving_average(&prices)
            };

            let eight_period_sma = if index < 8 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 8)..index].iter().map(|bar| bar.c).collect();
                tindi::simple_moving_average(&prices)
            };

            let thirteen_period_sma = if index < 13 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 13)..index].iter().map(|bar| bar.c).collect();
                tindi::simple_moving_average(&prices)
            };

            let twenty_period_ema = if index < 20 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 20)..index].iter().map(|bar| bar.c).collect();
                tindi::exponential_moving_average(&prices, 20).expect("EMA failed")
            };

            let nine_period_rsi = if index < 9 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 9)..index].iter().map(|bar| bar.c).collect();
                tindi::relative_strength_index(&prices)
            };

            let twenty_period_high = if index < 20 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 20)..index].iter().map(|bar| bar.h).collect();
                tindi::find_high(&prices)
            };

            let twenty_period_low = if index < 20 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 20)..index].iter().map(|bar| bar.l).collect();
                tindi::find_low(&prices)
            };

            let eight_period_high = if index < 8 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 8)..index].iter().map(|bar| bar.h).collect();
                tindi::find_high(&prices)
            };

            let eight_period_low = if index < 8 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 8)..index].iter().map(|bar| bar.l).collect();
                tindi::find_low(&prices)
            };

            let five_period_high = if index < 5 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 5)..index].iter().map(|bar| bar.h).collect();
                tindi::find_high(&prices)
            };

            let five_period_low = if index < 5 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 5)..index].iter().map(|bar| bar.l).collect();
                tindi::find_low(&prices)
            };

            let bollinger_bands = if index < 13 {
                BollingerBands {
                    top_band: 0.0,
                    mid_band: 0.0,
                    bottom_band: 0.0,
                }
            } else {
                let prices: Vec<f32> = bars[(index - 13)..index].iter().map(|bar| bar.c).collect();
                tindi::BollingerBands::new(&prices, 13, 3.0).expect("Bollinger Bands failed")
            };

            let top_bollinger_band = bollinger_bands.top_band;
            let mid_bollinger_band = bollinger_bands.mid_band;
            let bottom_bollinger_band = bollinger_bands.bottom_band;

            let bar_trend = if bar.o > bar.c {
                Trend::Bearish
            } else {
                Trend::Bullish
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

            let entry = HourlyStockBarModelEntry::new(
                bar,
                &symbol,
                TimeFrame::OneHour,
                bar_trend,
                buy_or_sell,
                next_frame_price,
                next_frame_trend,
                &next_frame_event_datetime,
                five_period_sma,
                eight_period_sma,
                thirteen_period_sma,
                twenty_period_ema,
                nine_period_rsi,
                bottom_bollinger_band,
                mid_bollinger_band,
                top_bollinger_band,
                twenty_period_high,
                twenty_period_low,
                eight_period_high,
                eight_period_low,
                five_period_high,
                five_period_low,
            )
            .unwrap();

            stock_bar_entries.push(entry);
        }
        db.insert_batch_of_hourly_stock_bars(&stock_bar_entries)
            .await
            .unwrap();

        println!("Insertion completed for stock: {}", symbol);
    }
}

pub async fn insert_fifteen_min_stock_bars(symbols: Vec<&str>) {
    println!("Fetching historical stock data");
    let bars_map = HistoricalBarsQuery::new(symbols, TimeFrame::FifteenMinutes)
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

            let five_period_sma = if index < 5 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 5)..index].iter().map(|bar| bar.c).collect();
                tindi::simple_moving_average(&prices)
            };

            let eight_period_sma = if index < 8 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 8)..index].iter().map(|bar| bar.c).collect();
                tindi::simple_moving_average(&prices)
            };

            let thirteen_period_sma = if index < 13 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 13)..index].iter().map(|bar| bar.c).collect();
                tindi::simple_moving_average(&prices)
            };

            let twenty_period_ema = if index < 20 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 20)..index].iter().map(|bar| bar.c).collect();
                tindi::exponential_moving_average(&prices, 20).expect("EMA failed")
            };

            let nine_period_rsi = if index < 9 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 9)..index].iter().map(|bar| bar.c).collect();
                tindi::relative_strength_index(&prices)
            };

            let twenty_period_high = if index < 20 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 20)..index].iter().map(|bar| bar.h).collect();
                tindi::find_high(&prices)
            };

            let twenty_period_low = if index < 20 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 20)..index].iter().map(|bar| bar.l).collect();
                tindi::find_low(&prices)
            };

            let eight_period_high = if index < 8 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 8)..index].iter().map(|bar| bar.h).collect();
                tindi::find_high(&prices)
            };

            let eight_period_low = if index < 8 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 8)..index].iter().map(|bar| bar.l).collect();
                tindi::find_low(&prices)
            };

            let five_period_high = if index < 5 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 5)..index].iter().map(|bar| bar.h).collect();
                tindi::find_high(&prices)
            };

            let five_period_low = if index < 5 {
                0.0
            } else {
                let prices: Vec<f32> = bars[(index - 5)..index].iter().map(|bar| bar.l).collect();
                tindi::find_low(&prices)
            };

            let bollinger_bands = if index < 13 {
                BollingerBands {
                    top_band: 0.0,
                    mid_band: 0.0,
                    bottom_band: 0.0,
                }
            } else {
                let prices: Vec<f32> = bars[(index - 13)..index].iter().map(|bar| bar.c).collect();
                tindi::BollingerBands::new(&prices, 13, 3.0).expect("Bollinger Bands failed")
            };

            let top_bollinger_band = bollinger_bands.top_band;
            let mid_bollinger_band = bollinger_bands.mid_band;
            let bottom_bollinger_band = bollinger_bands.bottom_band;

            let bar_trend = if bar.o > bar.c {
                Trend::Bearish
            } else {
                Trend::Bullish
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

            let entry = FifteenMinStockBarModelEntry::new(
                bar,
                &symbol,
                TimeFrame::FifteenMinutes,
                bar_trend,
                buy_or_sell,
                next_frame_price,
                next_frame_trend,
                &next_frame_event_datetime,
                five_period_sma,
                eight_period_sma,
                thirteen_period_sma,
                twenty_period_ema,
                nine_period_rsi,
                bottom_bollinger_band,
                mid_bollinger_band,
                top_bollinger_band,
                twenty_period_high,
                twenty_period_low,
                eight_period_high,
                eight_period_low,
                five_period_high,
                five_period_low,
            )
            .unwrap();

            stock_bar_entries.push(entry);
        }
        db.insert_batch_of_fifteen_min_stock_bars(&stock_bar_entries)
            .await
            .unwrap();

        println!("Insertion completed for stock: {}", symbol);
    }
}
