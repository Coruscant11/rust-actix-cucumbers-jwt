use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::repository::ValidFields;

#[derive(Eq, PartialEq, Debug, Clone, Deserialize, Serialize)]
pub enum BotRole {
    RoleBot,
    RoleAdmin,
}

impl Display for BotRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BotRole::RoleBot => write!(f, "RoleBot"),
            BotRole::RoleAdmin => write!(f, "RoleAdmin"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Bot {
    pub name: String,
    pub role: BotRole,
    pub exp: usize,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct BotToken {
    pub name: String,
    pub role: BotRole,
    pub token: String,
}

impl Bot {
    pub fn new(name: String, role: BotRole, exp: usize) -> Self {
        Self { name, role, exp }
    }
}

impl ValidFields for BotToken {
    fn check_fields(&mut self) -> bool {
        self.name = self.name.trim().to_string();
        self.name.len() > 0 && self.token.len() > 0
    }
}

impl Default for BotRole {
    fn default() -> Self {
        Self::RoleBot
    }
}
