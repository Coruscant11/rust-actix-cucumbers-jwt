use mongodb::{
    options::{ClientOptions, Credential},
    Client,
};

use crate::repository::player_repository::*;

use super::MongoRepo;

pub const MONGO_URI: &str = "MONGO_URI";
pub const MONGO_USER: &str = "MONGO_USER";
pub const MONGO_PASSWORD: &str = "MONGO_PASSWORD";
pub const MONGO_SOURCE: &str = "MONGO_SOURCE";
pub const MONGO_APP_NAME: &str = "MONGO_APP_NAME";
pub const MONGO_DATABASE: &str = "MONGO_DATABASE";

pub fn database_name() -> String {
    let mongo_database = std::env::var(MONGO_DATABASE)
        .expect(format!("{} environment variable is missing.", MONGO_DATABASE).as_str());
    return mongo_database;
}

pub async fn init_database() -> Client {
    /* Environment variables */
    let mongo_uri = std::env::var(MONGO_URI)
        .expect(format!("{} environment variable is missing.", MONGO_URI).as_str());

    let mongo_source = std::env::var(MONGO_SOURCE)
        .expect(format!("{} environment variable is missing.", MONGO_SOURCE).as_str());
    let mongo_username = std::env::var(MONGO_USER)
        .expect(format!("{} environment variable is missing.", MONGO_USER).as_str());
    let mongo_password = std::env::var(MONGO_PASSWORD)
        .expect(format!("{} environment variable is missing.", MONGO_PASSWORD).as_str());
    let mongo_app_name = std::env::var(MONGO_APP_NAME)
        .expect(format!("{} environment variable is missing.", MONGO_APP_NAME).as_str());

    /* Credentials */
    let mongo_creds = Credential::builder()
        .username(mongo_username)
        .password(mongo_password)
        .source(mongo_source)
        .build();

    /* Connection */
    let mut client_options = ClientOptions::parse(&mongo_uri).await.unwrap();
    client_options.app_name = Some(mongo_app_name);
    client_options.credential = Some(mongo_creds);
    let client = Client::with_options(client_options).expect("Failed to connect to MongoDB.");

    /* Setup collections */
    init_collections(&client).await;

    return client;
}

async fn init_collections(client: &Client) {
    PlayerRepo::init(client).await.unwrap();
}
