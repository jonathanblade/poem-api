use std::time::Instant;

use poem::{Endpoint, IntoResponse, Middleware, Request, Response, Result};

use crate::common::AppError;

pub struct ErrorMiddleware;

impl<E: Endpoint> Middleware<E> for ErrorMiddleware {
    type Output = ErrorMiddlewareImpl<E>;

    fn transform(&self, ep: E) -> Self::Output {
        ErrorMiddlewareImpl { ep }
    }
}

pub struct ErrorMiddlewareImpl<E> {
    ep: E,
}

#[poem::async_trait]
impl<E: Endpoint> Endpoint for ErrorMiddlewareImpl<E> {
    type Output = Response;

    async fn call(&self, req: Request) -> Result<Self::Output> {
        let method = req.method().clone();
        let uri = req.uri().clone();
        let start = Instant::now();
        let resp = self.ep.call(req).await;
        let elapsed = start.elapsed();
        match resp {
            Ok(resp) => {
                let resp = resp.into_response();
                println!(
                    "{} -> {} {} - {} ms",
                    method,
                    uri,
                    resp.status().as_u16(),
                    elapsed.as_millis()
                );
                Ok(resp)
            }
            Err(e) => {
                if e.is::<poem::error::NotFoundError>() {
                    return Ok(AppError::ResourceNotFound(uri.to_string()).into_response());
                }
                if e.is::<poem::error::MethodNotAllowedError>() {
                    return Ok(AppError::MethodNotAllowed.into_response());
                }
                if e.is::<poem_openapi::error::ParseParamError>() {
                    return Ok(AppError::MalformedRequestParam.into_response());
                }
                if e.is::<poem_openapi::error::ParseRequestPayloadError>() {
                    return Ok(AppError::MalformedRequestPayload.into_response());
                }
                if e.is::<poem_openapi::error::ContentTypeError>() {
                    match e
                        .downcast::<poem_openapi::error::ContentTypeError>()
                        .unwrap()
                    {
                        poem_openapi::error::ContentTypeError::ExpectContentType => {
                            return Ok(AppError::MissingContentType.into_response());
                        }
                        poem_openapi::error::ContentTypeError::NotSupported { content_type } => {
                            return Ok(
                                AppError::UnsupportedContentType(content_type).into_response()
                            );
                        }
                    }
                }
                Ok(e.into_response())
            }
        }
    }
}
