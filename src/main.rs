use log;
use log::{Metadata, Record};
use actix_web::{self, web, App, HttpServer, HttpResponse, Responder, middleware::Logger};
use uuid::Uuid;
use actix_web::dev::Service;
use rand::distributions::{Alphanumeric, Distribution};
use rand::Rng;
mod logger;

#[actix_web::get("/health")]
async fn health() -> actix_web::Result<impl Responder> {
    log::info!("Info Health check");
    log::error!("Error Health check");
    log::warn!("Warn Health check");
    Ok(HttpResponse::Ok().json("Ok"))
}

pub fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    logger::init_logger();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            // .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap_fn(|req, srv| {
                let request_id =  generate_random_string(10);
                log_mdc::insert("request_id", request_id.to_string());
                let fut = srv.call(req);
                async move {
                    let res = fut.await;
                    log_mdc::remove("request_id");
                    res
                }
            })
            .service(health)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
