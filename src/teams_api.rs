use crate::ha_api::HaApi;
use crate::teams_configuration::{
    change_teams_configuration, TeamsConfiguration, TEAMS, TEAMS_API_TOKEN,
};
use crate::teams_states::TeamsStates;
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use log::{error, info};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

const JSON_MEETING_UPDATE: &str = "meetingUpdate";
const JSON_MEETING_STATE: &str = "meetingState";
const JSON_IS_IN_MEETING: &str = "isInMeeting";
const JSON_IS_VIDEO_ON: &str = "isVideoOn";
const JSON_TOKEN_REFRESH: &str = "tokenRefresh";
pub struct TeamsAPI {
    pub teams_states: Arc<TeamsStates>,
    pub url: String,
}

impl TeamsAPI {
    pub fn new(conf: &TeamsConfiguration) -> Self {
        let teams_states = Arc::new(TeamsStates {
            is_video_on: AtomicBool::new(false),
            is_in_meeting: AtomicBool::new(false),
        });

        let api_token = if !conf.api_token.is_empty() {
            format!("token={}&", &conf.api_token)
        } else {
            "".to_string()
        };
        let url = format!(
            "{url}?{api_token}protocol-version=2.0.0&manufacturer=HA-Integration&device=MyPC&app=teams-status-rs&app-version=1.0", 
            url = conf.url,
            api_token = api_token);

        Self { teams_states, url }
    }

    // todo: HaApi creates a dependency on Home Assistant, could be fun to abstract it
    pub async fn start_listening(
        &self,
        listener: Arc<HaApi>,
        is_running: Arc<AtomicBool>,
        toggle_mute: Arc<AtomicBool>,
    ) {
        let url_local = url::Url::parse(&self.url).unwrap();
        let (ws_stream, _) = connect_async(url_local).await.expect("Failed to connect");
        let (mut write, read) = ws_stream.split();
        let force_update = Arc::new(AtomicBool::new(true));
        let ws_to_parser = {
            read.for_each(|message| async {
                let data = &message.unwrap().into_data();
                let json = String::from_utf8_lossy(data);
                info!("{}", json);
                parse_data(
                    &json,
                    listener.clone(),
                    self.teams_states.clone(),
                    force_update.clone(),
                )
                .await;
            })
        };

        let running_future = async {
            let one_second = time::Duration::from_secs(1);

            while is_running.load(Ordering::Relaxed) {
                tokio::time::sleep(one_second).await;

                if toggle_mute.load(Ordering::Relaxed) {
                    let msg = Message::text(
                        r#"{"requestId":1,"apiVersion":"2.0.0","service":"toggle-mute","action":"toggle-mute"}"#,
                    );

                    write.send(msg).await.unwrap();
                    toggle_mute.swap(false, Ordering::Relaxed);
                }
            }

            info!("Application close requested");
        };

        pin_mut!(running_future, ws_to_parser);
        future::select(running_future, ws_to_parser).await;
    }
}

async fn parse_data(
    json: &str,
    listener: Arc<HaApi>,
    teams_states: Arc<TeamsStates>,
    force_update: Arc<AtomicBool>,
) {
    let answer = json::parse(&json.to_string()).unwrap_or(json::parse("{}").unwrap());

    if answer.has_key(JSON_MEETING_UPDATE) {
        let new_in_meeting = answer[JSON_MEETING_UPDATE][JSON_MEETING_STATE][JSON_IS_IN_MEETING]
            .as_bool()
            .unwrap_or_else(|| {
                error!("Unable to locate {} variable in JSON", JSON_IS_VIDEO_ON);
                false
            });

        let new_video_on = answer[JSON_MEETING_UPDATE][JSON_MEETING_STATE][JSON_IS_VIDEO_ON]
            .as_bool()
            .unwrap_or_else(|| {
                error!("Unable to locate {} variable in JSON", JSON_IS_VIDEO_ON);
                false
            });

        let is_in_meeting_changed = teams_states
            .is_in_meeting
            .swap(new_in_meeting, Ordering::Relaxed)
            != new_in_meeting;

        let is_video_on_changed = teams_states
            .is_video_on
            .swap(new_video_on, Ordering::Relaxed)
            != new_video_on;

        if force_update.swap(false, Ordering::Relaxed)
            || is_in_meeting_changed
            || is_video_on_changed
        {
            listener.notify_changed(&teams_states).await;
        }
    } else if answer.has_key(JSON_TOKEN_REFRESH) && !answer[JSON_TOKEN_REFRESH].is_empty() {
        change_teams_configuration(
            TEAMS,
            TEAMS_API_TOKEN,
            &answer[JSON_TOKEN_REFRESH].to_string(),
        )
    }
}

// mod tests {
//     #[test]
//     #[should_panic(expected = "TSAPITOKEN")]
//     fn new_missing_api_key_will_panic() {
//         std::env::set_var(ENV_API_TOKEN, "");
//         TeamsAPI::new(Arc::new(DefaultListener {}));
//     }
//
//     #[test]
//     fn listen_test() {
//         let mut api = TeamsAPI::new(Arc::new(DefaultListener {}));
//         api.listen_loop();
//     }
// }
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
