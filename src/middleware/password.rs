use std::future::{ready, Ready};
use actix_http::body::EitherBody;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse
};
use futures_util::future::LocalBoxFuture;

use crate::managers;

pub struct PasswordMiddleware;

impl<S, B> Transform<S, ServiceRequest> for PasswordMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = PasswordSetup<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PasswordSetup { service }))
    }
}

pub struct PasswordSetup<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for PasswordSetup<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth = req.headers().get("Authorization");
        let auth_check: &str;
        let auth_status: bool;

        unsafe {
            match auth {
                Some(val) => {
                    auth_check = val.to_str().unwrap();
                    if auth_check == managers::data_manager::SETTINGS.clone().unwrap().get("password").unwrap().as_str().unwrap() {
                        auth_status = true;
                    } else {
                        auth_status = false;
                    }
                }
                None => {
                    auth_status = false;
                }
            }
        }

        if !auth_status {
            let resp: HttpResponse<EitherBody<B>> = HttpResponse::BadRequest().body("Incorrect Password").map_into_right_body();
            let (request, _pl) = req.into_parts();
            return Box::pin( async {Ok(ServiceResponse::new(request, resp))});
        }
        

        let fut = self.service.call(req);

        Box::pin(async move {
            fut.await.map(ServiceResponse::map_into_left_body)
        })
    }
}