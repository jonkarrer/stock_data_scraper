# Migrations
add-migration name:
     sqlx migrate add {{name}}

run-migration:
    export DATABASE_URL=sqlite:///Volumes/karrer_ssd/datastores/sqlite/market_data/stocks.db && sqlx migrate run

revert-migration:
    export DATABASE_URL=sqlite:///Volumes/karrer_ssd/datastores/sqlite/market_data/stocks.db && sqlx migrate revert

reset-database:
    export DATABASE_URL=sqlite:///Volumes/karrer_ssd/datastores/sqlite/market_data/stocks.db && sqlx database reset

# Database
create-db uri:
    cargo run --bin cli -- database create-database {{uri}}

test-db uri:
    cargo run --bin cli -- database test-connection {{uri}}