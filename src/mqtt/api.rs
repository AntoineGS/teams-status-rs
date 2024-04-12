use crate::mqtt::configuration::MqttConfiguration;
use crate::teams_ws::states::TeamsStates;
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
            mqtt_configuration.url(),
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
    async fn notify_changed(&self, teams_states: &TeamsStates, _: bool) -> anyhow::Result<()> {
        let muted = &*bool_to_str(teams_states.is_muted.load(Ordering::Relaxed));
        let video_on = &*bool_to_str(teams_states.is_video_on.load(Ordering::Relaxed));
        let hand_raised = &*bool_to_str(teams_states.is_hand_raised.load(Ordering::Relaxed));
        let in_meeting = &*bool_to_str(teams_states.is_in_meeting.load(Ordering::Relaxed));
        let recording = &*bool_to_str(teams_states.is_recording_on.load(Ordering::Relaxed));
        let background_blurred =
            &*bool_to_str(teams_states.is_background_blurred.load(Ordering::Relaxed));
        let sharing = &*bool_to_str(teams_states.is_sharing.load(Ordering::Relaxed));
        let unread_messages =
            &*bool_to_str(teams_states.has_unread_messages.load(Ordering::Relaxed));

        let mqtt_entities = &self.mqtt_configuration.mqtt_entities;

        let payload = json!({
            &mqtt_entities.muted:muted,
            &mqtt_entities.video:video_on,
            &mqtt_entities.hand_raised:hand_raised,
            &mqtt_entities.meeting:in_meeting,
            &mqtt_entities.recording:recording,
            &mqtt_entities.background_blurred:background_blurred,
            &mqtt_entities.sharing:sharing,
            &mqtt_entities.unread_messages:unread_messages,
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

    fn reconnect(&mut self) {
        let mut mqtt_options = MqttOptions::new(
            "teams-status",
            self.mqtt_configuration.url(),
            self.mqtt_configuration.port,
        );

        mqtt_options.set_credentials(&self.mqtt_configuration.username, &self.mqtt_configuration.password);
        mqtt_options.set_keep_alive(Duration::from_secs(5));
        let (client, mut event_loop) = AsyncClient::new(mqtt_options, 10);

        self.client = client;
        // mqttc requires this to work
        task::spawn(async move { while let Ok(_) = event_loop.poll().await {} });
    }
}
