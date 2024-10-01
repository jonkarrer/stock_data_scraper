# Database
add_migration name:
     sqlx migrate add {{name}}

create_db uri:
    cargo run --bin cli -- database create-database {{uri}}