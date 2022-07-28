use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Bot {
    pub name: String,
    pub role: BotRole,
    pub exp: usize,
}

impl Bot {
    pub fn new(name: String, role: BotRole, exp: usize) -> Self {
        Self { name, role, exp }
    }
}

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
pub enum BotRole {
    RoleBot,
    RoleAdmin,
}
