use crate::teams::configuration::{
    change_teams_configuration, TeamsConfiguration, TEAMS, TEAMS_API_TOKEN,
};
use crate::teams::states::TeamsStates;
use crate::traits::Listener;
use anyhow::Context;
use futures_util::{future, pin_mut, SinkExt, StreamExt};
use json::JsonValue;
use log::{error, info};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

const JSON_MEETING_UPDATE: &str = "meetingUpdate";
const JSON_MEETING_STATE: &str = "meetingState";
const JSON_IS_MUTED: &str = "isMuted";
const JSON_IS_VIDEO_ON: &str = "isVideoOn";
const JSON_IS_HAND_RAISED: &str = "isHandRaised";
const JSON_IS_IN_MEETING: &str = "isInMeeting";
const JSON_IS_RECORDING_ON: &str = "isRecordingOn";
const JSON_IS_BACKGROUND_BLURRED: &str = "isBackgroundBlurred";
const JSON_IS_SHARING: &str = "isSharing";
const JSON_HAS_UNREAD_MESSAGES: &str = "hasUnreadMessages";
const JSON_TOKEN_REFRESH: &str = "tokenRefresh";
pub struct TeamsAPI {
    pub teams_states: Arc<TeamsStates>,
    pub url: String,
}

impl TeamsAPI {
    pub fn new(conf: &TeamsConfiguration) -> Self {
        let teams_states = Arc::new(TeamsStates {
            is_muted: AtomicBool::new(false),
            is_video_on: AtomicBool::new(false),
            is_hand_raised: AtomicBool::new(false),
            is_in_meeting: AtomicBool::new(false),
            is_recording_on: AtomicBool::new(false),
            is_background_blurred: AtomicBool::new(false),
            is_sharing: AtomicBool::new(false),
            has_unread_messages: AtomicBool::new(false),
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

    pub async fn start_listening(
        &self,
        listener: Arc<Box<dyn Listener>>,
        is_running: Arc<AtomicBool>,
        toggle_mute: Arc<AtomicBool>,
    ) -> anyhow::Result<()> {
        let url_local = url::Url::parse(&self.url)?;
        let (ws_stream, _) = connect_async(url_local)
            .await
            .with_context(|| "Failed to connect")?;
        let (mut write, read) = ws_stream.split();
        let force_update = Arc::new(AtomicBool::new(true));
        let ws_to_parser = {
            read.for_each(|message| async {
                if message.is_ok() {
                    let data = &message.unwrap().into_data();
                    let json = String::from_utf8_lossy(data);
                    info!("{}", json);
                    let parse_result = parse_data(
                        &json,
                        listener.clone(),
                        self.teams_states.clone(),
                        force_update.clone(),
                    )
                    .await;

                    if parse_result.is_err() {
                        error!("{}", parse_result.unwrap_err())
                    }
                }
            })
        };

        let running_future = async {
            let one_second = time::Duration::from_secs(1);

            while is_running.load(Ordering::Relaxed) {
                tokio::time::sleep(one_second).await;

                if toggle_mute.load(Ordering::Relaxed) {
                    let msg = Message::text(
                        r#"{"requestId":1,"apiVersion":"2.0.0","action":"toggle-mute"}"#,
                    );

                    write.send(msg).await.unwrap();
                    toggle_mute.swap(false, Ordering::Relaxed);
                }
            }

            info!("Application close requested");
        };

        pin_mut!(running_future, ws_to_parser);
        future::select(running_future, ws_to_parser).await;
        Ok(())
    }
}

async fn update_value(
    teams_state_value: &AtomicBool,
    answer: &JsonValue,
    json_value_name: &str,
) -> bool {
    let new_value = answer[JSON_MEETING_UPDATE][JSON_MEETING_STATE][json_value_name]
        .as_bool()
        .unwrap_or_else(|| {
            error!("Unable to locate {} variable in JSON", json_value_name);
            false
        });

    teams_state_value.swap(new_value, Ordering::Relaxed) != new_value
}

async fn parse_data(
    json: &str,
    listener: Arc<Box<dyn Listener>>,
    teams_states: Arc<TeamsStates>,
    force_update: Arc<AtomicBool>,
) -> anyhow::Result<()> {
    let answer = json::parse(&json.to_string()).unwrap_or(json::parse("{}").unwrap());

    if answer.has_key(JSON_MEETING_UPDATE) {
        let mut has_changed = update_value(&teams_states.is_muted, &answer, JSON_IS_MUTED).await;
        has_changed |= update_value(&teams_states.is_video_on, &answer, JSON_IS_VIDEO_ON).await;
        has_changed |=
            update_value(&teams_states.is_hand_raised, &answer, JSON_IS_HAND_RAISED).await;
        has_changed |= update_value(&teams_states.is_in_meeting, &answer, JSON_IS_IN_MEETING).await;
        has_changed |=
            update_value(&teams_states.is_recording_on, &answer, JSON_IS_RECORDING_ON).await;
        has_changed |= update_value(
            &teams_states.is_background_blurred,
            &answer,
            JSON_IS_BACKGROUND_BLURRED,
        )
        .await;
        has_changed |= update_value(&teams_states.is_sharing, &answer, JSON_IS_SHARING).await;
        has_changed |= update_value(
            &teams_states.has_unread_messages,
            &answer,
            JSON_HAS_UNREAD_MESSAGES,
        )
        .await;

        if force_update.swap(false, Ordering::Relaxed) || has_changed {
            listener.notify_changed(&teams_states).await?;
        }
    } else if answer.has_key(JSON_TOKEN_REFRESH) && !answer[JSON_TOKEN_REFRESH].is_empty() {
        change_teams_configuration(
            TEAMS,
            TEAMS_API_TOKEN,
            &answer[JSON_TOKEN_REFRESH].to_string(),
        )
    }

    Ok(())
}
