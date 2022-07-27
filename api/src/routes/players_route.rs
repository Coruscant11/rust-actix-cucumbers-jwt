use lib::models::player::Player;
use lib::repository::player_repository::PlayerRepo;
use lib::repository::MongoRepo;
use lib::repository::RepoError;
use tide::convert::json;
use tide::Request;
use tide::Response;
use tide::Server;
use tide::StatusCode;

use crate::RepoState;

pub fn config(app: &mut Server<RepoState>) {
    app.at("/players").get(get_players);
    app.at("/players/:discord_id").get(get_player);
    app.at("/players/:discord_id").post(create_player);
    app.at("players/:discord_id").put(update_player);
    app.at("players/:discord_id").delete(delete_player);
}

pub async fn get_players(req: Request<RepoState>) -> tide::Result {
    match PlayerRepo::get_all(&req.state().client).await {
        Ok(players) => {
            let mut response = Response::new(StatusCode::Ok);
            response.set_body(json!(players));
            Ok(response)
        }
        Err(_) => Ok(Response::new(StatusCode::InternalServerError)),
    }
}

async fn get_player(req: Request<RepoState>) -> tide::Result {
    let discord_id = req.param("discord_id").unwrap().to_string();

    match PlayerRepo::get(&req.state().client, &discord_id).await {
        Ok(player) => match player {
            Some(player) => {
                let mut response = Response::new(StatusCode::Ok);
                response.set_body(json!(player));
                Ok(response)
            }
            None => Ok(Response::new(StatusCode::NotFound)),
        },
        Err(_) => Ok(Response::new(StatusCode::InternalServerError)),
    }
}

pub async fn create_player(mut req: Request<RepoState>) -> tide::Result {
    let discord_id = req.param("discord_id").unwrap().to_string();
    let player = req.body_json::<Player>().await.unwrap();

    if discord_id.eq(&player.discord_id) {
        if player.check_fields() {
            match PlayerRepo::create(&req.state().client, player).await {
                Ok(_) => {
                    let mut response = Response::new(StatusCode::Created);
                    response.set_body(format!("Player [{}] created.", &discord_id));
                    Ok(response)
                }
                Err(e) => match e {
                    RepoError::AlreadyExistsError => {
                        let mut response = Response::new(StatusCode::Conflict);
                        response.set_body(format!("Player [{}] already exists.", &discord_id));
                        Ok(response)
                    }
                    RepoError::BadFieldError => {
                        let mut response = Response::new(StatusCode::BadRequest);
                        response.set_body(format!("Player [{}] has bad fields.", &discord_id));
                        Ok(response)
                    }
                    _ => Ok(Response::new(StatusCode::InternalServerError)),
                },
            }
        } else {
            let mut response = Response::new(StatusCode::BadRequest);
            response.set_body(format!("Player [{}] has bad fields.", &discord_id));
            Ok(response)
        }
    } else {
        let mut response = Response::new(StatusCode::BadRequest);
        response.set_body("The discord_id in the path must match the discord_id in the body.");
        Ok(response)
    }
}

pub async fn update_player(mut req: Request<RepoState>) -> tide::Result {
    let discord_id = req.param("discord_id").unwrap().to_string();
    let player = req.body_json::<Player>().await.unwrap();

    if discord_id.eq(&player.discord_id) {
        if player.check_fields() {
            match PlayerRepo::update(&req.state().client, &discord_id, player).await {
                Ok(_) => {
                    let mut response = Response::new(StatusCode::Ok);
                    response.set_body(format!("Player [{}] updated.", &discord_id));
                    Ok(response)
                }
                Err(e) => match e {
                    RepoError::DoNotExistsError => Ok(Response::new(StatusCode::NotFound)),
                    RepoError::BadFieldError => Ok(Response::new(StatusCode::BadRequest)),
                    _ => Ok(Response::new(StatusCode::BadRequest)),
                },
            }
        } else {
            Ok(Response::new(StatusCode::BadRequest))
        }
    } else {
        let mut response = Response::new(StatusCode::BadRequest);
        response.set_body("The discord_id in the path must match the discord_id in the body.");
        Ok(response)
    }
}

pub async fn delete_player(req: Request<RepoState>) -> tide::Result {
    let discord_id = req.param("discord_id").unwrap().to_string();

    match PlayerRepo::delete(&req.state().client, &discord_id).await {
        Ok(_) => {
            let mut response = Response::new(StatusCode::Ok);
            response.set_body(format!("Player [{}] deleted.", &discord_id));
            Ok(response)
        }
        Err(e) => match e {
            RepoError::DoNotExistsError => Ok(Response::new(StatusCode::NotFound)),
            _ => Ok(Response::new(StatusCode::InternalServerError)),
        },
    }
}
