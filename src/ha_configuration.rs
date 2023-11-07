pub const HOME_ASSISTANT: &str = "Home Assistant";
pub const HA_LONG_LIVE_TOKEN: &str = "Long Live Token";
pub const HA_URL: &str = "URL";
pub const HA_ICONS: &str = "Home Assistant Icons";
pub const HA_IN_A_MEETING: &str = "In a Meeting";
pub const HA_NOT_IN_A_MEETING: &str = "Not in a Meeting";
pub const HA_CAMERA_ON: &str = "Camera On";
pub const HA_CAMERA_OFF: &str = "Camera Off";
pub const HA_ENTITIES: &str = "Home Assistant Entities";
pub const HA_MEETING_ID: &str = "Meeting Id";
pub const HA_MEETING_FRIENDLY_NAME: &str = "Meeting Friendly Name";
pub const HA_CAMERA_ID: &str = "Camera Id";
pub const HA_CAMERA_FRIENDLY_NAME: &str = "Camera Friendly Name";

pub struct HaIcons {
    pub in_a_meeting: String,
    pub not_in_a_meeting: String,
    pub camera_on: String,
    pub camera_off: String,
}

pub struct HaEntities {
    pub meeting_id: String,
    pub meeting_friendly_name: String,
    pub camera_id: String,
    pub camera_friendly_name: String,
}

pub struct HaConfiguration {
    pub long_live_token: String,
    pub url: String,
    pub icons: HaIcons,
    pub entities: HaEntities,
}

pub fn create_ha_configuration() -> HaConfiguration {
    let ha_icons = HaIcons {
        in_a_meeting: "mdi:phone-in-talk".to_string(),
        not_in_a_meeting: "mdi:phone-off".to_string(),
        camera_on: "mdi:camera".to_string(),
        camera_off: "mdi:camera-off".to_string(),
    };

    let ha_entities = HaEntities {
        meeting_id: "binary_sensor.teams_meeting".to_string(),
        meeting_friendly_name: "Teams Meeting".to_string(),
        camera_id: "binary_sensor.teams_camera".to_string(),
        camera_friendly_name: "Teams Camera".to_string(),
    };

    HaConfiguration {
        long_live_token: "".to_string(),
        url: "".to_string(),
        icons: ha_icons,
        entities: ha_entities,
    }
}
