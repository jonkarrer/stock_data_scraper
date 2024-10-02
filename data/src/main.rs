mod watchlist;

use alpaca_api_client::{market_data::stocks::HistoricalBarsQuery, TimeFrame, Trend};
use database::{SqliteDb, StockBarModelEntry, StockBarRepository};

#[tokio::main]
async fn main() {
    let bars_map = HistoricalBarsQuery::new(vec!["AAPL", "TSLA"], TimeFrame::OneWeek)
        .start("2016-01-01")
        .send()
        .unwrap();

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

            let entry = StockBarModelEntry::new(
                bar,
                &symbol,
                TimeFrame::OneWeek,
                "tech",
                bar_trend,
                buy_or_sell,
                next_frame_price,
                next_frame_trend,
                &next_frame_event_datetime,
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
