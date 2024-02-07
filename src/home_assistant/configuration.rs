pub const HOME_ASSISTANT: &str = "Home Assistant";
pub const HA_LONG_LIVE_TOKEN: &str = "Long Live Token";
pub const HA_URL: &str = "URL";
pub const HA_MUTED: &str = "Home Assistant Entity - Muted";
pub const HA_VIDEO_ON: &str = "Home Assistant Entity - Video On";
pub const HA_HAND_RAISED: &str = "Home Assistant Entity - Hand Raised";
pub const HA_IN_A_MEETING: &str = "Home Assistant Entity - In a Meeting";
pub const HA_RECORDING: &str = "Home Assistant Entity - Recording";
pub const HA_BACKGROUND_BLURRED: &str = "Home Assistant Entity - Background Blurred";
pub const HA_SHARING: &str = "Home Assistant Entity - Sharing";
pub const HA_UNREAD_MESSAGES: &str = "Home Assistant Entity - Unread Messages";
pub const HA_ID: &str = "ID";
pub const HA_FRIENDLY_NAME: &str = "Friendly Name";
pub const HA_ICON_ON: &str = "Icon On";
pub const HA_ICON_OFF: &str = "Icon Off";
pub struct HaIcons {
    pub on: String,
    pub off: String,
}
pub struct HaEntity {
    pub id: String,
    pub friendly_name: String,
    pub icons: HaIcons,
}
pub struct HaEntities {
    pub is_muted: HaEntity,
    pub is_video_on: HaEntity,
    pub is_hand_raised: HaEntity,
    pub is_in_meeting: HaEntity,
    pub is_recording_on: HaEntity,
    pub is_background_blurred: HaEntity,
    pub is_sharing: HaEntity,
    pub has_unread_messages: HaEntity,
}

pub struct HaConfiguration {
    pub long_live_token: String,
    pub url: String,
    pub entities: HaEntities,
}

pub fn create_ha_configuration() -> HaConfiguration {
    let ha_entities = HaEntities {
        is_muted: HaEntity {
            id: "binary_sensor.teams_muted".to_string(),
            friendly_name: "Teams Muted".to_string(),
            icons: HaIcons {
                on: "mdi:microphone".to_string(),
                off: "mdi:microphone-off".to_string(),
            },
        },

        is_video_on: HaEntity {
            id: "binary_sensor.teams_video".to_string(),
            friendly_name: "Teams Video".to_string(),
            icons: HaIcons {
                on: "mdi:webcam".to_string(),
                off: "mdi:webcam-off".to_string(),
            },
        },

        is_hand_raised: HaEntity {
            id: "binary_sensor.teams_hand_raised".to_string(),
            friendly_name: "Teams Hand Raised".to_string(),
            icons: HaIcons {
                on: "mdi:hand-back-left".to_string(),
                off: "mdi:hand-back-left-off".to_string(),
            },
        },

        is_in_meeting: HaEntity {
            id: "binary_sensor.teams_meeting".to_string(),
            friendly_name: "Teams Meeting".to_string(),
            icons: HaIcons {
                on: "mdi:phone-in-talk".to_string(),
                off: "mdi:phone-off".to_string(),
            },
        },

        is_recording_on: HaEntity {
            id: "binary_sensor.teams_recording".to_string(),
            friendly_name: "Teams Recording".to_string(),
            icons: HaIcons {
                on: "mdi:record-rec".to_string(),
                off: "mdi:power-off".to_string(),
            },
        },

        is_background_blurred: HaEntity {
            id: "binary_sensor.teams_background_blurred".to_string(),
            friendly_name: "Teams Background Blurred".to_string(),
            icons: HaIcons {
                on: "mdi:blur".to_string(),
                off: "mdi:blur-off".to_string(),
            },
        },

        is_sharing: HaEntity {
            id: "binary_sensor.teams_sharing".to_string(),
            friendly_name: "Teams Sharing".to_string(),
            icons: HaIcons {
                on: "mdi:projector-screen".to_string(),
                off: "mdi:projector-screen-off".to_string(),
            },
        },

        has_unread_messages: HaEntity {
            id: "binary_sensor.teams_unread_messages".to_string(),
            friendly_name: "Teams Unread Messages".to_string(),
            icons: HaIcons {
                on: "mdi:message-alert".to_string(),
                off: "mdi:message-off".to_string(),
            },
        },
    };

    HaConfiguration {
        long_live_token: "".to_string(),
        url: "".to_string(),
        entities: ha_entities,
    }
}
