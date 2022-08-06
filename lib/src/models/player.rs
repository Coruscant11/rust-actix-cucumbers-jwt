use serde::{Deserialize, Serialize};

use crate::repository::ValidFields;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Player {
    pub discord_id: String,
    pub name: String,
    pub na_id: String,
    pub jp_id: String,
}

impl ValidFields for Player {
    fn check_fields(&mut self) -> bool {
        self.discord_id = self.discord_id.trim().to_string();
        let id_and_name = is_string_numeric(self.discord_id.clone())
            && self.discord_id.len() > 0
            && self.name.len() > 0;
        let na_length = if self.na_id.len() > 0 {
            self.na_id.len() == 9
        } else {
            true
        };
        let jp_length = if self.jp_id.len() > 0 {
            self.jp_id.len() == 9
        } else {
            true
        };

        id_and_name && na_length && jp_length
    }
}

fn is_string_numeric(str: String) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}
