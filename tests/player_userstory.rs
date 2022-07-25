use std::convert::Infallible;

use async_trait::async_trait;
use cucumber::*;

#[derive(WorldInit, Debug, Default)]
pub struct PlayerWorld {
    players: Vec<Player>,
    latest_status: u16,
    latest_body: String,
}

#[async_trait(?Send)]
impl World for PlayerWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

#[tokio::main]
async fn main() {}
