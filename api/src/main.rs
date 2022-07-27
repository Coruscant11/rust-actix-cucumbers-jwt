mod routes;

use actix_web::{guard::Guard, web, App, HttpServer};

use lib::repository::database_manager::init_database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const HOST: &str = "0.0.0.0";
    const PORT: u16 = 8080;

    println!("Initializing database...");
    let client = init_database().await;

    println!("Starting server [{}:{}].", HOST, PORT);
    HttpServer::new(move || {
        App::new()
            .service(
                web::scope("")
                    .guard(AuthGuard)
                    .configure(routes::players_route::config),
            )
            .app_data(web::Data::new(client.clone()))
    })
    .bind((HOST, PORT))?
    .run()
    .await
}

pub struct AuthGuard;

impl Guard for AuthGuard {
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
