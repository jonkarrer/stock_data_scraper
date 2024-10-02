mod watchlist;

use alpaca_api_client::{market_data::stocks::HistoricalBarsQuery, TimeFrame};
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

    for (symbol, bars) in bars_map {
        let mut stock_bar_entries = Vec::new();
        for bar in bars {
            let entry =
                StockBarModelEntry::new(bar, &symbol, TimeFrame::OneWeek.to_string(), "tech")
                    .unwrap();

            stock_bar_entries.push(entry);
        }
        db.insert_batch_of_stock_bars(&stock_bar_entries)
            .await
            .unwrap();
    }
}
