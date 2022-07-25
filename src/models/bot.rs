use crate::models::api_role::ApiRole;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Bot {
    pub token: String,
    pub role: ApiRole,
}
