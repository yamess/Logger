
use log4rs::{
    append::rolling_file::policy::compound::trigger::size::SizeTrigger,
    append::rolling_file::RollingFileAppender,
    encode::json::JsonEncoder,
    encode::pattern::PatternEncoder,
};
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;


const TRIGGER_FILE_SIZE: u64 = 1024 * 1024 * 1;  // 1MB
const ROLLING_FILE_PATH: &str = "/tmp/logs/output.json";
const LOG_FILE_COUNT: u32 = 100;
const ARCHIVE_FILE_PATTERN: &str = "/tmp/logs/archives/output.{}.json";


pub fn rolling_file_appender() -> RollingFileAppender {
    let _pattern = PatternEncoder::new(
        "[{X(request_id)(Internal):<10}] - [{d(%Y-%m-%dT%H:%M:%S)(utc)} - {h({l}):<5.5} - \
        {T} - {f}:{L} - {M}]: {m}{n}"
    );

    let size_trigger = SizeTrigger::new(TRIGGER_FILE_SIZE);
    // let time_trigger = TimeTrigger::new(TimeTriggerConfig::new(TimeTriggerInterval::Second(60)));
    let roller = FixedWindowRoller::builder()
        .base(0)
        .build(ARCHIVE_FILE_PATTERN, LOG_FILE_COUNT)
        .unwrap();
    let policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(roller));

    let log_file = RollingFileAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build(ROLLING_FILE_PATH, Box::new(policy))
        .unwrap();
    log_file
}