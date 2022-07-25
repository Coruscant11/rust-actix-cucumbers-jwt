use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Player {
    pub discord_id: String,
    pub name: String,
    pub na_id: String,
    pub jp_id: String,
}
