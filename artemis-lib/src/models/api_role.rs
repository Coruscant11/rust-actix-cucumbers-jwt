use serde::{Deserialize, Serialize};

#[derive(Eq, PartialEq, Debug, Deserialize, Serialize)]
pub enum ApiRole {
    RoleBot,
    RoleAdmin,
}
