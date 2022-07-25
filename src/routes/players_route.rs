use actix_web::{get, post, web, HttpResponse, Responder, Result};
use mongodb::Client;

use crate::models::player::Player;
use crate::repository::player_repository::PlayerRepo;
use crate::repository::MongoRepo;
use crate::repository::RepoError;

#[get("/players")]
pub async fn get_players(client: web::Data<Client>) -> Result<impl Responder> {
    Ok(web::Json(PlayerRepo::get_all(&client).await.ok()))
}

#[get("/players/{discord_id}")]
pub async fn get_player(
    client: web::Data<Client>,
    discord_id: web::Path<String>,
) -> Result<impl Responder> {
    Ok(web::Json(PlayerRepo::get(&client, &discord_id).await.ok()))
}

#[post("/players")]
pub async fn create_player(client: web::Data<Client>, player: web::Json<Player>) -> impl Responder {
    let player_id = player.discord_id.clone();

    match PlayerRepo::create(&client, player.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body(format!("Player [{}] created.", &player_id)),
        Err(e) => match e {
            RepoError::AlreadyExistsError => {
                HttpResponse::BadRequest().body(format!("Player [{}] already exists.", &player_id))
            }
            _ => HttpResponse::InternalServerError()
                .body(format!("Error while creating player [{}].", &player_id)),
        },
    }
}
