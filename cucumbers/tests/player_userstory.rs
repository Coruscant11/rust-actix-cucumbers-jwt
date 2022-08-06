use assert_str::assert_str_eq;
use async_trait::async_trait;
use cucumber::*;
use gherkin::Step;
use lib::models::bot::BotRole;
use lib::models::player::Player;
use lib::repository::player_repository::PlayerRepo;
use lib::repository::{MongoRepo, ValidFields};
use mongodb::Client;
use reqwest::Response;
use std::convert::Infallible;

const PLAYERS_ENDPOINT: &str = "http://localhost:8080/players";

#[derive(WorldInit, Debug)]
pub struct PlayerWorld {
    pub players: Vec<Player>,
    pub latest_responses: Vec<Response>,
    pub latest_bodys: Vec<String>,
    pub client: Client,
    pub token: String,
}

#[async_trait(?Send)]
impl World for PlayerWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Self::Error> {
        Ok(Self {
            players: vec![],
            latest_responses: vec![],
            latest_bodys: vec![],
            client: lib::repository::database_manager::init_database().await,
            token: String::new(),
        })
    }
}

#[given(expr = "I am authenticated as {string}")]
async fn i_am_authenticated_as(world: &mut PlayerWorld, role: String) {
    let env_token: String;
    match role.as_str() {
        "admin" => {
            env_token =
                std::env::var("API_ADMIN_TOKEN").expect("Expected API_ADMIN_TOKEN in environment.")
        }
        _ => {
            env_token =
                std::env::var("API_BOT_TOKEN").expect("Expected API_BOT_TOKEN in environment.")
        }
    }

    world.token = env_token;
}

#[given(expr = "a player")]
async fn a_player(world: &mut PlayerWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) {
            // NOTE: skip header
            let discord_id = &row[0];
            let name = &row[1];
            let na_id = &row[2];
            let jp_id = &row[3];
            let player: Player = Player {
                discord_id: discord_id.to_string(),
                name: name.to_string(),
                na_id: na_id.to_string(),
                jp_id: jp_id.to_string(),
            };

            world.players.push(player);
        }
    }
}

#[given(expr = "the discord_id is already registered")]
async fn the_discord_id_is_already_registered(world: &mut PlayerWorld) {
    for player in &world.players {
        let mut exist = PlayerRepo::exists(&world.client, &player.discord_id)
            .await
            .unwrap();

        if !exist {
            PlayerRepo::create(&world.client, &mut player.clone())
                .await
                .unwrap();
        }

        exist = PlayerRepo::exists(&world.client, &player.discord_id)
            .await
            .unwrap();

        assert_eq!(true, exist);
    }
}

#[given(expr = "the discord_id is not already registered")]
async fn the_discord_id_is_not_already_registered(world: &mut PlayerWorld) {
    assert_eq!(false, world.players.is_empty());

    for player in &world.players {
        let mut exist = PlayerRepo::exists(&world.client, &player.discord_id)
            .await
            .unwrap();

        if exist {
            PlayerRepo::delete(&world.client, &player.discord_id)
                .await
                .unwrap();
        }

        exist = PlayerRepo::exists(&world.client, &player.discord_id)
            .await
            .unwrap();

        assert_eq!(false, exist);
    }
}

#[given(expr = "the discord_id is invalid")]
async fn the_discord_id_is_invalid(world: &mut PlayerWorld) {
    for player in &world.players {
        assert_eq!(false, player.clone().check_fields());
    }
}

/* WHEN STEPS */
#[when(expr = "I register the player")]
async fn i_register_the_player(world: &mut PlayerWorld) {
    for player in &world.players {
        let client = reqwest::Client::new();
        let mut res = client.post(PLAYERS_ENDPOINT).json(&player);

        if world.token.len() > 0 {
            res = res.bearer_auth(&world.token);
        }

        world.latest_responses.push(res.send().await.unwrap());
    }
}

#[when(expr = "I get the player")]
async fn i_get_the_player(world: &mut PlayerWorld) {
    for player in &world.players {
        let client = reqwest::Client::new();
        let mut res = client.get(format!("{}/{}", PLAYERS_ENDPOINT, &player.discord_id));

        if world.token.len() > 0 {
            res = res.bearer_auth(&world.token);
        }

        world.latest_responses.push(res.send().await.unwrap());
    }
}

#[when(expr = "I get all players")]
async fn i_get_all_players(world: &mut PlayerWorld) {
    let client = reqwest::Client::new();
    let mut res = client.get(PLAYERS_ENDPOINT);

    if world.token.len() > 0 {
        res = res.bearer_auth(&world.token);
    }

    world.latest_responses.push(res.send().await.unwrap());
}

#[when(expr = "I update the player")]
async fn i_update_the_player(world: &mut PlayerWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) {
            // NOTE: skip header
            let discord_id = &row[0];
            let name = &row[1];
            let na_id = &row[2];
            let jp_id = &row[3];
            let player: Player = Player {
                discord_id: discord_id.to_string(),
                name: name.to_string(),
                na_id: na_id.to_string(),
                jp_id: jp_id.to_string(),
            };

            let client = reqwest::Client::new();
            let mut res = client
                .put(format!("{}/{}", PLAYERS_ENDPOINT, discord_id))
                .json(&player);

            if world.token.len() > 0 {
                res = res.bearer_auth(&world.token);
            }

            world.latest_responses.push(res.send().await.unwrap());
        }
    }
}

#[when(expr = "I delete the player")]
async fn i_delete_the_player(world: &mut PlayerWorld) {
    for player in &world.players {
        let client = reqwest::Client::new();
        let mut res = client.delete(format!("{}/{}", PLAYERS_ENDPOINT, &player.discord_id));

        if world.token.len() > 0 {
            res = res.bearer_auth(&world.token);
        }

        world.latest_responses.push(res.send().await.unwrap());
    }
}

/* THEN STEPS */
#[then(expr = "the player is registered")]
async fn the_player_is_registered(world: &mut PlayerWorld) {
    for player in &world.players {
        let exist = PlayerRepo::exists(&world.client, &player.discord_id)
            .await
            .unwrap();

        assert_eq!(true, exist);
    }
}

#[then(expr = "the player is not registered")]
async fn the_player_is_not_registered(world: &mut PlayerWorld) {
    for player in &world.players {
        let exist = PlayerRepo::exists(&world.client, &player.discord_id)
            .await
            .unwrap();

        assert_eq!(false, exist);
    }
}

#[then(expr = "I receive a code {int}")]
async fn i_receive_a_code(world: &mut PlayerWorld, code: i32) {
    for response in &world.latest_responses {
        assert_eq!(code, response.status().as_u16() as i32);
    }
}

#[then(expr = "the player is returned")]
async fn the_player_is_returned(world: &mut PlayerWorld) {
    for response in world.latest_responses.drain(..) {
        let player: Player = response.json::<Player>().await.unwrap();

        let is_returned = world
            .players
            .iter()
            .find(|&p| {
                p.discord_id.eq(&player.discord_id)
                    || p.name.eq(&player.name)
                    || p.na_id.eq(&player.na_id)
                    || p.jp_id.eq(&player.jp_id)
            })
            .is_some();

        assert_eq!(true, is_returned);
    }
}

#[then(expr = "the players are returned")]
async fn the_player_are_returned(world: &mut PlayerWorld) {
    for response in world.latest_responses.drain(..) {
        let players: Vec<Player> = response.json::<Vec<Player>>().await.unwrap();

        for player in &world.players {
            let is_returned = players
                .iter()
                .find(|&p| {
                    p.discord_id.eq(&player.discord_id)
                        || p.name.eq(&player.name)
                        || p.na_id.eq(&player.na_id)
                        || p.jp_id.eq(&player.jp_id)
                })
                .is_some();

            assert_eq!(true, is_returned);
        }
    }
}

#[then(expr = "the player is updated")]
async fn the_player_is_updated(world: &mut PlayerWorld, step: &Step) {
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter().skip(1) {
            let discord_id = &row[0];
            let name = &row[1];
            let na_id = &row[2];
            let jp_id = &row[3];

            let found = PlayerRepo::get(&world.client, discord_id).await.unwrap();
            match found {
                Some(player) => {
                    assert_str_eq!(&player.discord_id, discord_id);
                    assert_str_eq!(&player.name, name);
                    assert_str_eq!(&player.na_id, na_id);
                    assert_str_eq!(&player.jp_id, jp_id);
                }
                None => assert!(false),
            }
        }
    } else {
        assert!(false);
    }
}

#[then(expr = "the player is deleted")]
async fn the_player_is_deleted(world: &mut PlayerWorld) {
    for player in &world.players {
        let exist = PlayerRepo::exists(&world.client, &player.discord_id)
            .await
            .unwrap();

        assert_eq!(false, exist);
    }
}

#[tokio::main]
async fn main() {
    PlayerWorld::cucumber()
        .fail_on_skipped()
        .after(|_feature, _rule, _scenario, world| {
            Box::pin(async {
                let world = world.unwrap();

                let players = PlayerRepo::get_all(&world.client).await.unwrap();
                for player in &players {
                    PlayerRepo::delete(&world.client, &player.discord_id)
                        .await
                        .unwrap();
                    let exist = PlayerRepo::exists(&world.client, &player.discord_id)
                        .await
                        .unwrap();
                    assert_eq!(false, exist);
                }
            })
        })
        .run_and_exit("tests/features")
        .await;
}
