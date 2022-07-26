mod payload;
mod routes;

use actix_web::{web, App, HttpServer};

use artemis_lib::repository::database_manager::init_database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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