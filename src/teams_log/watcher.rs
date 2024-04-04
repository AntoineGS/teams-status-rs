use std::fs;
use std::path::PathBuf;
use crate::teams_log::file_locator::{locate_latest_log, TEAMS_PREFIX};
use crate::teams_log::file_notifier::FileNotifier;
use crate::teams_log::parser::get_last_state;

// higher-level unit, will coordinate between locator, notifier and parser
pub struct Watcher {
    teams_base_path: PathBuf,
    file_notifier: FileNotifier,
}

impl Watcher {
    pub fn new() -> Self {
        let file_notifier = FileNotifier::new().unwrap();
        let mut app_data = std::env::var("APPDATA").unwrap();
        app_data = app_data.replace("\\Roaming", "");
        let teams_base_path: PathBuf = [r"C:\", &app_data, "Local", "Packages", "MSTeams_8wekyb3d8bbwe", "LocalCache", "Microsoft", "MSTeams", "Logs"].iter().collect();
        Self { teams_base_path, file_notifier }
    }

    pub async fn watch_teams_files(&mut self) -> anyhow::Result<()> {
        let mut latest_log_file = locate_latest_log(&self.teams_base_path)?;

        let (mut last_state, mut file_position) = get_last_state(latest_log_file.as_ref(), 0).unwrap();
        // todo: call API here
        fs::write(r"C:\Users\antoi\Documents\teams_log.txt", last_state.unwrap_or("None".to_string())).unwrap();

        self.file_notifier.watch(latest_log_file.as_ref())?;
        self.file_notifier.watch(Some(&self.teams_base_path))?;

        while let Some(res) = self.file_notifier.rx.recv().await {
            match res {
                Ok(res) => {
                    if (res.paths.len() == 0) || res.paths.first().is_none() {
                        continue;
                    }

                    let changed_file = res.paths.first().unwrap();

                    if changed_file.file_name().is_none() || changed_file.file_name().unwrap().to_str().is_none() {
                        continue;
                    }

                    if changed_file.file_name().unwrap().to_str().unwrap().starts_with(TEAMS_PREFIX) {
                        // Is it a different file then the latest log file?
                        if Some(changed_file) != latest_log_file.as_ref() {
                            let new_latest_log_file = locate_latest_log(&self.teams_base_path)?;

                            if new_latest_log_file != latest_log_file {
                                self.file_notifier.unwatch(latest_log_file.as_ref())?;
                                // latest_log_file = res.paths.first()?.into_path_buf();
                                latest_log_file = new_latest_log_file;
                                self.file_notifier.watch(latest_log_file.as_ref())?;
                                file_position = 0;
                            }
                        }

                        let (new_last_state, mew_file_position) = get_last_state(latest_log_file.as_ref(), file_position).unwrap();
                        last_state = new_last_state;
                        file_position = mew_file_position;

                        // todo: we will be calling API here
                        fs::write(r"C:\Users\antoi\Documents\teams_log.txt", last_state.clone().unwrap_or("None".to_string())).unwrap();
                        if last_state.is_some() {
                            println!("{}", last_state.clone().unwrap());
                        }
                    }
                }
                // todo: not sure I should exit here
                Err(res) => break,
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::teams_log::watcher::Watcher;

    #[tokio::test]
    async fn end_to_end_test() {
        let mut watcher = Watcher::new();
        watcher.watch_teams_files().await.unwrap();
    }
}