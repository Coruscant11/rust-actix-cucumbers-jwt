use super::{database_manager, MongoRepo, RepoError, ValidFields};
use crate::{models::bot::*, security::jwt::generate_token};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::IndexOptions, Client, IndexModel};

pub struct BotRepo {}

impl BotRepo {
    async fn check_initials_tokens(client: &Client) -> Result<(), RepoError> {
        match Self::check_if_there_an_admin(client).await {
            Ok(_) => Self::check_if_there_a_bot(client).await,
            Err(e) => Err(e),
        }
    }

    async fn check_if_there_an_admin(client: &Client) -> Result<(), RepoError> {
        match client
            .database(database_manager::database_name().as_str())
            .collection::<BotToken>("botTokens")
            .find_one(doc! {"name": "Administrator"}, None)
            .await
        {
            Ok(Some(_)) => Ok(()),
            Ok(None) => {
                let bot = Bot {
                    name: "Administrator".to_string(),
                    role: BotRole::RoleAdmin,
                    exp: 10000000000,
                };
                let mut bot_token = BotToken {
                    name: bot.name.clone(),
                    role: bot.role.clone(),
                    token: generate_token(&bot),
                };
                match Self::create(client, &mut bot_token).await {
                    Ok(_) => {
                        println!(
                            "No administrator found. Created one : \n\tName : [{}]\n\tRole: [{}]\n\tToken : [{}]\n",
                            &bot_token.name, &bot_token.role, &bot_token.token
                        );
                        Ok(())
                    }
                    Err(e) => {
                        println!("Couldn't find the administrator token.");
                        Err(e)
                    }
                }
            }
            Err(_) => Err(RepoError::FindError),
        }
    }

    async fn check_if_there_a_bot(client: &Client) -> Result<(), RepoError> {
        match client
            .database(database_manager::database_name().as_str())
            .collection::<BotToken>("botTokens")
            .find_one(doc! {"name": "Tester"}, None)
            .await
        {
            Ok(Some(_)) => Ok(()),
            Ok(None) => {
                let bot = Bot {
                    name: "Tester".to_string(),
                    role: BotRole::RoleBot,
                    exp: 10000000000,
                };
                let mut bot_token = BotToken {
                    name: bot.name.clone(),
                    role: bot.role.clone(),
                    token: generate_token(&bot),
                };
                match Self::create(client, &mut bot_token).await {
                    Ok(_) => {
                        println!(
                            "No tester bot found. Created one : \n\tName : [{}]\n\tRole: [{}]\n\tToken : [{}]\n",
                            &bot_token.name, &bot_token.role, &bot_token.token
                        );
                        Ok(())
                    }
                    Err(e) => {
                        println!("Couldn't find the tester bot token.");
                        Err(e)
                    }
                }
            }
            Err(_) => Err(RepoError::FindError),
        }
    }
}

#[async_trait]
impl MongoRepo<BotToken, String> for BotRepo {
    async fn init(client: &Client) -> Result<(), RepoError> {
        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! {"name": 1, "token": 1})
            .options(options)
            .build();

        /* Apply the index option to the collection */
        match client
            .database(database_manager::database_name().as_str())
            .collection::<BotToken>("botTokens")
            .create_index(model, None)
            .await
        {
            Ok(_) => Self::check_initials_tokens(client).await,
            Err(_) => Err(RepoError::InitError),
        }
    }

    async fn create(client: &Client, new_element: &mut BotToken) -> Result<(), RepoError> {
        if !new_element.check_fields() {
            return Err(RepoError::BadFieldError);
        }

        match Self::exists(client, &new_element.token).await {
            Ok(true) => Err(RepoError::AlreadyExistsError),
            Ok(false) => {
                match client
                    .database(database_manager::database_name().as_str())
                    .collection::<BotToken>("botTokens")
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
        new_element: &mut BotToken,
    ) -> Result<(), RepoError> {
        if !new_element.check_fields() {
            return Err(RepoError::BadFieldError);
        }

        match Self::exists(client, &new_element.token).await {
            Ok(true) => {
                match client
                    .database(database_manager::database_name().as_str())
                    .collection::<BotToken>("botTokens")
                    .replace_one(doc! {"name": existing_element_id}, new_element, None)
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
                    .collection::<BotToken>("botTokens")
                    .delete_one(doc! {"name": existing_element_id}, None)
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
    ) -> Result<Option<BotToken>, RepoError> {
        match client
            .database(database_manager::database_name().as_str())
            .collection::<BotToken>("botTokens")
            .find_one(doc! {"name": existing_element_id}, None)
            .await
        {
            Ok(Some(token)) => Ok(Some(token)),
            Ok(None) => Ok(None),
            Err(_) => Err(RepoError::FindError),
        }
    }

    async fn get_all(client: &Client) -> Result<Vec<BotToken>, RepoError> {
        match client
            .database(database_manager::database_name().as_str())
            .collection::<BotToken>("botTokens")
            .find(None, None)
            .await
            .ok()
        {
            Some(mut cursor) => {
                let mut tokens = Vec::new();
                while let Some(token) = cursor.try_next().await.ok() {
                    match token {
                        Some(token) => tokens.push(token),
                        None => break,
                    }
                }
                Ok(tokens)
            }
            None => Err(RepoError::FindError),
        }
    }
}
