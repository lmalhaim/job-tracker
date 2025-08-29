use job_status_tracker::db::setup_pool_from_env;
use tracing::error;

use crate::cli::MainMenu;

mod cli;
// todo:
// [x] Create Menu
// [x] add option to go back
// [x] While loop in menu

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    if let Err(e) = setup_pool_from_env().await {
        error!("Error connecting to database: {}", e);
        std::process::exit(1)
    }
    MainMenu::show_menu(); 
}
