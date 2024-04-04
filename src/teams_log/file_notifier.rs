// todo: Use notify crate to know if there are changes to THAT file, and then run the seek/file parser
// todo: Use notify crate to watch folder and pickup any new log file changes

// todo: notifier on existing file, file changes, get notification
// todo: new file in folder, get notification

use notify::{Config, Event, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::PathBuf;
use tokio::sync::mpsc::{channel, Receiver};

pub struct FileNotifier {
    watcher: RecommendedWatcher,
    pub rx: Receiver<notify::Result<Event>>,
}

impl FileNotifier {
    pub fn new() -> anyhow::Result<Self> {
        let (tx, rx) = channel(10);

        let watcher = RecommendedWatcher::new(
            move |res| {
                tx.blocking_send(res).unwrap();
            }, Config::default())?;

        Ok(Self { watcher, rx })
    }

    pub fn watch(&mut self, path: Option<&PathBuf>) -> anyhow::Result<()> {
        if path.is_none() {
            return Ok(());
        }

        let _ = &self.watcher
            .watch(
                path.unwrap(),
                RecursiveMode::Recursive,
            )?;

        Ok(())
    }

    pub fn unwatch(&mut self, path: Option<&PathBuf>) -> anyhow::Result<()> {
        if path.is_none() {
            return Ok(());
        }

        let _ = &self.watcher
            .unwatch(
                path.unwrap(),
            )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::teams_log::file_notifier::FileNotifier;
    use std::env::current_dir;
    use std::{fs};
    use std::time::Duration;
    use tokio::time::timeout;

    const TEST_PATH: &str = "tests";

    #[tokio::test]
    async fn watch_folder_edit_file_will_notify() {
        let folder_path = current_dir().unwrap().join(TEST_PATH);
        let mut file_notifier = FileNotifier::new().unwrap();
        // ensure not to name the watcher "_" as it will get destroyed right away
        file_notifier.watch(Some(&folder_path)).unwrap();
        fs::write("C:\\Gits\\teams-status-rs\\tests\\log.txt", "test").unwrap();

        let res = file_notifier.rx.recv().await.unwrap();
        assert!(res.is_ok());
    }

    // todo: use a different folder for each test as this one fails when run alongside the others
    #[tokio::test]
    async fn watch_folder_do_nothing_will_not_notify() {
        let folder_path = current_dir().unwrap().join(TEST_PATH);
        let mut file_notifier = FileNotifier::new().unwrap();
        // ensure not to name the watcher "_" as it will get destroyed right away
        file_notifier.watch(Some(&folder_path)).unwrap();

        let res = timeout(Duration::new(1, 0), file_notifier.rx.recv()).await;
        assert!(res.is_err());
    }

    #[tokio::test]
    async fn watch_file_edit_file_will_notify() {
        let file_path = current_dir().unwrap().join(TEST_PATH).join("log.txt");
        let mut file_notifier = FileNotifier::new().unwrap();
        // ensure not to name the watcher "_" as it will get destroyed right away
        file_notifier.watch(Some(&file_path)).unwrap();
        fs::write("C:\\Gits\\teams-status-rs\\tests\\log.txt", "test").unwrap();

        let res = file_notifier.rx.recv().await.unwrap();
        assert!(res.is_ok());
    }
}
