use std::error::Error;

use tracing::info;

use talino_tap_api::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    info!("Starting TalinoTap API");

    run().await
}
