use chrono::prelude::*;
use dotenv::dotenv;
use home_assistant_rest::post::StateParams;
use home_assistant_rest::{get::StateEnum, Client};
use log::error;
use std::collections::HashMap;

const ENV_HA_LONG_LIVE_TOKEN: &str = "TSHATOKEN";
const ENV_HA_URL: &str = "TSHAURL";

pub struct HAApi {
    // ha_client: Arc<RwLock<HomeAssistantAPI>>
    client: Client,
}

impl HAApi {
    pub fn new() -> Self {
        let ha_token = Self::get_env_var(ENV_HA_LONG_LIVE_TOKEN);
        let ha_url = Self::get_env_var(ENV_HA_URL);
        let client = Client::new(&*ha_url, &*ha_token).unwrap();
        Self { client }
    }

    fn get_env_var(key: &str) -> String {
        // I would have liked to convert the "env..." string to be a const but was unable to make
        // it work :(
        let error_msg = &*format!("{} env variable must be set", key);
        let env_var = std::env::var(key).expect(error_msg);
        env_var
    }

    pub async fn update_ha(&self, state: &str, friendly_name: &str, icon: &str, entity_id: &str) {
        let api_status = self.client.get_api_status().await.unwrap();

        if api_status.message != "API running." {
            error!("API is not running");
            return;
        }

        let mut attributes = HashMap::new();
        attributes.insert("friendly_name", friendly_name);
        attributes.insert("icon", icon);

        let params = StateParams {
            entity_id: entity_id.to_string(),
            state: state.to_string(),
            attributes: Default::default(),
        };

        let post_states_res = self.client.post_states(params).await;

        if post_states_res.is_err() {
            // error!(post_states_res.unwrap());
        }
    }
}

// Cannot use consts in should_panic, see:
// https://internals.rust-lang.org/t/passing-variables-or-constants-as-arguments-to-the-should-panic-expected-attribute-macro/16695
#[test]
#[should_panic(expected = "TSHATOKEN")]
fn new_token_not_set_will_raise() {
    std::env::set_var(ENV_HA_URL, "1234");
    HAApi::new();
}

#[test]
#[should_panic(expected = "TSHAURL")]
fn new_url_not_set_will_raise() {
    std::env::set_var(ENV_HA_LONG_LIVE_TOKEN, "1234");
    HAApi::new();
}

#[actix_rt::test]
async fn update_ha_will_work() {
    dotenv().ok();
    let random_state = &*Utc::now().to_string();
    let ha_api = HAApi::new();

    ha_api
        .update_ha(
            random_state,
            "Microsoft Teams Activity",
            "mdi:phone-on",
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

    let attributes = states_entity.attributes;

    assert_eq!(
        random_state,
        attributes
            .get("friendly_name")
            .expect("friendly_name not returned")
    );

    // if let Some(state) = states_entity.state {
    //     match state {
    //         StateEnum::String(x) => assert_eq!(random_state, x),
    //         _ => panic!("Invalid data type detected for entity state."),
    //     }
    // } else {
    //     panic!("Error reading entity states.")
    // }
}
