use jsonwebtoken::{
    decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use lib::models::bot::{Bot, BotRole};

pub fn generate_token(bot: Bot) -> String {
    // let bot = Bot {
    //     name: "Nouveau bot de JEANNE".to_string(),
    //     role: BotRole::RoleAdmin,
    //     exp: 10000000000,
    // };

    let token = encode(
        &Header::new(Algorithm::RS256),
        &bot,
        &EncodingKey::from_rsa_pem(include_bytes!("../../.keys/private.pem")).unwrap(),
    )
    .unwrap();

    return token;
}

pub fn validate_token(token: String, required_role: BotRole) -> bool {
    let decode_headers = decode_header(&token).unwrap();

    let decoding = decode::<Bot>(
        &token.as_str(),
        &DecodingKey::from_rsa_pem(include_bytes!("../../.keys/public.pem")).unwrap(),
        &Validation::new(decode_headers.alg),
    );

    match decoding {
        Ok(token_data) => match required_role {
            BotRole::RoleAdmin => token_data.claims.role == BotRole::RoleAdmin,
            BotRole::RoleBot => {
                token_data.claims.role == BotRole::RoleBot
                    || token_data.claims.role == BotRole::RoleAdmin
            }
            _ => false,
        },
        Err(e) => {
            println!("Error while validating token: {}", e);
            false
        }
    }
}
