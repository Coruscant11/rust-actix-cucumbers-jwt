mod config;
mod routes;

use routes::players_route;

use crate::config::state::RepoState;

#[async_std::main]
async fn main() -> tide::Result<()> {
    println!("Starting server...");
    let mut app = tide::with_state(RepoState::new().await.unwrap());
    app.with(tide::log::LogMiddleware::new());
    players_route::config(&mut app);
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
