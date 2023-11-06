use crate::ha_configuration::HaConfiguration;
use crate::teams_states::TeamsStates;
use crate::utils::bool_to_str;
use home_assistant_rest::post::StateParams;
use home_assistant_rest::Client;
use log::error;
use std::collections::HashMap;
use std::sync::atomic::Ordering;

pub struct HaApi {
    client: Client,
}

impl HaApi {
    pub fn new(_ha_base_configuration: &HaConfiguration) -> Self {
        let ha_base_configuration = _ha_base_configuration;
        let client = Client::new(
            &*ha_base_configuration.url,
            &*ha_base_configuration.long_live_token,
        )
        .unwrap();
        Self { client }
    }

    /* friendly_name is needed as API calls wipe the configured name */
    pub async fn update_ha(&self, state: &str, icon: &str, friendly_name: &str, entity_id: &str) {
        let api_status = self.client.get_api_status().await.unwrap();

        if api_status.message != "API running." {
            error!("Home Assistant API cannot be reached");
            return;
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
        }
    }

    pub async fn notify_changed(&self, teams_status: &TeamsStates) {
        let in_meeting = &*bool_to_str(teams_status.in_meeting.load(Ordering::Relaxed));
        let icon = if teams_status.in_meeting.load(Ordering::Relaxed) {
            "mdi:phone"
        } else {
            "mdi:phone-in-talk"
        };
        self.update_ha(
            in_meeting,
            icon,
            "Teams Meeting",
            "binary_sensor.teams_meeting",
        )
        .await;

        let camera_on = &*bool_to_str(teams_status.camera_on.load(Ordering::Relaxed));
        let icon = if teams_status.camera_on.load(Ordering::Relaxed) {
            "mdi:camera"
        } else {
            "mdi:camera-off"
        };
        self.update_ha(
            camera_on,
            icon,
            "Teams Camera",
            "binary_sensor.teams_camera",
        )
        .await;
    }
}

#[allow(unused_imports)]
mod tests {
    // use crate::ha_api::{HaApi, ENV_HA_LONG_LIVE_TOKEN, ENV_HA_URL};
    // use chrono::Utc;
    // use dotenv::dotenv;
    // use home_assistant_rest::get::StateEnum;
    //
    // // Cannot use consts in should_panic, see:
    // // https://internals.rust-lang.org/t/passing-variables-or-constants-as-arguments-to-the-should-panic-expected-attribute-macro/16695
    // #[test]
    // #[should_panic(expected = "TSHATOKEN")]
    // fn new_token_not_set_will_panic() {
    //     std::env::set_var(ENV_HA_URL, "1234");
    //     std::env::set_var(ENV_HA_LONG_LIVE_TOKEN, "");
    //     HaApi::new();
    // }
    //
    // #[test]
    // #[should_panic(expected = "TSHAURL")]
    // fn new_url_not_set_will_panic() {
    //     std::env::set_var(ENV_HA_URL, "");
    //     std::env::set_var(ENV_HA_LONG_LIVE_TOKEN, "1234");
    //     HaApi::new();
    // }

    // I have not found a way to query friendly_name and icon to confirm this test
    #[actix_rt::test]
    async fn update_ha_state_will_match() {
        dotenv().ok();
        let random_state = &*Utc::now().to_string();
        let ha_api = HaApi::new();

        ha_api
            .update_ha(
                random_state,
                "Microsoft Teams Activity",
                "mdi:phone",
                "sensor.teams_activity",
            )
            .await;

        let states_entity = ha_api
            .client
            .get_states_of_entity("sensor.teams_activity")
            .await
            .unwrap();

        if let Some(state) = states_entity.state {
            match state {
                StateEnum::String(x) => assert_eq!(random_state, x),
                _ => panic!("Invalid data type detected for entity state."),
            }
        } else {
            panic!("Error reading entity states.")
        }
    }
}
