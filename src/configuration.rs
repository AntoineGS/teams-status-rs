use crate::ha_configuration::{
    set_default_values_if_needed as ha_set_default_values_if_needed, HaConfiguration, HaEntities,
    HaIcons, HA_CAMERA_FRIENDLY_NAME, HA_CAMERA_ID, HA_ENTITIES, HA_ICONS, HA_IN_A_CALL,
    HA_LONG_LIVE_TOKEN, HA_NOT_IN_A_CALL, HA_STATUS_FRIENDLY_NAME, HA_STATUS_ID, HA_URL,
    HOME_ASSISTANT,
};
use crate::teams_configuration::{
    set_default_values_if_needed as teams_set_default_values_if_needed, TeamsConfiguration, TEAMS,
    TEAMS_API_TOKEN, TEAMS_URL,
};
use ini::Ini;

pub struct Configuration {
    pub ha: HaConfiguration,
    pub teams: TeamsConfiguration,
}
// TODO: This looks like aberration...
pub fn load_configuration() -> Configuration {
    let mut conf = create_configuration();
    let i = Ini::load_from_file("conf.ini").unwrap();
    for (sec, prop) in i.iter() {
        for (k, v) in prop.iter() {
            if v.is_empty() {
                continue;
            }

            match sec {
                Some(HOME_ASSISTANT) => match k {
                    HA_LONG_LIVE_TOKEN => conf.ha.long_live_token = v.to_string(),
                    HA_URL => conf.ha.url = v.to_string(),
                    _ => { /* We just ignore incorrect configs */ }
                },
                Some(HA_ICONS) => match k {
                    HA_IN_A_CALL => conf.ha.icons.in_a_call = v.to_string(),
                    HA_NOT_IN_A_CALL => conf.ha.icons.not_in_a_call = v.to_string(),
                    _ => { /* We just ignore incorrect configs */ }
                },
                Some(HA_ENTITIES) => match k {
                    HA_STATUS_ID => conf.ha.entities.status_id = v.to_string(),
                    HA_STATUS_FRIENDLY_NAME => {
                        conf.ha.entities.status_friendly_name = v.to_string()
                    }
                    HA_CAMERA_ID => conf.ha.entities.camera_id = v.to_string(),
                    HA_CAMERA_FRIENDLY_NAME => {
                        conf.ha.entities.camera_friendly_name = v.to_string()
                    }
                    _ => { /* We just ignore incorrect configs */ }
                },
                Some(TEAMS) => match k {
                    TEAMS_URL => conf.teams.url = v.to_string(),
                    TEAMS_API_TOKEN => conf.teams.api_token = v.to_string(),
                    _ => { /* We just ignore incorrect configs */ }
                },
                _ => { /* We just ignore incorrect configs */ }
            }
        }
    }

    ha_set_default_values_if_needed(&mut conf.ha);
    teams_set_default_values_if_needed(&mut conf.teams);
    // We recreate the file each time in case we introduce new values or configs
    save_ha_configuration(&conf);
    conf
}

fn create_configuration() -> Configuration {
    let ha_icons = HaIcons {
        in_a_call: "".to_string(),
        not_in_a_call: "".to_string(),
    };

    let ha_entities = HaEntities {
        status_id: "".to_string(),
        status_friendly_name: "".to_string(),
        camera_id: "".to_string(),
        camera_friendly_name: "".to_string(),
    };

    let ha_configuration = HaConfiguration {
        long_live_token: "".to_string(),
        url: "".to_string(),
        icons: ha_icons,
        entities: ha_entities,
    };

    let teams_configuration = TeamsConfiguration {
        url: "".to_string(),
        api_token: "".to_string(),
    };

    Configuration {
        ha: ha_configuration,
        teams: teams_configuration,
    }
}
fn save_ha_configuration(conf: &Configuration) {
    let mut ini = Ini::new();
    ini.with_section(Some(TEAMS))
        .set(TEAMS_URL, &conf.teams.url)
        .set(TEAMS_API_TOKEN, &conf.teams.api_token);
    ini.with_section(Some(HOME_ASSISTANT))
        .set(HA_URL, &conf.ha.url)
        .set(HA_LONG_LIVE_TOKEN, &conf.ha.long_live_token);
    ini.with_section(Some(HA_ICONS))
        .set(HA_IN_A_CALL, &conf.ha.icons.in_a_call)
        .set(HA_NOT_IN_A_CALL, &conf.ha.icons.not_in_a_call);
    ini.with_section(Some(HA_ENTITIES))
        .set(HA_STATUS_ID, &conf.ha.entities.status_id)
        .set(
            HA_STATUS_FRIENDLY_NAME,
            &conf.ha.entities.status_friendly_name,
        )
        .set(HA_CAMERA_ID, &conf.ha.entities.camera_id)
        .set(
            HA_CAMERA_FRIENDLY_NAME,
            &conf.ha.entities.camera_friendly_name,
        );
    ini.write_to_file("conf.ini").unwrap();
}
