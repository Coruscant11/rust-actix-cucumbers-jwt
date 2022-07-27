use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use mongodb::Client;

use lib::models::player::Player;
use lib::repository::player_repository::PlayerRepo;
use lib::repository::MongoRepo;
use lib::repository::RepoError;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_players);
    cfg.service(get_player);
    cfg.service(create_player);
    cfg.service(update_player);
    cfg.service(delete_player);
}

#[get("/players")]
pub async fn get_players(client: web::Data<Client>) -> impl Responder {
    HttpResponse::Ok().json(PlayerRepo::get_all(&client).await.ok())
}

#[get("/players/{discord_id}")]
pub async fn get_player(
    client: web::Data<Client>,
    discord_id: web::Path<String>,
) -> impl Responder {
    match PlayerRepo::get(&client, &discord_id.into_inner()).await {
        Ok(player) => match player {
            Some(player) => HttpResponse::Ok().json(player),
            None => HttpResponse::NotFound().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().body("Error while finding the player."),
    }
}

#[post("/players/{discord_id}")]
pub async fn create_player(
    client: web::Data<Client>,
    path: web::Path<String>,
    player: web::Json<Player>,
) -> impl Responder {
    let player = player.into_inner();
    let discord_id = path.into_inner();

    if discord_id.eq(&player.discord_id) {
        if player.check_fields() {
            match PlayerRepo::create(&client, player).await {
                Ok(_) => HttpResponse::Created().body(format!("Player [{}] created.", &discord_id)),
                Err(e) => match e {
                    RepoError::AlreadyExistsError => HttpResponse::Conflict()
                        .body(format!("Player [{}] already exists.", &discord_id)),
                    RepoError::BadFieldError => HttpResponse::BadRequest()
                        .body(format!("Player [{}] has bad fields.", &discord_id)),
                    _ => HttpResponse::InternalServerError()
                        .body(format!("Error while creating player [{}].", &discord_id)),
                },
            }
        } else {
            HttpResponse::BadRequest().body("Please checks your fields.")
        }
    } else {
        HttpResponse::BadRequest()
            .body("The discord_id in the path must match the discord_id in the body.")
    }
}

#[put("/players/{discord_id}")]
pub async fn update_player(
    client: web::Data<Client>,
    path: web::Path<String>,
    player: web::Json<Player>,
) -> impl Responder {
    let player = player.into_inner();
    let discord_id = path.into_inner();

    if discord_id.eq(&player.discord_id) {
        if player.check_fields() {
            match PlayerRepo::update(&client, &discord_id, player).await {
                Ok(_) => HttpResponse::Ok().body(format!("Player [{}] updated.", &discord_id)),
                Err(e) => match e {
                    RepoError::DoNotExistsError => HttpResponse::NotFound()
                        .body(format!("Player [{}] not found.", &discord_id)),
                    RepoError::BadFieldError => HttpResponse::BadRequest()
                        .body(format!("Player [{}] has bad fields.", &discord_id)),
                    _ => HttpResponse::InternalServerError()
                        .body(format!("Error while updating player [{}].", &discord_id)),
                },
            }
        } else {
            HttpResponse::BadRequest().body("Please checks your fields.")
        }
    } else {
        HttpResponse::BadRequest()
            .body("The discord_id in the path must match the discord_id in the body.")
    }
}

#[delete("/players/{discord_id}")]
pub async fn delete_player(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let discord_id = path.into_inner();

    match PlayerRepo::delete(&client, &discord_id).await {
        Ok(_) => HttpResponse::Ok().body(format!("Player [{}] deleted.", &discord_id)),
        Err(e) => match e {
            RepoError::DoNotExistsError => {
                HttpResponse::NotFound().body(format!("Player [{}] not found.", &discord_id))
            }
            _ => HttpResponse::InternalServerError()
                .body(format!("Error while deleting player [{}].", &discord_id)),
        },
    }
}
