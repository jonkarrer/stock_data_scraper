-- Add migration script here
CREATE TABLE IF NOT EXISTS monthly_stock_bars (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_datetime TEXT NOT NULL,
    event_unix_timestamp INTEGER NOT NULL,
    open_price REAL NOT NULL DEFAULT 0.0,
    close_price REAL NOT NULL DEFAULT 0.0,
    high_price REAL NOT NULL DEFAULT 0.0,
    low_price REAL NOT NULL DEFAULT 0.0,
    volume REAL NOT NULL DEFAULT 0.0,
    volume_weighted_price REAL DEFAULT 0.0,
    stock_symbol TEXT NOT NULL,
    timeframe TEXT NOT NULL,
    bar_trend TEXT NOT NULL,
    buy_or_sell INTEGER NOT NULL,
    next_frame_price REAL NOT NULL,
    next_frame_trend TEXT NOT NULL,
    next_frame_unix_timestamp INTEGER NOT NULL,
    next_frame_event_datetime TEXT NOT NULL,
    ten_week_moving_avg REAL NOT NULL,
    ten_week_rsi REAL NOT NULL,
    ten_week_ema REAL NOT NULL
);