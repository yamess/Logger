use std::time::Duration;
use actix_web::http::{StatusCode, Version};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Root};
use log::LevelFilter;
use log;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::filter::threshold::ThresholdFilter;
use crate::utils::rolling_file_appender::rolling_file_appender;

#[derive(Debug)]
pub struct RequestLogger {
    pub path: String,
    pub method: String,
    pub ip: String,
    pub version: Version,
    pub user_agent: String,
    pub status_code: StatusCode,
}
#[derive(Debug)]
pub struct RequestLoggerBuilder {
    path: String,
    method: String,
    ip: String,
    version: Version,
    user_agent: Option<String>,
    status_code: StatusCode,
}

impl RequestLoggerBuilder {
    pub fn new() -> Self {
        RequestLoggerBuilder {
            path: "".to_string(),
            method: "".to_string(),
            ip: "".to_string(),
            version: Version::HTTP_11,
            user_agent: None,
            status_code: StatusCode::OK,
        }
    }

    pub fn path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    pub fn method(mut self, method: String) -> Self {
        self.method = method;
        self
    }

    pub fn ip(mut self, ip: String) -> Self {
        self.ip = ip;
        self
    }

    pub fn version(mut self, version: Version) -> Self {
        self.version = version;
        self
    }

    pub fn user_agent(mut self, user_agent: Option<String>) -> Self {
        self.user_agent = user_agent;
        self
    }


    pub fn build(self) -> RequestLogger {
        RequestLogger {
            path: self.path,
            method: self.method,
            ip: self.ip,
            version: self.version,
            user_agent: self.user_agent.unwrap_or("Unknown".to_string()),
            status_code: self.status_code,
        }
    }
}


impl RequestLogger {
    pub fn log(&self, duration: Option<Duration>) {
        let duration = duration.unwrap_or(Duration::from_secs_f64(0.0)).as_secs_f64();
        let format = format!(
            "{} {} {} {:?} {} {:.5}s {}",
            self.method, self.path, self.ip, self.version, self.status_code, duration,
            self.user_agent
        );
        match self.status_code.as_u16() {
            0..=299 => log::info!("{format}"),
            300..=399 => log::warn!("{format}"),
            _ => log::error!("{format}"),
        }
    }
}

pub fn init_logger() {

    let pattern = PatternEncoder::new(
        "[{X(request_id)(Internal-Request):<16}] - [{d(%Y-%m-%dT%H:%M:%S)(utc)} - {h({l}):<5.5} - \
        {T} - {f}:{L} - {M}]: {m}{n}"
    );

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(pattern.clone()))
        .build();

    let rolling_file = rolling_file_appender();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Trace)))
                .build("rolling_file", Box::new(rolling_file))
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("rolling_file")
                .build(LevelFilter::Debug)
        )
        .unwrap();
    log4rs::init_config(config).unwrap();
}
