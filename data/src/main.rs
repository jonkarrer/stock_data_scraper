mod watchlist;

use alpaca_api_client::{market_data::stocks::HistoricalBarsQuery, TimeFrame};

fn main() {
    let bars = HistoricalBarsQuery::new(vec!["AAPL", "TSLA"], TimeFrame::OneWeek)
        .start("2016-01-01")
        .end("2016-01-31")
        .send()
        .unwrap();

    println!("{:?}", bars);
}
