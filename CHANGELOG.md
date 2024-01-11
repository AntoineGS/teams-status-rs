# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0](https://github.com/AntoineGS/teams-status-rs/releases/tag/v0.3.0) - 2024-01-11

### Added
- addition of new entities (all that are in the Teams API) for both HA and MQTT
- retain mqtt messages

### Fixed
- prevent application from crashing if Teams is closed while running
- fix wrong value used for video boolean, ensure there is an initial update when opening the app

### Other
- ignore exe
- add release-plz configuration
- remove unused CICD
- bump version to 0.3.0
- fix warning
- increase versioning
- remove unused error unit
- update dependencies
- Update tokio-tungstenite requirement from 0.20.1 to 0.21.0
- Up version, add build profile for unoptimized
- Fix compile error from missing explicit declaration
- Move logging out of main unit
- Refactor Listener creation now that I understand Boxes
- bump version to 0.2.1
- Fix incorrect Icon Id crashing the app upon startup
- Change Windows resource compiler, which seems to also fix the app from being flagged as virus 🤷‍♀️
- Update readme
- Remove default mqtt username
- Update doc
- mqtt support, retry mechanism, additional logging, ensure log rolling
- Restructuring to facilitate maintenance and addition of mqtt
- Update readme.md: fix connection string data type
- Update readme.md with connection strings
- Some level of encryption for keys, see notes in utils.rs for comments on a better implementation
- remove env file remnants
- Change icon color to slightly improve visibility on dark theme
- Update todos
- Simplify call
- Update Readme
- disable CI for now
- Put back teams token as it is used to persist granted API rights, save api token when received, more camera to video label conversion
- Change variables to match new naming conventions
- Migrate to New Teams, fix the stdin reading causing the command prompt (fail), allow muting through tray icon to trigger API registering, remove traces of token
- Move the dependabot.yml into its correct folder
- Fix typo in comment
- Rework todos
- Add dependapot
- conf.ini was not actually used, fix that, change default values and naming convention to fit new data
- ignore conf.ini
- Cleanup redundant clones and calls
- Add configuration, upgrade packages
- add/move Todos, add new config for later use
- Merge branch 'master' of https://github.com/AntoineGS/teams-status-rs
- Create rust.yml
- Remove linux configs
- Revert change that did not help, remove useless false return
- Ignore log file
- Managed to get the application from running outside of cargo, cleanup some comments, add logging
- Extract code from tray to support linux/windows builds, mostly for development, might remove later
- Allow application to be closed by user, fix up tray, extract util, cleanup uses
- Add code for testing on linux since tray is broken on it atm
- Add some todos
- Cleanup objects a bit, more functional
- Convert to tokio version of tungstenite to allow quitting the app where we used to have a blocking .read_messages
- Close works BUT only when a message comes in
- add icon (untested, wont build), give up on cancellation of process for now
- Add tray icon and exit logic
- First working draft, with a lot of hardcoded settings.
- First commit, basic api stubbing and tests done.
