use std::{error::Error, time::Duration};

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub async fn connect() -> Result<Pool<Postgres>, Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .connect("postgresql://grpc-user:grpc@localhost:5432/grpc-demo-db")
        .await?;

    Ok(pool)
}
