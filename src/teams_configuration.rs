pub const TEAMS: &str = "Teams";
pub const TEAMS_URL: &str = "URL";

pub struct TeamsConfiguration {
    pub url: String,
}

pub fn create_teams_configuration() -> TeamsConfiguration {
    TeamsConfiguration {
        url: "ws://localhost:8124".to_string(),
    }
}
