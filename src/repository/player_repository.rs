use crate::models::player::Player;
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::IndexOptions, Client, IndexModel};

use super::{database_manager, MongoRepo, RepoError};

pub struct PlayerRepo {}

#[async_trait]
impl MongoRepo<Player, String> for PlayerRepo {
    async fn init(client: &Client) -> Result<(), RepoError> {
        /* Building the index option */
        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! {"discord_id": 1})
            .options(options)
            .build();

        /* Apply the index option to the collection */
        match client
            .database(database_manager::database_name().as_str())
            .collection::<Player>("players")
            .create_index(model, None)
            .await
        {
            Ok(_) => {
                println!("PlayerRepo: Index created. Repository ready to use.");
                Ok(())
            }
            Err(_) => Err(RepoError::InitError),
        }
    }

    async fn create(client: &Client, new_element: Player) -> Result<(), RepoError> {
        match Self::exists(client, &new_element.discord_id).await {
            Ok(true) => Err(RepoError::AlreadyExistsError),
            Ok(false) => {
                match client
                    .database(database_manager::database_name().as_str())
                    .collection::<Player>("players")
                    .insert_one(new_element, None)
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(RepoError::CreateError),
                }
            }
            Err(e) => Err(e),
        }
    }

    async fn update(
        client: &Client,
        existing_element_id: &String,
        new_element: Player,
    ) -> Result<(), RepoError> {
        match Self::exists(client, &new_element.discord_id).await {
            Ok(true) => {
                match client
                    .database(database_manager::database_name().as_str())
                    .collection::<Player>("players")
                    .replace_one(doc! {"discord_id": existing_element_id}, new_element, None)
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(RepoError::UpdateError),
                }
            }
            Ok(false) => Err(RepoError::DoNotExistsError),
            Err(e) => Err(e),
        }
    }

    async fn delete(client: &Client, existing_element_id: &String) -> Result<(), RepoError> {
        match Self::exists(client, &existing_element_id).await {
            Ok(true) => {
                match client
                    .database(database_manager::database_name().as_str())
                    .collection::<Player>("players")
                    .delete_one(doc! {"discord_id": existing_element_id}, None)
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(RepoError::DeleteError),
                }
            }
            Ok(false) => Err(RepoError::DoNotExistsError),
            Err(e) => Err(e),
        }
    }

    async fn get(
        client: &Client,
        existing_element_id: &String,
    ) -> Result<Option<Player>, RepoError> {
        match client
            .database(database_manager::database_name().as_str())
            .collection::<Player>("players")
            .find_one(doc! {"discord_id": existing_element_id}, None)
            .await
        {
            Ok(Some(player)) => Ok(Some(player)),
            Ok(None) => Ok(None),
            Err(_) => Err(RepoError::FindError),
        }
    }

    async fn get_all(client: &Client) -> Result<Vec<Player>, RepoError> {
        match client
            .database(database_manager::database_name().as_str())
            .collection::<Player>("players")
            .find(None, None)
            .await
            .ok()
        {
            Some(mut cursor) => {
                let mut players = Vec::new();
                while let Some(player) = cursor.try_next().await.ok() {
                    match player {
                        Some(player) => players.push(player),
                        None => break,
                    }
                }
                Ok(players)
            }
            None => Err(RepoError::FindError),
        }
    }
}
