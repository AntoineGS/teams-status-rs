use crate::home_assistant::configuration::{HaConfiguration, HaEntity};
use crate::teams::states::TeamsStates;
use crate::traits::Listener;
use crate::utils::bool_to_str;
use anyhow::anyhow;
use async_trait::async_trait;
use home_assistant_rest::post::StateParams;
use home_assistant_rest::Client;
use log::error;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct HaApi {
    client: Client,
    ha_configuration: HaConfiguration,
}

impl HaApi {
    pub fn new(ha_configuration: HaConfiguration) -> anyhow::Result<Self> {
        let client = Client::new(&*ha_configuration.url, &*ha_configuration.long_live_token)?;
        Ok(Self {
            client,
            ha_configuration,
        })
    }

    /* friendly_name is needed as API calls wipe the configured name */
    async fn update_ha(&self, state: &AtomicBool, ha_entity: &HaEntity) -> anyhow::Result<()> {
        let api_status = self.client.get_api_status().await.unwrap();

        if api_status.message != "API running." {
            error!("Home Assistant API cannot be reached");
            return Err(anyhow!("Home Assistant API cannot be reached"));
        }

        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert(
            "friendly_name".to_string(),
            ha_entity.friendly_name.to_string(),
        );

        let state_bool = state.load(Ordering::Relaxed);

        let icon = if state_bool {
            &ha_entity.icons.on
        } else {
            &ha_entity.icons.off
        };

        attributes.insert("icon".to_string(), icon.to_string());

        let params = StateParams {
            entity_id: ha_entity.id.to_string(),
            state: bool_to_str(state_bool),
            attributes,
        };

        let post_states_res = self.client.post_states(params).await;

        if post_states_res.is_err() {
            error!("{}", post_states_res.unwrap_err());
        };

        Ok(())
    }
}

#[async_trait]
impl Listener for HaApi {
    async fn notify_changed(&self, teams_states: &TeamsStates) -> anyhow::Result<()> {
        // Reflection would be nice here... Tried with bevy_reflect but ran into an issue with AtomicBool
        self.update_ha(
            &teams_states.is_muted,
            &self.ha_configuration.entities.is_muted,
        )
        .await?;

        self.update_ha(
            &teams_states.is_video_on,
            &self.ha_configuration.entities.is_video_on,
        )
        .await?;

        self.update_ha(
            &teams_states.is_hand_raised,
            &self.ha_configuration.entities.is_hand_raised,
        )
        .await?;

        self.update_ha(
            &teams_states.is_in_meeting,
            &self.ha_configuration.entities.is_in_meeting,
        )
        .await?;

        self.update_ha(
            &teams_states.is_recording_on,
            &self.ha_configuration.entities.is_recording_on,
        )
        .await?;

        self.update_ha(
            &teams_states.is_background_blurred,
            &self.ha_configuration.entities.is_background_blurred,
        )
        .await?;

        self.update_ha(
            &teams_states.is_sharing,
            &self.ha_configuration.entities.is_sharing,
        )
        .await?;

        self.update_ha(
            &teams_states.has_unread_messages,
            &self.ha_configuration.entities.has_unread_messages,
        )
        .await?;

        Ok(())
    }
}
