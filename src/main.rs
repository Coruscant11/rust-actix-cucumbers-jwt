mod models;
mod repository;
mod routes;

use actix_web::{web, App, HttpServer};

use repository::database_manager::init_database;

fn config_api(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::players_route::get_players);
    cfg.service(routes::players_route::get_player);
    cfg.service(routes::players_route::create_player);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const HOST: &str = "localhost";
    const PORT: u16 = 8080;

    println!("Initializing database...");
    let client = init_database().await;

    println!("Starting server [{}:{}].", HOST, PORT);
    HttpServer::new(move || {
        App::new()
            .configure(config_api)
            .app_data(web::Data::new(client.clone()))
    })
    .bind((HOST, PORT))?
    .run()
    .await
}
