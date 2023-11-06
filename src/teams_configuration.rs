use crate::utils::set_if_empty;

pub const TEAMS: &str = "Teams";
pub const TEAMS_URL: &str = "URL";
pub const TEAMS_API_TOKEN: &str = "API Token";

pub struct TeamsConfiguration {
    pub url: String,
    pub api_token: String,
}

pub fn set_default_values_if_needed(teams_conf: &mut TeamsConfiguration) {
    set_if_empty(&mut teams_conf.url, "ws://localhost:8124");
}
