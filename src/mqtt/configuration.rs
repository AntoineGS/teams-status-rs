pub const MQTT: &str = "MQTT";
pub const MQTT_URL: &str = "URL";
pub const MQTT_PORT: &str = "Port";
pub const MQTT_TOPIC: &str = "Topic";
pub const MQTT_USERNAME: &str = "Username";
pub const MQTT_PASSWORD: &str = "Password";
pub const MQTT_ENTITIES: &str = "MQTT Entities";
pub const MQTT_MUTED: &str = "Muted";
pub const MQTT_VIDEO: &str = "Video";
pub const MQTT_HAND_RAISED: &str = "Hand Raised";
pub const MQTT_MEETING: &str = "Meeting";
pub const MQTT_RECORDING: &str = "Recording";
pub const MQTT_BACKGROUND_BLURRED: &str = "Background Blurred";
pub const MQTT_SHARING: &str = "Sharing";
pub const MQTT_UNREAD_MESSAGES: &str = "Unread Messages";
pub const MQTT_PORT_DEFAULT: u16 = 1883;

pub struct MqttEntities {
    pub muted: String,
    pub video: String,
    pub hand_raised: String,
    pub meeting: String,
    pub recording: String,
    pub background_blurred: String,
    pub sharing: String,
    pub unread_messages: String,
}

pub struct MqttConfiguration {
    url: String,
    pub port: u16,
    pub topic: String,
    pub username: String,
    pub password: String,
    pub mqtt_entities: MqttEntities,
}

impl MqttConfiguration {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn set_url(&mut self, url: String) {
        self.url = if url.to_lowercase().starts_with("mqtt://") {
            url[7..].to_string()
        } else {
            url
        };
    }
}

pub fn create_mqtt_configuration() -> MqttConfiguration {
    let mqtt_entities = MqttEntities {
        muted: "muted".to_string(),
        video: "video_on".to_string(),
        hand_raised: "hand_raised".to_string(),
        meeting: "in_meeting".to_string(),
        recording: "recording_on".to_string(),
        background_blurred: "background_blurred".to_string(),
        sharing: "sharing".to_string(),
        unread_messages: "unread_messages".to_string(),
    };

    MqttConfiguration {
        url: "".to_string(),
        port: 1883,
        topic: "teams-status".to_string(),
        username: "".to_string(),
        password: "".to_string(),
        mqtt_entities,
    }
}
