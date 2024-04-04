use anyhow::{Context};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub const TEAMS_PREFIX: &str = "MSTeams_";
const TEAMS_SUFFIX: &str = ".log";

// todo: When Teams is unreachable, leave the HA variables as Off
// note: May want to move to https://github.com/uutils/coreutils/tree/main/src/uu/tail at some point

pub fn locate_latest_log(path: &Path) -> anyhow::Result<Option<PathBuf>> {
    let mut entries = fs::read_dir(path)
        .context("Teams log path is invalid")?
        .filter(|entry| {
            let dir_entry = entry.as_ref();
            if dir_entry.is_ok() {
                let str = dir_entry
                    .unwrap()
                    .file_name()
                    .to_str()
                    .unwrap_or("")
                    .to_owned();

                str.starts_with(TEAMS_PREFIX) && str.ends_with(TEAMS_SUFFIX)
            } else {
                false
            }
        })
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();
    let mut latest_file_name = "";

    if entries.last().is_some() {
        let last_entry = entries.last().unwrap();

        if last_entry.to_str().is_some() {
            latest_file_name = last_entry.to_str().unwrap();
        }
    }

    if latest_file_name != "" {
        return Ok(Some(PathBuf::from(latest_file_name)));
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use crate::teams_log::file_locator::{locate_latest_log, TEAMS_PREFIX};
    use chrono::{Datelike, Local};
    use std::env::current_dir;
    use std::fs;
    use std::path::PathBuf;

    const TEST_PATH: &str = "tests\\file_locator";

    fn create_file(path: &str, file_name: String) -> PathBuf {
        let file_path = current_dir().unwrap().join(path).join(file_name);
        fs::write(file_path.to_str().unwrap(), "").unwrap();
        file_path
    }

    fn cleanup_test_folder(path: &str) {
        fs::remove_dir_all(path).ok();
        fs::create_dir_all(path).unwrap();
    }

    #[test]
    // Mimics the naming convention of the Teams log file
    fn locate_latest_log_one_file_will_return_correct() {
        let current_test_path = TEST_PATH.to_owned() + "/test1";
        cleanup_test_folder(&current_test_path);
        let now = Local::now();
        let file_path = create_file(
            &current_test_path,
            format!(
                "{}_{}-{:02}-{:02}_12-18-41.01.log",
                TEAMS_PREFIX,
                now.year(),
                now.month(),
                now.day()
            ),
        );
        let found_file_path = locate_latest_log(&file_path.parent().unwrap());
        assert_eq!(found_file_path.unwrap().unwrap(), file_path.as_path());
    }

    #[test]
    // Mimics the naming convention of the Teams log file
    fn locate_latest_log_three_files_same_day_will_return_correct() {
        let current_test_path = TEST_PATH.to_owned() + "/test2";
        cleanup_test_folder(&current_test_path);
        let now = Local::now();

        create_file(
            &current_test_path,
            format!(
                "{}_{}-{:02}-{:02}_12-17-41.01.log",
                TEAMS_PREFIX,
                now.year(),
                now.month(),
                now.day()
            ),
        );
        let file_path = create_file(
            &current_test_path,
            format!(
                "{}_{}-{:02}-{:02}_14-17-41.00.log",
                TEAMS_PREFIX,
                now.year(),
                now.month(),
                now.day()
            ),
        );
        create_file(
            &current_test_path,
            format!(
                "{}_{}-{:02}-{:02}_14-16-41.00.log",
                TEAMS_PREFIX,
                now.year(),
                now.month(),
                now.day()
            ),
        );
        create_file(
            &current_test_path,
            format!(
                "{}_{}-{:02}-{:02}_13-19-41.02.log",
                TEAMS_PREFIX,
                now.year(),
                now.month(),
                now.day()
            ),
        );

        let found_file_path = locate_latest_log(&file_path.parent().unwrap());
        assert_eq!(found_file_path.unwrap().unwrap(), file_path.as_path());
    }
}
