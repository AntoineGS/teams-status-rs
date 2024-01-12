use crate::home_assistant::configuration::{
    create_ha_configuration, HaConfiguration, HaEntity, HA_BACKGROUND_BLURRED, HA_FRIENDLY_NAME,
    HA_HAND_RAISED, HA_ICON_OFF, HA_ICON_ON, HA_ID, HA_IN_A_MEETING, HA_LONG_LIVE_TOKEN, HA_MUTED,
    HA_RECORDING, HA_SHARING, HA_UNREAD_MESSAGES, HA_URL, HA_VIDEO_ON, HOME_ASSISTANT,
};
use crate::mqtt::configuration::{
    create_mqtt_configuration, MqttConfiguration, MQTT, MQTT_BACKGROUND_BLURRED, MQTT_ENTITIES,
    MQTT_HAND_RAISED, MQTT_MEETING, MQTT_MUTED, MQTT_PASSWORD, MQTT_PORT, MQTT_PORT_DEFAULT,
    MQTT_RECORDING, MQTT_SHARING, MQTT_TOPIC, MQTT_UNREAD_MESSAGES, MQTT_URL, MQTT_USERNAME,
    MQTT_VIDEO,
};
use crate::teams::configuration::{
    create_teams_configuration, TeamsConfiguration, TEAMS, TEAMS_API_TOKEN, TEAMS_URL,
};
use crate::utils::{decrypt_if_needed, encrypt};
use ini::Ini;
use log::{error, info};
use std::fs;

const GENERAL: &str = "General";
const GEN_CONF_VERSION: &str = "Configuration Version";
const GEN_CONF_VERSION_CURRENT: u32 = 1;
// Anything below this will result in copying the configuration as a backup as there are breaking changes
const GEN_CONF_VERSION_CUTOFF: u32 = 1;
const INI_FILE_NAME: &str = "conf.ini";

pub struct Configuration {
    pub ha: HaConfiguration,
    pub teams: TeamsConfiguration,
    pub mqtt: MqttConfiguration,
    pub version: u32,
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

fn load_entity(ha_entity: &mut HaEntity, config_name: &str, config_value: String) {
    match config_name {
        HA_ID => ha_entity.id = config_value,
        HA_FRIENDLY_NAME => ha_entity.friendly_name = config_value,
        HA_ICON_ON => ha_entity.icons.on = config_value,
        HA_ICON_OFF => ha_entity.icons.off = config_value,
        _ => { /* We just ignore incorrect configs */ }
    }
}

fn load_configuration(conf: &mut Configuration) {
    let i = Ini::load_from_file(INI_FILE_NAME).unwrap_or_else(|err| {
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

            let v_string = v.to_string();

            match sec {
                Some(GENERAL) => match k {
                    GEN_CONF_VERSION => conf.version = v.parse::<u32>().unwrap_or(0),
                    &_ => {}
                },
                Some(HOME_ASSISTANT) => match k {
                    HA_LONG_LIVE_TOKEN => conf.ha.long_live_token = decrypt_if_needed(v),
                    HA_URL => conf.ha.url = v.to_string(),
                    _ => { /* We just ignore incorrect configs */ }
                },
                Some(HA_MUTED) => load_entity(&mut conf.ha.entities.is_muted, k, v_string),
                Some(HA_VIDEO_ON) => load_entity(&mut conf.ha.entities.is_video_on, k, v_string),
                Some(HA_HAND_RAISED) => {
                    load_entity(&mut conf.ha.entities.is_hand_raised, k, v_string)
                }
                Some(HA_IN_A_MEETING) => {
                    load_entity(&mut conf.ha.entities.is_in_meeting, k, v_string)
                }
                Some(HA_RECORDING) => {
                    load_entity(&mut conf.ha.entities.is_recording_on, k, v_string)
                }
                Some(HA_BACKGROUND_BLURRED) => {
                    load_entity(&mut conf.ha.entities.is_background_blurred, k, v_string)
                }
                Some(HA_SHARING) => load_entity(&mut conf.ha.entities.is_sharing, k, v_string),
                Some(HA_UNREAD_MESSAGES) => {
                    load_entity(&mut conf.ha.entities.has_unread_messages, k, v_string)
                }
                Some(TEAMS) => match k {
                    TEAMS_URL => conf.teams.url = v.to_string(),
                    TEAMS_API_TOKEN => conf.teams.api_token = decrypt_if_needed(v),
                    _ => { /* We just ignore incorrect configs */ }
                },
                Some(MQTT) => match k {
                    MQTT_URL => conf.mqtt.set_url(v.to_string()),
                    MQTT_PORT => conf.mqtt.port = v.parse().unwrap_or(MQTT_PORT_DEFAULT),
                    MQTT_TOPIC => conf.mqtt.topic = v.to_string(),
                    MQTT_USERNAME => conf.mqtt.username = v.to_string(),
                    MQTT_PASSWORD => conf.mqtt.password = decrypt_if_needed(v),
                    _ => { /* We just ignore incorrect configs */ }
                },
                Some(MQTT_ENTITIES) => match k {
                    MQTT_MUTED => conf.mqtt.mqtt_entities.muted = v.to_string(),
                    MQTT_VIDEO => conf.mqtt.mqtt_entities.video = v.to_string(),
                    MQTT_HAND_RAISED => conf.mqtt.mqtt_entities.hand_raised = v.to_string(),
                    MQTT_MEETING => conf.mqtt.mqtt_entities.meeting = v.to_string(),
                    MQTT_RECORDING => conf.mqtt.mqtt_entities.recording = v.to_string(),
                    MQTT_BACKGROUND_BLURRED => {
                        conf.mqtt.mqtt_entities.background_blurred = v.to_string()
                    }
                    MQTT_SHARING => conf.mqtt.mqtt_entities.sharing = v.to_string(),
                    MQTT_UNREAD_MESSAGES => conf.mqtt.mqtt_entities.unread_messages = v.to_string(),
                    _ => { /* We just ignore incorrect configs */ }
                },
                _ => { /* We just ignore incorrect configs */ }
            }
        }
    }

    if conf.version < GEN_CONF_VERSION_CUTOFF {
        fs::copy(
            INI_FILE_NAME,
            format!("{}_backup_v{}.ini", INI_FILE_NAME, conf.version),
        )
        .unwrap_or_else(|err| {
            error!(
                "Unable to back up original configuration: {}",
                err.to_string()
            );
            0
        });
    }
}

fn create_configuration() -> Configuration {
    Configuration {
        ha: create_ha_configuration(),
        teams: create_teams_configuration(),
        mqtt: create_mqtt_configuration(),
        version: 0,
    }
}

fn add_entity(ini: &mut Ini, section: &str, ha_entity: &HaEntity) {
    ini.with_section(Some(section))
        .set(HA_ID, &ha_entity.id)
        .set(HA_FRIENDLY_NAME, &ha_entity.friendly_name)
        .set(HA_ICON_ON, &ha_entity.icons.on)
        .set(HA_ICON_OFF, &ha_entity.icons.off);
}
fn save_ha_configuration(conf: &Configuration) {
    let mut ini = Ini::new();
    ini.with_section(Some(TEAMS))
        .set(TEAMS_URL, &conf.teams.url)
        .set(TEAMS_API_TOKEN, encrypt(&conf.teams.api_token));

    ini.with_section(Some(HOME_ASSISTANT))
        .set(HA_URL, &conf.ha.url)
        .set(HA_LONG_LIVE_TOKEN, encrypt(&conf.ha.long_live_token));

    let ha_entities = &conf.ha.entities;
    add_entity(&mut ini, HA_MUTED, &ha_entities.is_muted);
    add_entity(&mut ini, HA_VIDEO_ON, &ha_entities.is_video_on);
    add_entity(&mut ini, HA_HAND_RAISED, &ha_entities.is_hand_raised);
    add_entity(&mut ini, HA_IN_A_MEETING, &ha_entities.is_in_meeting);
    add_entity(&mut ini, HA_RECORDING, &ha_entities.is_recording_on);
    add_entity(
        &mut ini,
        HA_BACKGROUND_BLURRED,
        &ha_entities.is_background_blurred,
    );
    add_entity(&mut ini, HA_SHARING, &ha_entities.is_sharing);
    add_entity(
        &mut ini,
        HA_UNREAD_MESSAGES,
        &ha_entities.has_unread_messages,
    );

    let mqtt = &conf.mqtt;
    ini.with_section(Some(MQTT))
        .set(MQTT_URL, mqtt.url())
        .set(MQTT_PORT, &mqtt.port.to_string())
        .set(MQTT_TOPIC, &mqtt.topic)
        .set(MQTT_USERNAME, &mqtt.username)
        .set(MQTT_PASSWORD, encrypt(&mqtt.password));

    let mqtt_entities = &conf.mqtt.mqtt_entities;
    ini.with_section(Some(MQTT_ENTITIES))
        .set(MQTT_MUTED, &mqtt_entities.muted)
        .set(MQTT_VIDEO, &mqtt_entities.video)
        .set(MQTT_HAND_RAISED, &mqtt_entities.hand_raised)
        .set(MQTT_MEETING, &mqtt_entities.meeting)
        .set(MQTT_RECORDING, &mqtt_entities.recording)
        .set(MQTT_BACKGROUND_BLURRED, &mqtt_entities.background_blurred)
        .set(MQTT_SHARING, &mqtt_entities.sharing)
        .set(MQTT_UNREAD_MESSAGES, &mqtt_entities.unread_messages);
    ini.with_section(Some(GENERAL))
        .set(GEN_CONF_VERSION, GEN_CONF_VERSION_CURRENT.to_string());
    ini.write_to_file(INI_FILE_NAME).unwrap();
}
