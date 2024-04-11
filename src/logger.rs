use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log::LevelFilter;
use log4rs::append::file::FileAppender;

use log;

use log4rs::encode::pattern::PatternEncoder;
pub fn init_logger() {

    let stdout = ConsoleAppender::builder()
        .encoder(
            Box::new(
                PatternEncoder::new("[{X(request_id)(Unknown):<10}] {d(%Y-%m-%dT%H:%M:%S)(utc)} \
                [{I}] \
                [{T}] \
                [{f}:{L}] [{h({l}):<5.5}] \
                {M}:{m}{n}")))
        .build();
    // let requests = FileAppender::builder()
    //     .encoder(Box::new(PatternEncoder::new("[{d(%Y-%m-%dT%H:%M:%S%.6f)} {h({l}):<5.5} {M}] \
    //     {m}{n}")))
    //     .build("log/requests.log")
    //     .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        // .appender(Appender::builder().build("requests", Box::new(requests)))
        .build(Root::builder().appender("stdout").build
        (LevelFilter::Info))
        .unwrap();
    log4rs::init_config(config).unwrap();
}