#[derive(Deserialize, Serialize)]
pub struct TokenResponse {
    pub message: String,
    pub token: String,
}
