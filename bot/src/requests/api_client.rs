use lib::models::player::Player;
use lib::models::server::FGOServer;

const PLAYERS_ENDPOINT: &str = "/players";

#[derive(Debug)]
pub enum ApiError {
    AlreadyExistsError,
    DoNotExistsError,
    InitError,
    CreateError,
    UpdateError,
    DeleteError,
    FindError,
    BadFieldError,
    OtherError,
    ParseError,
    InternalServerError,
}

fn get_api_url_from_env() -> String {
    std::env::var("API_URL").expect("API_URL must be set")
}

fn get_api_token_from_env() -> String {
    std::env::var("API_TOKEN").expect("API_TOKEN must be set")
}

pub async fn send_register_player_id_request_to_api(
    discord_id: &String,
    fgo_server: FGOServer,
    fgo_id: &String,
    fgo_name: &String,
) -> Result<(), ApiError> {
    let api_url = get_api_url_from_env();
    let api_token = get_api_token_from_env();

    let player = Player {
        discord_id: discord_id.clone(),
        name: fgo_name.clone(),
        na_id: if fgo_server == FGOServer::NA {
            fgo_id.clone()
        } else {
            String::new()
        },
        jp_id: if fgo_server == FGOServer::JP {
            fgo_id.clone()
        } else {
            String::new()
        },
    };

    let client = reqwest::Client::new();
    let mut res = client
        .post(format!("{}{}", api_url, PLAYERS_ENDPOINT))
        .json(&player);

    if api_token.len() > 0 {
        res = res.bearer_auth(&api_token);
    }

    match res.send().await {
        Ok(res) => match res.status() {
            reqwest::StatusCode::CREATED => Ok(()),
            reqwest::StatusCode::CONFLICT => send_update_player_id_request_to_api(player).await,
            reqwest::StatusCode::NOT_FOUND => Err(ApiError::DoNotExistsError),
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => Err(ApiError::OtherError),
            reqwest::StatusCode::BAD_REQUEST => Err(ApiError::BadFieldError),
            _ => {
                println!(
                    "Unhandled status received for creating player : {}",
                    res.status()
                );
                return Err(ApiError::OtherError);
            }
        },
        Err(e) => {
            println!("Error while creating player : {}", e);
            return Err(ApiError::OtherError);
        }
    }
}

pub async fn send_get_player_id_request_to_api(fgo_id: &String) -> Result<Player, ApiError> {
    let api_url = get_api_url_from_env();
    let api_token = get_api_token_from_env();

    let client = reqwest::Client::new();
    let mut res = client.get(format!("{}{}/{}", api_url, PLAYERS_ENDPOINT, fgo_id));

    if api_token.len() > 0 {
        res = res.bearer_auth(&api_token);
    }

    match res.send().await {
        Ok(res) => match res.status() {
            reqwest::StatusCode::OK => match res.json::<Player>().await {
                Ok(player) => Ok(player),
                Err(e) => Err(ApiError::ParseError),
            },
            reqwest::StatusCode::NOT_FOUND => Err(ApiError::DoNotExistsError),
            _ => return Err(ApiError::OtherError),
        },
        Err(e) => {
            println!("Failed to get player ID: {}", e);
            Err(ApiError::OtherError)
        }
    }
}

pub async fn send_update_player_id_request_to_api(player: Player) -> Result<(), ApiError> {
    let api_url = get_api_url_from_env();
    let api_token = get_api_token_from_env();

    let client = reqwest::Client::new();
    let mut res = client
        .put(format!(
            "{}{}/{}",
            api_url, PLAYERS_ENDPOINT, &player.discord_id
        ))
        .json(&player);

    if api_token.len() > 0 {
        res = res.bearer_auth(&api_token);
    }

    match res.send().await {
        Ok(res) => match res.status() {
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::NOT_FOUND => Err(ApiError::DoNotExistsError),
            reqwest::StatusCode::INTERNAL_SERVER_ERROR => Err(ApiError::OtherError),
            reqwest::StatusCode::BAD_REQUEST => Err(ApiError::BadFieldError),
            _ => {
                println!(
                    "Unhandled status received for updating player : {}",
                    res.status()
                );
                Err(ApiError::OtherError)
            }
        },
        Err(e) => {
            println!("Failed to get player ID: {}", e);
            Err(ApiError::OtherError)
        }
    }
}
