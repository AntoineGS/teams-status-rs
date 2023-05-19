use crate::ha_api::HAApi;
use crate::teams_states::TeamsStates;
use crate::utils;
use std::fmt::{Debug, Formatter};
use std::net::TcpStream;
use std::ops::Deref;
use std::sync::mpsc::Receiver;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};
use url::Url;

const ENV_API_TOKEN: &str = "TSAPITOKEN";

pub struct TeamsAPI {
    listener: HAApi,
    teams_states: TeamsStates,
    pub socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

impl TeamsAPI {
    pub fn new(listener: HAApi) -> Self {
        let api_token = utils::get_env_var(ENV_API_TOKEN);
        let teams_states = TeamsStates {
            camera_on: false,
            in_meeting: false,
        };

        let url = &format!(
            "ws://localhost:8124?token={}&protocol-version=1.0.0",
            api_token
        );

        let (socket, _response) =
            connect(Url::parse(url).unwrap()).expect("Unable to connect to Teams API");

        // println!("Connected to the server");
        // println!("Response HTTP code: {}", response.status());
        // println!("Response contains the following headers:");
        // for (ref header, _value) in response.headers() {
        //     println!("* {}", header);
        // }

        Self {
            listener,
            teams_states,
            socket,
        }
    }

    pub async fn listen_loop(&mut self, receiver: Receiver<bool>) {
        loop {
            if receiver.try_recv().unwrap_or(false) {
                break;
            }

            let msg = self.socket.read_message().expect("Error reading message");
            let mut has_changed = false;
            let answer = json::parse(&msg.to_string()).unwrap();

            if answer["meetingUpdate"]["meetingState"]["isInMeeting"]
                .as_bool()
                .expect("Unable to locate isInMeeting variable in JSON")
                != self.teams_states.in_meeting
            {
                self.teams_states.in_meeting = !self.teams_states.in_meeting;
                has_changed = true;
            }

            if answer["meetingUpdate"]["meetingState"]["isCameraOn"]
                .as_bool()
                .expect("Unable to locate isCameraOn variable in JSON")
                != self.teams_states.camera_on
            {
                self.teams_states.camera_on = !self.teams_states.camera_on;
                has_changed = true;
            }

            if has_changed {
                self.listener.notify_changed(&self.teams_states).await;
            }
        }
    }
}

impl Debug for TeamsAPI {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TeamsAPI")
    }
}

// #[test]
// #[should_panic(expected = "TSAPITOKEN")]
// fn new_missing_api_key_will_panic() {
//     std::env::set_var(ENV_API_TOKEN, "");
//     TeamsAPI::new(Arc::new(DefaultListener {}));
// }
//
// #[test]
// fn listen_test() {
//     let mut api = TeamsAPI::new(Arc::new(DefaultListener {}));
//     api.listen_loop();
// }
