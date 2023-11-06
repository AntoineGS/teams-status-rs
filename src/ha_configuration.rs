use crate::utils::set_if_empty;
pub const HOME_ASSISTANT: &str = "Home Assistant";
pub const HA_LONG_LIVE_TOKEN: &str = "Long Live Token";
pub const HA_URL: &str = "URL";
pub const HA_ICONS: &str = "Home Assistant Icons";
pub const HA_IN_A_CALL: &str = "In a Call";
pub const HA_NOT_IN_A_CALL: &str = "Not in a Call";
pub const HA_ENTITIES: &str = "Home Assistant Entities";
pub const HA_STATUS_ID: &str = "Status Id";
pub const HA_STATUS_FRIENDLY_NAME: &str = "Status Friendly Name";
pub const HA_CAMERA_ID: &str = "Camera Id";
pub const HA_CAMERA_FRIENDLY_NAME: &str = "Camera Friendly Name";

pub struct HaIcons {
    pub in_a_call: String,
    pub not_in_a_call: String,
}

pub struct HaEntities {
    pub status_id: String,
    pub status_friendly_name: String,
    pub camera_id: String,
    pub camera_friendly_name: String,
}

pub struct HaConfiguration {
    pub long_live_token: String,
    pub url: String,
    pub icons: HaIcons,
    pub entities: HaEntities,
}

pub fn set_default_values_if_needed(ha_conf: &mut HaConfiguration) {
    set_if_empty(&mut ha_conf.icons.in_a_call, "mdi:phone-in-talk");
    set_if_empty(&mut ha_conf.icons.not_in_a_call, "mdi:phone-off");
    set_if_empty(&mut ha_conf.entities.status_id, "sensor.teams_status");
    set_if_empty(
        &mut ha_conf.entities.status_friendly_name,
        "Microsoft Teams Status",
    );
    set_if_empty(&mut ha_conf.entities.camera_id, "sensor.teams_cam_status");
    set_if_empty(
        &mut ha_conf.entities.camera_friendly_name,
        "Microsoft Teams Camera Status",
    );
}
