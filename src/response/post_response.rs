use poem_openapi::{
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse,
};

use super::{ResponseError, ResponseSuccess};
use crate::common::AppError;

#[derive(ApiResponse)]
pub enum PostResponseSuccess<T: ParseFromJSON + ToJSON + Send + Sync> {
    /// Request completed successfully.
    #[oai(status = 201)]
    Created(Json<ResponseSuccess<T>>),
}

impl<T: ParseFromJSON + ToJSON + Send + Sync> PostResponseSuccess<T> {
    pub fn new(result: T) -> Self {
        PostResponseSuccess::Created(Json(ResponseSuccess::new(result)))
    }
}

#[derive(ApiResponse)]
pub enum PostResponseError {
    /// Bad request.
    #[oai(status = 400)]
    BadRequest(Json<ResponseError>),
    /// Authorization failed.
    #[oai(status = 401)]
    Unauthorized(Json<ResponseError>),
    /// Internal server error.
    #[oai(status = 500)]
    InternalError(Json<ResponseError>),
}

impl From<AppError> for PostResponseError {
    fn from(e: AppError) -> Self {
        match e {
            // 400
            AppError::MethodNotAllowed
            | AppError::MissingContentType
            | AppError::UnsupportedContentType(_)
            | AppError::MalformedRequestPayload
            | AppError::MalformedRequestParam => PostResponseError::BadRequest(Json(e.into())),
            // 401
            AppError::InvalidAccessToken
            | AppError::MissingAccessToken
            | AppError::AccessTokenExpired
            | AppError::InvalidCredentials
            | AppError::SuperuserScopeRequired => PostResponseError::Unauthorized(Json(e.into())),
            // 500
            _ => PostResponseError::InternalError(Json(e.into())),
        }
    }
}
