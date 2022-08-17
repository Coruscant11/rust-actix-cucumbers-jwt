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

#[cfg(test)]
mod tests {
    use crate::models::player::{is_string_numeric, Player};
    use crate::repository::ValidFields;

    #[test]
    fn test_is_string_numeric() {
        assert_eq!(is_string_numeric("123".to_string()), true);
        assert_eq!(is_string_numeric("123a".to_string()), false);
        assert_eq!(is_string_numeric("a123".to_string()), false);
        assert_eq!(is_string_numeric("".to_string()), true);
    }

    #[test]
    fn test_check_fields() {
        let mut player = Player {
            discord_id: "123".to_string(),
            name: "test".to_string(),
            na_id: "".to_string(),
            jp_id: "".to_string(),
        };
        assert_eq!(player.check_fields(), true);
        player.na_id = "123456789".to_string();
        assert_eq!(player.check_fields(), true);
        player.jp_id = "123456789".to_string();
        assert_eq!(player.check_fields(), true);
        player.discord_id = "something".to_string();
        assert_eq!(player.check_fields(), false);
        player.discord_id = "".to_string();
        assert_eq!(player.check_fields(), false);
        player.discord_id = "123".to_string();
        assert_eq!(player.check_fields(), true);
        player.name = "".to_string();
        assert_eq!(player.check_fields(), false);
        player.name = "test".to_string();
        assert_eq!(player.check_fields(), true);
        player.na_id = "12345678".to_string();
        assert_eq!(player.check_fields(), false);
        player.na_id = "123456789".to_string();
        assert_eq!(player.check_fields(), true);
        player.jp_id = "12345678".to_string();
        assert_eq!(player.check_fields(), false);
    }
}
