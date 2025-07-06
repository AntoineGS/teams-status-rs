use log::LevelFilter;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::config::{Appender, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Config;

pub fn initialize_logging() {
    use std::env;
    let log_dir = match env::var("LOCALAPPDATA") {
        Ok(localappdata) => format!("{}\\teams-status-rs", localappdata),
        Err(_) => ".".to_string(),
    };
    let _ = std::fs::create_dir_all(&log_dir);
    let log_path = format!("{}\\output.log", log_dir);
    let old_log_path = format!("{}\\output_old{{}}.log", log_dir);

    let fixed_window_roller = FixedWindowRoller::builder()
        .build(&old_log_path, 1)
        .unwrap();
    let size_limit = 10 * 1024 * 1024;
    let size_trigger = SizeTrigger::new(size_limit);
    let compound_policy =
        CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));

    let logfile = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d:<36} {l} {t} - {m}{n}")))
        .build(&log_path, Box::new(compound_policy))
        .unwrap();

    let log_config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder().appender("logfile").build(LevelFilter::Info))
        .unwrap();

    log4rs::init_config(log_config).unwrap();
    log_panics::init();
}
