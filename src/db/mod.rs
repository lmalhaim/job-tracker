use sqlx::{Error, MySqlPool, mysql};
use tracing::info;
use std::{env, num::ParseIntError}; 
use dotenv::dotenv;

fn get_env_var(var_name: &str) -> Result<String, Error> {
     env::var(var_name).map_err(|e| sqlx::Error::Configuration(e.into()))
}

pub async fn init_pool(
    host: &str,
    port: u16,
    username: &str,
    password: &str,
    database_name: &str,
) -> Result<MySqlPool, Error> {

    info!("Initializing Database Connection");
    let opts = mysql::MySqlConnectOptions::new()
        .host(host)
        .port(port)
        .username(username)
        .password(password)
        .database(database_name);

    info!("Starting Connection to Database");
    let connection = mysql::MySqlPoolOptions::new()
        .max_connections(5)
        .connect_with(opts)
        .await;

    return connection;
}

pub async fn init_pool_from_env() -> Result<MySqlPool, sqlx::Error> {
    dotenv().map_err(|e| sqlx::Error::Configuration(e.into()))?; 
    info!("Fecthing DB variables from evinronment"); 
    let host = get_env_var("DB_HOST")?;
    let port: u16 = get_env_var("DB_PORT")?.parse().map_err(|e:ParseIntError| Error::Configuration(e.into()))?; 
    let username = get_env_var("DB_USERNAME")?;
    let password = get_env_var("DB_PASSWORD")?;
    let database_name = get_env_var("DB_NAME")?;

    init_pool(&host, port, &username, &password, &database_name).await
}