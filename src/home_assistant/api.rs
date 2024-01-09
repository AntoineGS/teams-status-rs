use crate::home_assistant::configuration::HaConfiguration;
use crate::teams::states::TeamsStates;
use crate::traits::Listener;
use crate::utils::bool_to_str;
use anyhow::anyhow;
use async_trait::async_trait;
use home_assistant_rest::post::StateParams;
use home_assistant_rest::Client;
use log::error;
use std::collections::HashMap;
use std::sync::atomic::Ordering;

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
    async fn update_ha(
        &self,
        state: &str,
        icon: &str,
        friendly_name: &str,
        entity_id: &str,
    ) -> anyhow::Result<()> {
        let api_status = self.client.get_api_status().await.unwrap();

        if api_status.message != "API running." {
            error!("Home Assistant API cannot be reached");
            return Err(anyhow!("Home Assistant API cannot be reached"));
        }

        let mut attributes: HashMap<String, String> = HashMap::new();
        attributes.insert("friendly_name".to_string(), friendly_name.to_string());
        attributes.insert("icon".to_string(), icon.to_string());

        let params = StateParams {
            entity_id: entity_id.to_string(),
            state: state.to_string(),
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
        let in_meeting = &*bool_to_str(teams_states.is_in_meeting.load(Ordering::Relaxed));
        let icon = if teams_states.is_in_meeting.load(Ordering::Relaxed) {
            &self.ha_configuration.icons.in_a_meeting
        } else {
            &self.ha_configuration.icons.not_in_a_meeting
        };
        self.update_ha(
            in_meeting,
            icon,
            &self.ha_configuration.entities.meeting_friendly_name,
            &self.ha_configuration.entities.meeting_id,
        )
        .await?;

        let video_on = &*bool_to_str(teams_states.is_video_on.load(Ordering::Relaxed));
        let icon = if teams_states.is_video_on.load(Ordering::Relaxed) {
            &self.ha_configuration.icons.video_on
        } else {
            &self.ha_configuration.icons.video_off
        };
        self.update_ha(
            video_on,
            icon,
            &self.ha_configuration.entities.video_friendly_name,
            &self.ha_configuration.entities.video_id,
        )
        .await?;

        Ok(())
    }
}
