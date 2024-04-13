
use std::future::{Ready, ready};
use std::str::FromStr;
use actix_web::dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::http::header;
use futures::future::LocalBoxFuture;
use crate::{generate_random_string, log_request};

pub struct CorrelationId;


impl<S, B> Transform<S, ServiceRequest> for CorrelationId
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error=actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Transform = CorrelationIdMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CorrelationIdMiddleware { service }))
    }
}

pub struct CorrelationIdMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for CorrelationIdMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error=actix_web::Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = actix_web::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let start = std::time::Instant::now();
        let request_id = generate_random_string(16);
        log_mdc::insert("request_id", request_id.to_string());
        let mut request_logger = log_request(&req);

        let fut = self.service.call(req);

        Box::pin(async move {
            let mut res = fut.await?;
            request_logger.status_code = res.status();
            let duration = start.elapsed();
            let elapsed_time = format!("{:.5}", duration.as_secs_f64());
            res.headers_mut().insert(
                header::HeaderName::from_str("X-Elapsed-Time").unwrap(),
                header::HeaderValue::from_str(&elapsed_time).unwrap(),
            );
            request_logger.log(Some(duration));
            log_mdc::remove("request_id");
            Ok(res)
        })
    }
}