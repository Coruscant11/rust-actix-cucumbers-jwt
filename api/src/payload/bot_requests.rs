use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct BotRegistration {
    pub name: String,
    pub exp: usize,
}
