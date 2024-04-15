use actix_service::{Service, Transform, forward_ready};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error, http::header};
use futures::future::{ok, Ready, Future};
use std::pin::Pin;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,  // Ensure that the future is 'static 
    //This guarantees that any data referenced by the future will live for the duration of the program, 
    //or at least as long as the future exists.
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract and copy the token from the header before moving `req`
        let token = req.headers().get(header::AUTHORIZATION)
            .and_then(|hv| hv.to_str().ok())
            .and_then(|header| header.strip_prefix("Bearer "))
            .map(|token| token.to_string());

        let fut = self.service.call(req);

        Box::pin(async move {
            match token {
                Some(token) if validate_token(&token) => {
                    fut.await  // Continue with the request
                },
                _ => Err(actix_web::error::ErrorUnauthorized("Invalid or missing token")),
            }
        })
    }
}

fn validate_token(token: &str) -> bool {
    // Placeholder for token validation logic
    // This should check the validity of the token, e.g., decoding a JWT
    token == "expected_token"
}