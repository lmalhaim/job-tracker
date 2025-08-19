use job_status_tracker::db::init_pool_from_env;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    match init_pool_from_env().await {
        Ok(_) => info!("Succesfully connected to database"),
        Err(e) => {
            error!("Error connecting to database: {}", e);
            std::process::exit(1)
        }
    }
}
