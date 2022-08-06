use actix_web::delete;
use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use mongodb::Client;

use lib::models::bot::*;
use lib::repository::bot_repository::BotRepo;
use lib::repository::MongoRepo;
use lib::repository::RepoError;
use lib::security::jwt::*;

use crate::payload::bot_requests::*;
use crate::security::bot_credentials::BotCredentials;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_bots);
    cfg.service(get_bot);
    cfg.service(create_bot);
    cfg.service(delete_bot);
}

pub fn not_admin_response() -> HttpResponse {
    HttpResponse::Forbidden().body("This endpoint requires admin authority.")
}

#[get("/auth/bots")]
pub async fn get_bots(credentials: BotCredentials, client: web::Data<Client>) -> impl Responder {
    if credentials.role != BotRole::RoleAdmin {
        return not_admin_response();
    }
    match credentials.role {
        BotRole::RoleAdmin => HttpResponse::Ok().json(BotRepo::get_all(&client).await.ok()),
        BotRole::RoleBot => not_admin_response(),
    }
}

#[get("/auth/bots/{bot_name}")]
pub async fn get_bot(
    credentials: BotCredentials,
    client: web::Data<Client>,
    bot_name: web::Path<String>,
) -> impl Responder {
    if credentials.role != BotRole::RoleAdmin {
        return not_admin_response();
    }

    match BotRepo::get(&client, &bot_name.into_inner()).await {
        Ok(bot) => match bot {
            Some(bot) => HttpResponse::Ok().json(bot),
            None => HttpResponse::NotFound().finish(),
        },
        Err(_) => HttpResponse::InternalServerError().body("Error while finding the bot."),
    }
}

#[post("auth/bots")]
pub async fn create_bot(
    credentials: BotCredentials,
    client: web::Data<Client>,
    bot_registration: web::Json<BotRegistration>,
) -> impl Responder {
    if credentials.role != BotRole::RoleAdmin {
        return not_admin_response();
    }

    let bot = bot_registration.into_inner();
    let bot = Bot {
        name: bot.name,
        role: BotRole::RoleBot,
        exp: bot.exp,
    };
    let mut bot_token = BotToken {
        name: bot.name.clone(),
        role: bot.role.clone(),
        token: generate_token(&bot),
    };
    match BotRepo::create(&client, &mut bot_token).await {
        Ok(_) => HttpResponse::Created()
            .body(format!("Bot [{}] with [{}] created.", &bot.name, &bot.role)),
        Err(e) => match e {
            RepoError::AlreadyExistsError => {
                HttpResponse::Conflict().body(format!("Bot [{}] already exists.", &bot.name))
            }
            RepoError::BadFieldError => {
                HttpResponse::BadRequest().body(format!("Bot [{}] has bad fields.", &bot.name))
            }
            _ => HttpResponse::InternalServerError()
                .body(format!("Error while creating bot [{}].", &bot.name)),
        },
    }
}

#[delete("auth/bots/{bot_id}")]
pub async fn delete_bot(
    credentials: BotCredentials,
    client: web::Data<Client>,
    path: web::Path<String>,
) -> impl Responder {
    if credentials.role != BotRole::RoleAdmin {
        return not_admin_response();
    }

    let bot_id = path.into_inner();

    match BotRepo::delete(&client, &bot_id).await {
        Ok(_) => HttpResponse::Ok().body(format!("Bot [{}] deleted.", &bot_id)),
        Err(e) => match e {
            RepoError::DoNotExistsError => {
                HttpResponse::NotFound().body(format!("Bot [{}] not found.", &bot_id))
            }
            _ => HttpResponse::InternalServerError()
                .body(format!("Error while deleting bot [{}].", &bot_id)),
        },
    }
}
