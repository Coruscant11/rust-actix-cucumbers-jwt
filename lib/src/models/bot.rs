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
        self.name.retain(|c| !c.is_whitespace());
        self.name.len() > 0 && self.token.len() > 0
    }
}

impl Default for BotRole {
    fn default() -> Self {
        Self::RoleBot
    }
}

#[cfg(test)]
mod tests {
    use crate::models::bot::{BotRole, BotToken};
    use crate::repository::ValidFields;

    #[test]
    fn test_bot_role() {
        let role = BotRole::RoleBot;
        assert_eq!(role.to_string(), "RoleBot");
        let role = BotRole::RoleAdmin;
        assert_eq!(role.to_string(), "RoleAdmin");
    }

    #[test]
    fn test_bot_default() {
        let role = BotRole::default();
        assert_eq!(role.to_string(), "RoleBot");
    }

    #[test]
    fn test_bot_token_check_fields() {
        let mut bot_token = BotToken {
            name: "test".to_string(),
            role: BotRole::RoleBot,
            token: "test".to_string(),
        };
        assert_eq!(bot_token.check_fields(), true);
        bot_token.name = "".to_string();
        assert_eq!(bot_token.check_fields(), false);
        bot_token.name = "test".to_string();
        bot_token.token = "".to_string();
        assert_eq!(bot_token.check_fields(), false);
    }
}
