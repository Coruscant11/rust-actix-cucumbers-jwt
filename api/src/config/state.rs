use lib::repository::database_manager::init_database;
use mongodb::Client;

#[derive(Clone, Debug)]
pub struct RepoState {
    pub client: Client,
}

impl RepoState {
    pub async fn new() -> tide::Result<Self> {
        Ok(Self {
            client: init_database().await,
        })
    }
}
