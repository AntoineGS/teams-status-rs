use crate::mqtt::configuration::MqttConfiguration;
use crate::teams::states::TeamsStates;
use crate::traits::Listener;
use crate::utils::bool_to_str;
use async_trait::async_trait;
use rumqttc::{AsyncClient, MqttOptions, QoS};
use serde_json::json;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::task;

pub struct MqttApi {
    client: AsyncClient,
    mqtt_configuration: MqttConfiguration,
}

impl MqttApi {
    pub fn new(mqtt_configuration: MqttConfiguration) -> anyhow::Result<Self> {
        let mut mqtt_options = MqttOptions::new(
            "teams-status",
            &mqtt_configuration.url,
            mqtt_configuration.port,
        );

        mqtt_options.set_credentials(&mqtt_configuration.username, &mqtt_configuration.password);
        mqtt_options.set_keep_alive(Duration::from_secs(5));
        let (client, mut event_loop) = AsyncClient::new(mqtt_options, 10);

        // mqttc requires this to work
        task::spawn(async move { while let Ok(_) = event_loop.poll().await {} });

        Ok(Self {
            client,
            mqtt_configuration,
        })
    }
}

#[async_trait]
impl Listener for MqttApi {
    async fn notify_changed(&self, teams_states: &TeamsStates) -> anyhow::Result<()> {
        let in_meeting = &*bool_to_str(teams_states.is_in_meeting.load(Ordering::Relaxed));
        let video_on = &*bool_to_str(teams_states.is_video_on.load(Ordering::Relaxed));

        let payload = json!({
            &self.mqtt_configuration.mqtt_entities.meeting:in_meeting,
            &self.mqtt_configuration.mqtt_entities.video:video_on,
        });

        // todo: log failures
        let _ = &self
            .client
            .publish(
                &self.mqtt_configuration.topic,
                QoS::AtLeastOnce,
                true,
                payload.to_string(),
            )
            .await?;

        Ok(())
    }
}
