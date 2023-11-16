# Configuration
- Run the application once, it will generate the ini file (conf.ini) in the same folder as the .exe
- In Microsoft Teams, enable the Third-Party API ([see Microsoft documentation](https://support.microsoft.com/en-us/office/connect-to-third-party-devices-in-microsoft-teams-aabca9f2-47bb-407f-9f9b-81a104a883d6?storagetype=live))
- In Home Assistant, generate a Long-Lived Access Token ([see HA documentation](https://developers.home-assistant.io/docs/auth_api/#long-lived-access-token))
  - Paste it into the conf.ini
- Run the application again
  - It will create the entities in HA automatically 
  - Start a meeting (you can be the only person in it)
  - From the tray icon, click on Toggle Mute
  - You will get a prompt in Teams to allow the application to use the API, if you do not click on time Teams will close the prompt. Simply click on Toggle Mute again.

# Notices
- This integration only supports the New Teams.
- There is currently no recovery process if the connection to Teams is unavailable upon launch or closed while it is active, you will need to restart the application (which will most likely have closed)

# Example JSONs from MS Teams
### API Connection Prior to Getting Token
```json
ws://localhost:8124?protocol-version=2.0.0&manufacturer=AntoineGS&device=HomeAssistant&app=MS-Teams-Websocket&app-version=1.0
```
### API Connection With Token
```json
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

### Reference Document (for legacy Teams)
https://lostdomain.notion.site/Microsoft-Teams-WebSocket-API-5c042838bc3e4731bdfe679e864ab52a
