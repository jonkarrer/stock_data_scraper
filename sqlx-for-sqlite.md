# Using Sqlx for SQLite

Recently had a project that needed to utilize all the benefits of SQLite, and Rust was interacting with the database. I have experience with `sqlx` already, so naturally I chose this to get going. Sqlx is a fantastic library that just get's the job done and it supports async features.

## Installation

The crate can be found here: [Sqlx on crates.io](https://crates.io/crates/sqlx)

```bash
cargo add sqlx --features sqlite
```

Some other features I used as well:

```bash
cargo add sqlx -F runtime-tokio-rustls,macros,chrono
```

## Usage

- started with migrations
- wrote db connection and test struc
- wrote the models
- wrote repo traits
- how i use it
