use async_trait::async_trait;
use mongodb::Client;

pub mod database_manager;
pub mod player_repository;

#[derive(Debug)]
pub enum RepoError {
    AlreadyExistsError,
    DoNotExistsError,
    InitError,
    CreateError,
    UpdateError,
    DeleteError,
    FindError,
}

#[async_trait]
pub trait MongoRepo<T, Tid: std::marker::Sync + 'static> {
    async fn init(client: &Client) -> Result<(), RepoError>;
    async fn create(client: &Client, new_element: T) -> Result<(), RepoError>;
    async fn update(
        client: &Client,
        existing_element_id: &Tid,
        new_element: T,
    ) -> Result<(), RepoError>;
    async fn delete(client: &Client, existing_element_id: &Tid) -> Result<(), RepoError>;
    async fn get(client: &Client, existing_element_id: &Tid) -> Result<Option<T>, RepoError>;
    async fn get_all(client: &Client) -> Result<Vec<T>, RepoError>;

    async fn exists(client: &Client, existing_element_id: &Tid) -> Result<bool, RepoError> {
        match Self::get(client, existing_element_id).await {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(e),
        }
    }
}
