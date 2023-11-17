pub const MQTT: &str = "MQTT";
pub const MQTT_URL: &str = "URL";
pub const MQTT_PORT: &str = "Port";
pub const MQTT_TOPIC: &str = "Topic";
pub const MQTT_USERNAME: &str = "Username";
pub const MQTT_PASSWORD: &str = "Password";
pub const MQTT_ENTITIES: &str = "MQTT Entities";
pub const MQTT_MEETING: &str = "Meeting";
pub const MQTT_VIDEO: &str = "Video";
pub const MQTT_PORT_DEFAULT: u16 = 1883;

pub struct MqttEntities {
    pub meeting: String,
    pub video: String,
}

pub struct MqttConfiguration {
    pub url: String,
    pub port: u16,
    pub topic: String,
    pub username: String,
    pub password: String,
    pub mqtt_entities: MqttEntities,
}

pub fn create_mqtt_configuration() -> MqttConfiguration {
    let mqtt_entities = MqttEntities {
        meeting: "in_meeting".to_string(),
        video: "video_on".to_string(),
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
