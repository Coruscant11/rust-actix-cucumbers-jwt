use super::jwt::validate_token;
use actix_web::guard::Guard;
use lib::models::bot::BotRole;

pub struct AuthGuardForUsers;
pub struct AuthGuardForAdmin;

impl Guard for AuthGuardForUsers {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        valide_header_bearer_token_role(ctx, BotRole::RoleBot)
    }
}

impl Guard for AuthGuardForAdmin {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        valide_header_bearer_token_role(ctx, BotRole::RoleAdmin)
    }
}

fn valide_header_bearer_token_role(
    ctx: &actix_web::guard::GuardContext<'_>,
    required_role: BotRole,
) -> bool {
    match ctx.head().headers.get("Authorization") {
        Some(token) => {
            let token = token.to_str();
            match token {
                Ok(token) => {
                    let token = token.split(" ").collect::<Vec<&str>>();
                    if token.len() == 2 {
                        if token[0] == "Bearer" {
                            let token_jwt = token[1];
                            return validate_token(token_jwt.to_string(), required_role);
                        }
                    }
                    return false;
                }
                Err(_) => return false,
            }
        }
        None => return false,
    }
}
