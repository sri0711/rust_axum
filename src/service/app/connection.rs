use mongodb::Database;
use mongodb::{Client, options::ClientOptions};
use std::error::Error;
use crate::service::app::config;

async fn get_mongo_client() -> Result<Client, Box<dyn Error>> {
    let environment = config::Environment::from_env();
    let mut client_options = ClientOptions::parse(environment.mongodb_uri).await?;
    client_options.app_name = Some("Rust_Backend".to_string());
    let client = Client::with_options(client_options)?;
    Ok(client)
}

pub async fn sample_database() -> Result<Database, Box<dyn Error>> {
    let environment = config::Environment::from_env();
    let connection = get_mongo_client().await?; // Propagate error if it occurs
    let sample_database = connection.database(environment.mongodb_database_name.as_str());
    Ok(sample_database) // Return the database
}