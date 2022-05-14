use poem_openapi::{
    types::{ParseFromJSON, ToJSON},
    Enum, Object,
};

use crate::common::AppError;

/// Response status scheme.
#[derive(Enum)]
#[oai(rename_all = "lowercase")]
pub enum ResponseStatus {
    Success,
    Error,
}

#[derive(Object)]
#[oai(inline)]
pub struct ResponseSuccess<T: ParseFromJSON + ToJSON + Send + Sync> {
    /// Response status.
    #[oai(default = "status_success")]
    pub status: ResponseStatus,
    /// Response result.
    pub result: Option<T>,
}

impl<T: ParseFromJSON + ToJSON + Send + Sync> ResponseSuccess<T> {
    pub fn new(result: T) -> Self {
        ResponseSuccess {
            status: status_success(),
            result: Some(result),
        }
    }
}

#[derive(Object)]
#[oai(inline)]
#[oai(example = "response_error_example")]
pub struct ResponseError {
    /// Response status.
    #[oai(default = "status_error")]
    pub status: ResponseStatus,
    /// Error reason.
    pub reason: String,
}

impl From<AppError> for ResponseError {
    fn from(e: AppError) -> Self {
        ResponseError {
            status: status_error(),
            reason: e.to_string(),
        }
    }
}

fn status_success() -> ResponseStatus {
    ResponseStatus::Success
}

fn status_error() -> ResponseStatus {
    ResponseStatus::Error
}

fn response_error_example() -> ResponseError {
    ResponseError {
        status: status_error(),
        reason: "Something went wrong.".to_string(),
    }
}
