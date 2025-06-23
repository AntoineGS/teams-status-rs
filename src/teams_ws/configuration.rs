use ini::Ini;
use log::info;

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

pub fn change_teams_configuration(section: &str, key: &str, value: &str) {
    let mut i = Ini::load_from_file("conf.ini").unwrap_or_else(|err| {
        info!(
            "The file conf.ini could not be loaded but should already exist, exiting application: {}",
            err.to_string()
        );
        panic!("{}", err.to_string());
    });

    i.with_section(Some(section)).set(key, value);
    i.write_to_file("conf.ini").unwrap();
}
