use mongodb::{Client, options::ClientOptions};
use std::error::Error;

pub async fn get_mongo_client() -> Result<Client, Box<dyn Error>> {
    let client_uri = "your_mongodb_connection_string_here";
    let mut client_options = ClientOptions::parse(client_uri).await?;
    client_options.app_name = Some("My App".to_string());

    let client = Client::with_options(client_options)?;
    Ok(client)
}