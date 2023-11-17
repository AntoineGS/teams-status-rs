use crate::home_assistant::configuration::{
    create_ha_configuration, HaConfiguration, HA_ENTITIES, HA_ICONS, HA_IN_A_MEETING,
    HA_LONG_LIVE_TOKEN, HA_MEETING_FRIENDLY_NAME, HA_MEETING_ID, HA_NOT_IN_A_MEETING, HA_URL,
    HA_VIDEO_FRIENDLY_NAME, HA_VIDEO_ID, HA_VIDEO_OFF, HA_VIDEO_ON, HOME_ASSISTANT,
};
use crate::mqtt::configuration::{
    create_mqtt_configuration, MqttConfiguration, MQTT, MQTT_ENTITIES, MQTT_MEETING, MQTT_PASSWORD,
    MQTT_PORT, MQTT_PORT_DEFAULT, MQTT_TOPIC, MQTT_URL, MQTT_USERNAME, MQTT_VIDEO,
};
use crate::teams::configuration::{
    create_teams_configuration, TeamsConfiguration, TEAMS, TEAMS_API_TOKEN, TEAMS_URL,
};
use crate::utils::{decrypt_if_needed, encrypt};
use ini::Ini;
use log::info;

pub struct Configuration {
    pub ha: HaConfiguration,
    pub teams: TeamsConfiguration,
    pub mqtt: MqttConfiguration,
}

pub fn get_configuration(save_configuration: bool) -> Configuration {
    let mut conf = create_configuration();
    load_configuration(&mut conf);
    // We recreate the file in case we introduce new values or configs
    if save_configuration {
        save_ha_configuration(&conf);
    };
    conf
}

fn load_configuration(conf: &mut Configuration) {
    let i = Ini::load_from_file("conf.ini").unwrap_or_else(|err| {
        info!(
            "The file conf.ini could not be loaded, we will create a new one: {}",
            err.to_string()
        );
        return Ini::new();
    });

    for (sec, prop) in i.iter() {
        for (k, v) in prop.iter() {
            if v.is_empty() {
                continue;
            }

            match sec {
                Some(HOME_ASSISTANT) => match k {
                    HA_LONG_LIVE_TOKEN => conf.ha.long_live_token = decrypt_if_needed(v),
                    HA_URL => conf.ha.url = v.to_string(),
                    _ => { /* We just ignore incorrect configs */ }
                },
                Some(HA_ICONS) => match k {
                    HA_IN_A_MEETING => conf.ha.icons.in_a_meeting = v.to_string(),
                    HA_NOT_IN_A_MEETING => conf.ha.icons.not_in_a_meeting = v.to_string(),
                    HA_VIDEO_ON => conf.ha.icons.video_on = v.to_string(),
                    HA_VIDEO_OFF => conf.ha.icons.video_off = v.to_string(),
                    _ => { /* We just ignore incorrect configs */ }
                },
                Some(HA_ENTITIES) => match k {
                    HA_MEETING_ID => conf.ha.entities.meeting_id = v.to_string(),
                    HA_MEETING_FRIENDLY_NAME => {
                        conf.ha.entities.meeting_friendly_name = v.to_string()
                    }
                    HA_VIDEO_ID => conf.ha.entities.video_id = v.to_string(),
                    HA_VIDEO_FRIENDLY_NAME => conf.ha.entities.video_friendly_name = v.to_string(),
                    _ => { /* We just ignore incorrect configs */ }
                },
                Some(TEAMS) => match k {
                    TEAMS_URL => conf.teams.url = v.to_string(),
                    TEAMS_API_TOKEN => conf.teams.api_token = decrypt_if_needed(v),
                    _ => { /* We just ignore incorrect configs */ }
                },
                Some(MQTT) => match k {
                    MQTT_URL => conf.mqtt.url = v.to_string(),
                    MQTT_PORT => conf.mqtt.port = v.parse().unwrap_or(MQTT_PORT_DEFAULT),
                    MQTT_TOPIC => conf.mqtt.topic = v.to_string(),
                    MQTT_USERNAME => conf.mqtt.username = v.to_string(),
                    MQTT_PASSWORD => conf.mqtt.password = decrypt_if_needed(v),
                    _ => { /* We just ignore incorrect configs */ }
                },
                Some(MQTT_ENTITIES) => match k {
                    MQTT_MEETING => conf.mqtt.mqtt_entities.meeting = v.to_string(),
                    MQTT_VIDEO => conf.mqtt.mqtt_entities.video = v.to_string(),
                    _ => { /* We just ignore incorrect configs */ }
                },
                _ => { /* We just ignore incorrect configs */ }
            }
        }
    }
}

fn create_configuration() -> Configuration {
    Configuration {
        ha: create_ha_configuration(),
        teams: create_teams_configuration(),
        mqtt: create_mqtt_configuration(),
    }
}
fn save_ha_configuration(conf: &Configuration) {
    let mut ini = Ini::new();
    ini.with_section(Some(TEAMS))
        .set(TEAMS_URL, &conf.teams.url)
        .set(TEAMS_API_TOKEN, encrypt(&conf.teams.api_token));
    ini.with_section(Some(HOME_ASSISTANT))
        .set(HA_URL, &conf.ha.url)
        .set(HA_LONG_LIVE_TOKEN, encrypt(&conf.ha.long_live_token));
    ini.with_section(Some(HA_ICONS))
        .set(HA_IN_A_MEETING, &conf.ha.icons.in_a_meeting)
        .set(HA_NOT_IN_A_MEETING, &conf.ha.icons.not_in_a_meeting);
    ini.with_section(Some(HA_ENTITIES))
        .set(HA_MEETING_ID, &conf.ha.entities.meeting_id)
        .set(
            HA_MEETING_FRIENDLY_NAME,
            &conf.ha.entities.meeting_friendly_name,
        )
        .set(HA_VIDEO_ID, &conf.ha.entities.video_id)
        .set(
            HA_VIDEO_FRIENDLY_NAME,
            &conf.ha.entities.video_friendly_name,
        );
    ini.with_section(Some(MQTT))
        .set(MQTT_URL, &conf.mqtt.url)
        .set(MQTT_PORT, &conf.mqtt.port.to_string())
        .set(MQTT_TOPIC, &conf.mqtt.topic)
        .set(MQTT_USERNAME, &conf.mqtt.username)
        .set(MQTT_PASSWORD, encrypt(&conf.mqtt.password));
    ini.with_section(Some(MQTT_ENTITIES))
        .set(MQTT_MEETING, &conf.mqtt.mqtt_entities.meeting)
        .set(MQTT_VIDEO, &conf.mqtt.mqtt_entities.video);
    ini.write_to_file("conf.ini").unwrap();
}
