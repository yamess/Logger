use log;

use actix_web::{self, App, HttpServer, HttpResponse, Responder, middleware};

use actix_web::dev::{Service, ServiceResponse, Transform};
use rand::distributions::{Alphanumeric, Distribution};
use rand::Rng;
mod utils;
mod middlewares;

use actix_web::middleware::Logger;
use log::Record;
use actix_web::dev::ServiceRequest;
use futures::future::{ok, Ready};
use std::task::{Context, Poll};
use std::time::Duration;
use actix_web::http::{StatusCode, Version};
use crate::middlewares::{CorrelationId, CorrelationIdMiddleware};
use crate::utils::logger;
use crate::utils::logger::{RequestLogger, RequestLoggerBuilder};


#[actix_web::get("/health")]
async fn health() -> actix_web::Result<impl Responder> {
    log::info!("Info Health check");
    Ok(HttpResponse::Ok().body("Ok"))
}

pub fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

pub fn log_request(req: &ServiceRequest) -> RequestLogger {
    let request_logger = RequestLoggerBuilder::new()
        .path(req.path().to_string())
        .method(req.method().to_string())
        .ip(req.connection_info().realip_remote_addr().unwrap().to_string())
        .version(req.version())
        .user_agent(req.headers().get("User-Agent").and_then(|v| v.to_str().ok()).map(|v| v
            .to_string()))
        .build();
    request_logger
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init_logger();

    HttpServer::new(|| {

        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(CorrelationId)
            .service(health)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
