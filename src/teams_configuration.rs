pub const TEAMS: &str = "Teams";
pub const TEAMS_URL: &str = "URL";
pub const TEAMS_API_TOKEN: &str = "API Token";

pub struct TeamsConfiguration {
    pub url: String,
    pub api_token: String,
}

pub fn create_teams_configuration() -> TeamsConfiguration {
    TeamsConfiguration {
        url: "ws://localhost:8124".to_string(),
        api_token: "".to_string(),
    }
}
