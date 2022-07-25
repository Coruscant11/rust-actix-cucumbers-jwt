#[derive(Deserialize, Serialize)]
pub struct PlayerRequest {
    pub name: String,
    pub discord_id: String,
    pub na_id: String,
    pub jp_id: String,
}
