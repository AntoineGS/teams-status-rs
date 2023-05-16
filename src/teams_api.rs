use crate::ha_api::HAApi;
use crate::teams_states::TeamsStates;
use crate::utils;
use tungstenite::{connect, Message};
use url::Url;

const ENV_API_TOKEN: &str = "TSAPITOKEN";

pub struct TeamsAPI {
    api_token: String,
    listener: HAApi,
    teams_states: TeamsStates,
}

impl TeamsAPI {
    pub fn new(listener: HAApi) -> Self {
        let api_token = utils::get_env_var(ENV_API_TOKEN);
        let teams_states = TeamsStates {
            camera_on: false,
            in_meeting: false,
        };
        Self {
            api_token,
            listener,
            teams_states,
        }
    }

    pub async fn listen_loop(&mut self) {
        let url = &format!(
            "ws://localhost:8124?token={}&protocol-version=1.0.0",
            self.api_token
        );
        let (mut socket, _response) =
            connect(Url::parse(url).unwrap()).expect("Unable to connect to Teams API");

        // println!("Connected to the server");
        // println!("Response HTTP code: {}", response.status());
        // println!("Response contains the following headers:");
        // for (ref header, _value) in response.headers() {
        //     println!("* {}", header);
        // }

        loop {
            let msg = socket.read_message().expect("Error reading message");
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
