use jsonwebtoken::{
    decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};

use crate::models::bot::{Bot, BotRole};

pub fn generate_token(bot: &Bot) -> String {
    // let bot = Bot {
    //     name: "Nouveau bot de JEANNE".to_string(),
    //     role: BotRole::RoleAdmin,
    //     exp: 10000000000,
    // };

    let token = encode(
        &Header::new(Algorithm::RS256),
        &bot,
        &EncodingKey::from_rsa_pem(include_bytes!("../../../.keys/private.pem")).unwrap(),
    )
    .unwrap();

    return token;
}

pub fn extract_bot_from_token(token: &String) -> Option<Bot> {
    let decode_headers = decode_header(&token).unwrap();

    let decoding = decode::<Bot>(
        &token.as_str(),
        &DecodingKey::from_rsa_pem(include_bytes!("../../../.keys/public.pem")).unwrap(),
        &Validation::new(decode_headers.alg),
    );

    match decoding {
        Ok(token_data) => Some(token_data.claims),
        _ => None,
    }
}

pub fn validate_token(token: &String, required_role: BotRole) -> bool {
    match extract_bot_from_token(token) {
        Some(bot) => match bot.role {
            BotRole::RoleAdmin => required_role == bot.role,
            BotRole::RoleBot => required_role == bot.role || bot.role == BotRole::RoleAdmin,
        },
        None => false,
    }
}
