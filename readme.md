# Setup

- Download teams_status.exe from https://github.com/AntoineGS/teams-status-rs/releases to your Windows computer that runs the Teams client
    - Note: This application works with the "new" Teams 2.0 client for Windows
- Launch the application, it will generate the ini file (conf.ini) in the same folder as the .exe
- Use Windows Task Manager (Details tab) to end the 'teams_status.exe' process
- In Microsoft Teams, enable the Third-Party
  API ([see Microsoft documentation](https://support.microsoft.com/en-us/office/connect-to-third-party-devices-in-microsoft-teams-aabca9f2-47bb-407f-9f9b-81a104a883d6?storagetype=live))
    - The API Token will be generated automatically by the integration, so leave it blank in the configuration file
- Decide on whether you will use MQTT or direct HA integration, setting the URL to the integration will activate it, but
  you can only use one or the other:
    - MQTT
        - Set the URL
        - Set the username and password if applicable
        - Double-check the other configurations, they have default values, but you may want/need to change them
    - HA (Home Assistant)
        - Set the URL
        - In Home Assistant, generate a Long-Lived Access
          Token ([see HA documentation](https://developers.home-assistant.io/docs/auth_api/#long-lived-access-token))
            - Paste it into the conf.ini
        - Double-check the other configurations, they have default values, but you may want/need to change them
        - (optional) Set the entities as persistent in HA, otherwise they will show up as missing if the application
          has been turned off for some time, see [here](#ha-persistent-entities).
- Run the application again
    - It will create the entities in HA automatically when it connects
    - Start a meeting in Teams (you can be the only person in it)
    - From the 'Teams Status' tray icon, right-click, and click on `Toggle Mute`
    - You will get a prompt in Teams to allow the application to use the API
        - If you do not click on time Teams will close the prompt. Simply click on Toggle Mute again.

# HA Persistent Entities

For the entities to persist with the native HA integration, you will need to create the entities manually:

- Warning! If you are already using the integration, make sure all entities are moved from HA and that
  the `teams-status` application is closed. Otherwise, it will duplicate sensors.
- In HA's `configuration.yaml` file, under the following section (add it if you do not have it):

```yaml
template:
  - binary_sensor:
```

```yaml
      - name: "Teams Muted"
        unique_id: "ts_a7703e21-2ae1-4af5-ba77-108f2462004a"
        icon: "mdi:microphone-off"
        state: "{{ None }}"
      - name: "Teams Video"
        unique_id: "ts_38dc82bf-bc6d-491f-84c1-9fbee02641a9"
        icon: "mdi:webcam-off"
        state: "{{ None }}"
      - name: "Teams Hand Raised"
        unique_id: "ts_9e7c62d5-5640-4cfb-8ff7-4eac9922030e"
        icon: "mdi:hand-back-left-off"
        state: "{{ None }}"
      - name: "Teams Meeting"
        unique_id: "ts_74837ead-9946-49c9-8aec-f25c0c031ec5"
        icon: "mdi:phone-off"
        state: "{{ None }}"
      - name: "Teams Recording"
        unique_id: "ts_493dcc2e-7cf6-456a-95b2-8cd029b2300c"
        icon: "mdi:power-off"
        state: "{{ None }}"
      - name: "Teams Background Blurred"
        unique_id: "ts_ae97f0dd-7dc3-4f9b-bfb4-ecbc30b8957b"
        icon: "mdi:blur-off"
        state: "{{ None }}"
      - name: "Teams Sharing"
        unique_id: "ts_402f1b21-5ad5-49d2-b451-2cf1e95cab65"
        icon: "mdi:projector-screen-off"
        state: "{{ None }}"
      - name: "Teams Unread Messages"
        unique_id: "ts_61500ecd-5f28-4be4-912d-a64f306fa0cc"
        icon: "mdi:message-off"
        state: "{{ None }}"
```

- The `name` and `friendly_name` should match what you have in the config file
- The `unique_id` can be any unique identifier

# Notices

- Pull Requests, Issues, Feature Requests are all welcomed
- This integration only supports the New Teams
- Logging is done in output.log, and rolls over at 10mb, keeping a maximum of two files
- Passwords and keys are encrypted

# Example Data

### API Connection Prior to Getting Token

```
ws://localhost:8124?protocol-version=2.0.0&manufacturer=AntoineGS&device=HomeAssistant&app=MS-Teams-Websocket&app-version=1.0
```

### API Connection With Token

```
ws://localhost:8124?token=FDUHINFHUSIDHNFSDFUIDSFHNUDSI&protocol-version=2.0.0&manufacturer=AntoineGS&device=HomeAssistant&app=MS-Teams-Websocket&app-version=1.0
```

### Teams -> Client Update

```json
{
  "meetingUpdate": {
    "meetingState": {
      "isMuted": false,
      "isVideoOn": false,
      "isHandRaised": false,
      "isInMeeting": true,
      "isRecordingOn": false,
      "isBackgroundBlurred": false,
      "isSharing": false,
      "hasUnreadMessages": false
    },
    "meetingPermissions": {
      "canToggleMute": true,
      "canToggleVideo": true,
      "canToggleHand": true,
      "canToggleBlur": false,
      "canLeave": true,
      "canReact": true,
      "canToggleShareTray": true,
      "canToggleChat": true,
      "canStopSharing": false,
      "canPair": false
    }
  }
}
```

### Client -> Teams Request Toggle Mute

```json
{
  "requestId": 1,
  "apiVersion": "2.0.0",
  "action": "toggle-mute"
}
```

### Teams -> Client Request Confirmation

```json
{
  "requestId": 2,
  "response": "Success"
}
```

### Client -> Teams Token Refresh

```json
{
  "tokenRefresh": "529547bd-9f11-4a83-9204-0e655b00fd5e"
}
```

### MQTT

```json
{
  "in_meeting": "on",
  "video_on": "off"
}
```

### Reference Document (for legacy Teams)

https://lostdomain.notion.site/Microsoft-Teams-WebSocket-API-5c042838bc3e4731bdfe679e864ab52a
