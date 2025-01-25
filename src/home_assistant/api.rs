use crate::home_assistant::configuration::{HaConfiguration, HaEntity};
use crate::teams_ws::states::TeamsStates;
use crate::traits::Listener;
use crate::utils::bool_to_str;
use anyhow::anyhow;
use async_trait::async_trait;
use futures_util::future::try_join_all;
use home_assistant_rest::post::StateParams;
use home_assistant_rest::Client;
use log::{error, info};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};

pub struct HaApi {
    ha_configuration: HaConfiguration,
    queried_attributes: bool,
}

impl HaApi {
    pub fn new(ha_configuration: HaConfiguration) -> anyhow::Result<Self> {
        Ok(Self {
            ha_configuration,
            queried_attributes: false,
        })
    }

    // fetch state of all entities used in the application, remove attributes that we are handling here, and save the rest to the entity struct
    async fn update_ha_entities_with_attributes(&mut self) -> anyhow::Result<()> {
        if self.queried_attributes {
            return Ok(());
        }

        let client = Client::new(
            &self.ha_configuration.url,
            &self.ha_configuration.long_live_token,
        )?;
        let api_status = client.get_api_status().await;

        if api_status.is_err() || api_status?.message != "API running." {
            error!("Home Assistant API cannot be reached");
            return Err(anyhow!("Home Assistant API cannot be reached"));
        }

        for (entity_id, entity) in self.ha_configuration.entities.iter_mut() {
            let state = client.get_states_of_entity(&entity_id).await;

            if state.is_err() {
                error!("Error fetching entity state: {}", state.unwrap_err());
                continue;
            }

            for (key, value) in state?.attributes {
                if key != "friendly_name" && key != "icon" {
                    info!(
                        "Adding attribute '{}' of value '{}' to entity '{}'",
                        &key, &value, entity_id
                    );
                    // the following will convert non-string values to string unfortunately
                    entity.additional_attributes.insert(
                        key,
                        value.as_str().unwrap_or(&*value.to_string()).to_string(),
                    );
                }
            }
        }

        self.queried_attributes = true;
        Ok(())
    }

    // friendly_name is needed as API calls wipe the configured name
    async fn update_ha(
        &self,
        state: &AtomicBool,
        prev_state: &AtomicBool,
        ha_entity: &HaEntity,
        force_update: bool,
    ) -> anyhow::Result<()> {
        let state_bool = state.load(Ordering::Relaxed);
        let prev_state_bool = prev_state.load(Ordering::Relaxed);

        // we exit early if nothing has changed, and we are not forcing an update
        if state_bool == prev_state_bool && !force_update {
            return Ok(());
        }

        let client = Client::new(
            &self.ha_configuration.url,
            &self.ha_configuration.long_live_token,
        )?;
        // use this code to connect and get attributes upon starting the application
        let api_status = client.get_api_status().await;

        if api_status.is_err() || api_status?.message != "API running." {
            error!("Home Assistant API cannot be reached");
            return Err(anyhow!("Home Assistant API cannot be reached"));
        }

        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert(
            "friendly_name".to_string(),
            ha_entity.friendly_name.to_string(),
        );

        let icon = if state_bool {
            &ha_entity.icons.on
        } else {
            &ha_entity.icons.off
        };

        attributes.insert("icon".to_string(), icon.to_string());

        for (key, value) in &ha_entity.additional_attributes {
            attributes.insert(key.to_string(), value.to_string());
        }

        let state_str = bool_to_str(state_bool);
        let params = StateParams {
            entity_id: ha_entity.id.to_string(),
            state: state_str.clone(),
            attributes,
        };

        info!("Updating HA entity ({}) to '{}'", &ha_entity.id, &state_str);

        let post_states_res = client.post_states(params).await;

        if post_states_res.is_err() {
            error!("{}", post_states_res.unwrap_err());
        };

        prev_state.store(state_bool, Ordering::Relaxed);

        Ok(())
    }
}

#[async_trait]
impl Listener for HaApi {
    async fn notify_changed(
        &mut self,
        teams_states: &TeamsStates,
        force_update: bool,
    ) -> anyhow::Result<()> {
        self.update_ha_entities_with_attributes().await?;

        // Reflection would be nice here... Tried with bevy_reflect but ran into an issue with AtomicBool
        let mut futures = Vec::new();
        // let is_in_meeting = self.ha_configuration.entities.is_in_meeting.clone();

        futures.push(self.update_ha(
            &teams_states.is_in_meeting,
            &teams_states.prev_is_in_meeting,
            &self.ha_configuration.entities.is_in_meeting,
            force_update,
        ));

        futures.push(self.update_ha(
            &teams_states.is_video_on,
            &teams_states.prev_is_video_on,
            &self.ha_configuration.entities.is_video_on,
            force_update,
        ));

        futures.push(self.update_ha(
            &teams_states.is_muted,
            &teams_states.prev_is_muted,
            &self.ha_configuration.entities.is_muted,
            force_update,
        ));

        futures.push(self.update_ha(
            &teams_states.is_hand_raised,
            &teams_states.prev_is_hand_raised,
            &self.ha_configuration.entities.is_hand_raised,
            force_update,
        ));

        futures.push(self.update_ha(
            &teams_states.is_recording_on,
            &teams_states.prev_is_recording_on,
            &self.ha_configuration.entities.is_recording_on,
            force_update,
        ));

        futures.push(self.update_ha(
            &teams_states.is_background_blurred,
            &teams_states.prev_is_background_blurred,
            &self.ha_configuration.entities.is_background_blurred,
            force_update,
        ));

        futures.push(self.update_ha(
            &teams_states.is_sharing,
            &teams_states.prev_is_sharing,
            &self.ha_configuration.entities.is_sharing,
            force_update,
        ));

        futures.push(self.update_ha(
            &teams_states.has_unread_messages,
            &teams_states.prev_has_unread_messages,
            &self.ha_configuration.entities.has_unread_messages,
            force_update,
        ));

        try_join_all(futures).await?;

        Ok(())
    }

    fn reconnect(&mut self) {
        // considered not needed for now, as I believe the API will reconnect upon failure (not tested)
    }
}
