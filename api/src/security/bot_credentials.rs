use std::{future::Future, pin::Pin};

use actix_web::{error::ErrorUnauthorized, Error, FromRequest};
use lib::{models::bot::BotRole, security::jwt::extract_bot_from_token};
use serde::{Deserialize, Serialize};

use super::auth_guard::extract_bearer_jwt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BotCredentials {
    pub name: String,
    pub role: BotRole,
}

impl FromRequest for BotCredentials {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            match extract_bearer_jwt(req.head()) {
                Some(jwt) => match extract_bot_from_token(&jwt) {
                    Some(bot) => Ok(BotCredentials {
                        name: bot.name,
                        role: bot.role,
                    }),
                    None => Err(ErrorUnauthorized("Invalid credentials")),
                },
                None => Err(ErrorUnauthorized("Missing valid bearer token.")),
            }
        })
    }
}
