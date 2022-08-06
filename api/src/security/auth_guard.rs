use actix_web::{dev::RequestHead, guard::Guard};
use lib::{models::bot::BotRole, security::jwt::validate_token};

pub struct AuthGuardForUsers;
pub struct AuthGuardForAdmin;

impl Guard for AuthGuardForUsers {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        valide_header_bearer_token_role(ctx.head(), BotRole::RoleBot)
    }
}

impl Guard for AuthGuardForAdmin {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        valide_header_bearer_token_role(ctx.head(), BotRole::RoleAdmin)
    }
}

pub fn extract_bearer_jwt(head: &RequestHead) -> Option<String> {
    match head.headers.get("Authorization") {
        Some(token) => {
            let token = token.to_str();
            match token {
                Ok(token) => {
                    let token = token.split(" ").collect::<Vec<&str>>();
                    if token.len() == 2 {
                        if token[0] == "Bearer" {
                            let token_jwt = token[1];
                            return Some(token_jwt.to_string());
                        }
                    }
                    return None;
                }
                Err(_) => return None,
            }
        }
        None => return None,
    }
}

pub fn valide_header_bearer_token_role(head: &RequestHead, required_role: BotRole) -> bool {
    match extract_bearer_jwt(head) {
        Some(jwt) => validate_token(&jwt, required_role),
        None => false,
    }
}
