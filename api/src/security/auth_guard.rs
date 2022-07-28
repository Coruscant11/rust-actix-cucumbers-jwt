use actix_web::guard::Guard;

pub struct AuthGuardForUsers;

impl Guard for AuthGuardForUsers {
    fn check(&self, ctx: &actix_web::guard::GuardContext<'_>) -> bool {
        println!(
            "Token d'authentification reçu : [{}]",
            ctx.head()
                .headers()
                .get("authorization")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
                .split(" ")
                .collect::<Vec<&str>>()[1..]
                .join(" ")
                .to_string()
        );
        return true;
    }
}
