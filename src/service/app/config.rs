use dotenv::dotenv;
use std::env;

pub struct Configuration {
    mongodb_uri: String,
}

impl  Configuration {
    pub fn from_env() ->Self {
        let mongo_database_url = env::var("MongoDB_DATABASE_URL").unwrap_or_else(|_|"mongodb://localhost:27017".to_owned());

        Configuration {
            mongodb_uri: mongo_database_url,
        }
    }
}