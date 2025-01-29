use std::collections::HashMap;

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

#[derive(Clone)]
pub struct HaIcons {
    pub on: String,
    pub off: String,
}

#[derive(Clone)]
pub struct HaEntity {
    pub id: String,
    pub friendly_name: String,
    pub icons: HaIcons,
    pub additional_attributes: HashMap<String, serde_json::Value>,
}

#[derive(Clone)]
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

impl IntoIterator for HaEntities {
    type Item = (String, HaEntity);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut entities = Vec::new();
        entities.push((self.is_muted.id.to_string(), self.is_muted));
        entities.push((self.is_video_on.id.to_string(), self.is_video_on));
        entities.push((self.is_hand_raised.id.to_string(), self.is_hand_raised));
        entities.push((self.is_in_meeting.id.to_string(), self.is_in_meeting));
        entities.push((self.is_recording_on.id.to_string(), self.is_recording_on));
        entities.push((
            self.is_background_blurred.id.to_string(),
            self.is_background_blurred,
        ));
        entities.push((self.is_sharing.id.to_string(), self.is_sharing));
        entities.push((
            self.has_unread_messages.id.to_string(),
            self.has_unread_messages,
        ));
        entities.into_iter()
    }
}

impl HaEntities {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (String, &mut HaEntity)> {
        let mut entities = Vec::new();
        entities.push((self.is_muted.id.clone(), &mut self.is_muted));
        entities.push((self.is_video_on.id.clone(), &mut self.is_video_on));
        entities.push((self.is_hand_raised.id.clone(), &mut self.is_hand_raised));
        entities.push((self.is_in_meeting.id.clone(), &mut self.is_in_meeting));
        entities.push((self.is_recording_on.id.clone(), &mut self.is_recording_on));
        entities.push((
            self.is_background_blurred.id.clone(),
            &mut self.is_background_blurred,
        ));
        entities.push((self.is_sharing.id.clone(), &mut self.is_sharing));
        entities.push((
            self.has_unread_messages.id.clone(),
            &mut self.has_unread_messages,
        ));
        entities.into_iter()
    }
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
            additional_attributes: HashMap::new(),
        },

        is_video_on: HaEntity {
            id: "binary_sensor.teams_video".to_string(),
            friendly_name: "Teams Video".to_string(),
            icons: HaIcons {
                on: "mdi:webcam".to_string(),
                off: "mdi:webcam-off".to_string(),
            },
            additional_attributes: HashMap::new(),
        },

        is_hand_raised: HaEntity {
            id: "binary_sensor.teams_hand_raised".to_string(),
            friendly_name: "Teams Hand Raised".to_string(),
            icons: HaIcons {
                on: "mdi:hand-back-left".to_string(),
                off: "mdi:hand-back-left-off".to_string(),
            },
            additional_attributes: HashMap::new(),
        },

        is_in_meeting: HaEntity {
            id: "binary_sensor.teams_meeting".to_string(),
            friendly_name: "Teams Meeting".to_string(),
            icons: HaIcons {
                on: "mdi:phone-in-talk".to_string(),
                off: "mdi:phone-off".to_string(),
            },
            additional_attributes: HashMap::new(),
        },

        is_recording_on: HaEntity {
            id: "binary_sensor.teams_recording".to_string(),
            friendly_name: "Teams Recording".to_string(),
            icons: HaIcons {
                on: "mdi:record-rec".to_string(),
                off: "mdi:power-off".to_string(),
            },
            additional_attributes: HashMap::new(),
        },

        is_background_blurred: HaEntity {
            id: "binary_sensor.teams_background_blurred".to_string(),
            friendly_name: "Teams Background Blurred".to_string(),
            icons: HaIcons {
                on: "mdi:blur".to_string(),
                off: "mdi:blur-off".to_string(),
            },
            additional_attributes: HashMap::new(),
        },

        is_sharing: HaEntity {
            id: "binary_sensor.teams_sharing".to_string(),
            friendly_name: "Teams Sharing".to_string(),
            icons: HaIcons {
                on: "mdi:projector-screen".to_string(),
                off: "mdi:projector-screen-off".to_string(),
            },
            additional_attributes: HashMap::new(),
        },

        has_unread_messages: HaEntity {
            id: "binary_sensor.teams_unread_messages".to_string(),
            friendly_name: "Teams Unread Messages".to_string(),
            icons: HaIcons {
                on: "mdi:message-alert".to_string(),
                off: "mdi:message-off".to_string(),
            },
            additional_attributes: HashMap::new(),
        },
    };

    HaConfiguration {
        long_live_token: "".to_string(),
        url: "".to_string(),
        entities: ha_entities,
    }
}
