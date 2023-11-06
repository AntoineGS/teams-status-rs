use crate::ha_api::HaApi;
use crate::teams_configuration::TeamsConfiguration;
use crate::teams_states::TeamsStates;
use futures_util::{future, pin_mut, StreamExt};
use log::info;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time;
use tokio::io::AsyncReadExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

pub struct TeamsAPI {
    pub teams_states: Arc<TeamsStates>,
    pub url: String,
}

impl TeamsAPI {
    pub fn new(conf: &TeamsConfiguration) -> Self {
        let teams_states = Arc::new(TeamsStates {
            camera_on: AtomicBool::new(false),
            in_meeting: AtomicBool::new(false),
        });

        let url = format!(
            "{url}?token={token}&protocol-version=1.0.0",
            url = conf.url,
            token = conf.api_token
        );

        Self { teams_states, url }
    }

    // todo: HaApi creates a dependency on Home Assistant, could be fun to abstract it
    pub async fn start_listening(&self, listener: Arc<HaApi>, is_running: Arc<AtomicBool>) {
        let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
        tokio::spawn(read_stdin(stdin_tx));
        let url_local = url::Url::parse(&self.url).unwrap();
        let (ws_stream, _) = connect_async(url_local).await.expect("Failed to connect");
        let (write, read) = ws_stream.split();
        let stdin_to_ws = stdin_rx.map(Ok).forward(write); // this here finishes right away
        let ws_to_stdout = {
            read.for_each(|message| async {
                let data = &message.unwrap().into_data();
                let json = String::from_utf8_lossy(data);
                info!("{}", json);
                parse_data(&json, listener.clone(), self.teams_states.clone()).await;
            })
        };

        let running_future = async {
            let one_second = time::Duration::from_secs(1);

            while is_running.load(Ordering::Relaxed) {
                tokio::time::sleep(one_second).await;
            }
            info!("Application close requested");
        };

        pin_mut!(stdin_to_ws, running_future, ws_to_stdout);
        let ws_futures = async {
            future::select(stdin_to_ws, ws_to_stdout).await;
        };

        pin_mut!(ws_futures);
        future::select(ws_futures, running_future).await;
    }
}

async fn read_stdin(tx: futures_channel::mpsc::UnboundedSender<Message>) {
    let mut stdin = tokio::io::stdin();
    loop {
        let mut buf = vec![0; 1024];
        let n = match stdin.read(&mut buf).await {
            Err(_) | Ok(0) => break,
            Ok(n) => n,
        };
        buf.truncate(n);
        tx.unbounded_send(Message::binary(buf)).unwrap();
    }
    info!("Exiting read_stdin")
}

async fn parse_data(json: &str, listener: Arc<HaApi>, teams_states: Arc<TeamsStates>) {
    let answer = json::parse(&json.to_string()).unwrap();
    let new_in_meeting = answer["meetingUpdate"]["meetingState"]["isInMeeting"]
        .as_bool()
        .expect("Unable to locate isInMeeting variable in JSON");
    let new_camera_on = answer["meetingUpdate"]["meetingState"]["isCameraOn"]
        .as_bool()
        .expect("Unable to locate isCameraOn variable in JSON");

    if (teams_states
        .in_meeting
        .swap(new_in_meeting, Ordering::Relaxed)
        != new_in_meeting)
        || (teams_states
            .camera_on
            .swap(new_in_meeting, Ordering::Relaxed)
            != new_camera_on)
    {
        listener.notify_changed(&teams_states).await;
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
