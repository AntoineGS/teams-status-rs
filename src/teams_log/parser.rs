// parses the given file from position x to locate the
// todo: there is a lot of logic in the Teams.ps1 file regarding getting the status right

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::PathBuf;

/// Returns the availability as a string, as well as the last position parsed
pub fn get_last_state(path: Option<&PathBuf>, position: u64) -> anyhow::Result<(Option<String>, u64)> {
    if path.is_none() {
        return Ok((None, 0));
    }

    let mut f = File::open(path.unwrap()).unwrap();
    f.seek(SeekFrom::Start(position)).unwrap();
    let position = f.metadata()?.len();
    let mut last_status: Option<String> = None;

    let reader = BufReader::new(&f);
    let re = Regex::new(r"UserPresenceAction:.*availability: (?<status>[a-zA-Z]*)").unwrap();
    for line in reader.lines() {
        let log_line = line.unwrap();
        let result = re.captures(&log_line);

        if result.is_some() {
            let status = result.unwrap().name("status");
            if status.is_some() {
                last_status = Some(status.unwrap().as_str().to_owned());
            }
        }
    }

    Ok((last_status, position))
}

#[cfg(test)]
mod tests {
    use crate::teams_log::parser::get_last_state;
    use std::env::current_dir;

    const TEST_PATH: &str = "tests";

    #[test]
    fn get_last_state_test_file_will_return_correct_state() {
        let file_path = current_dir()
            .unwrap()
            .join(TEST_PATH)
            .join("MSTeams_2024-02-07_14-47-09.05.log");

        let (status, _) = get_last_state(Some(&file_path), 0).unwrap();
        assert_eq!("Available", status.unwrap());
    }

    #[test]
    fn get_last_state_test_file_position_further_than_last_update_will_last_position_and_none_status() {
        let file_path = current_dir()
            .unwrap()
            .join(TEST_PATH)
            .join("MSTeams_2024-02-07_14-47-09.05.log");

        let (status, position) = get_last_state(Some(&file_path), 1670531).unwrap();
        assert!(position > 1670531);
        assert_eq!(status, None);
    }

    #[test]
    fn get_last_state_test_file_returned_position_will_be_higher() {
        let file_path = current_dir()
            .unwrap()
            .join(TEST_PATH)
            .join("MSTeams_2024-02-07_14-47-09.05.log");

        let (_, position) = get_last_state(Some(&file_path), 0).unwrap();
        assert!(position > 0);
    }
}
