mod routes;
mod security;

use actix_web::{web, App, HttpServer};

use lib::{models::bot, repository::database_manager::init_database};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let token = security::jwt::generate_token(bot::Bot::new(
        "Nouveau bot de JEANNE".to_string(),
        bot::BotRole::RoleBot,
        10000000000,
    ));

    println!("Token généré : [{}]", token);

    const HOST: &str = "0.0.0.0";
    const PORT: u16 = 8080;

    println!("Initializing database...");
    let client = init_database().await;

    println!("Starting server [{}:{}].", HOST, PORT);
    HttpServer::new(move || {
        App::new()
            .configure(routes::players_route::config)
            .app_data(web::Data::new(client.clone()))
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
