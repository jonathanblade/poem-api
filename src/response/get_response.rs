use poem_openapi::{
    payload::Json,
    types::{ParseFromJSON, ToJSON},
    ApiResponse, Object,
};

use super::{ResponseError, ResponseSuccess};
use crate::common::AppError;

#[derive(ApiResponse)]
pub enum GetResponseSuccess<T: ParseFromJSON + ToJSON + Send + Sync> {
    /// Request completed successfully.
    #[oai(status = 200)]
    OK(Json<ResponseSuccess<T>>),
}

impl<T: ParseFromJSON + ToJSON + Send + Sync> GetResponseSuccess<T> {
    pub fn new(result: T) -> Self {
        GetResponseSuccess::OK(Json(ResponseSuccess::new(result)))
    }
}

#[derive(ApiResponse)]
pub enum GetResponseError {
    /// Bad request.
    #[oai(status = 400)]
    BadRequest(Json<ResponseError>),
    /// Authorization failed.
    #[oai(status = 401)]
    Unauthorized(Json<ResponseError>),
    /// Object not found.
    #[oai(status = 404)]
    NotFound(Json<ResponseError>),
    /// Internal server error.
    #[oai(status = 500)]
    InternalError(Json<ResponseError>),
}

impl From<AppError> for GetResponseError {
    fn from(e: AppError) -> Self {
        match e {
            // 400
            AppError::MethodNotAllowed
            | AppError::MissingContentType
            | AppError::UnsupportedContentType(_)
            | AppError::MalformedRequestPayload
            | AppError::MalformedRequestParam => GetResponseError::BadRequest(Json(e.into())),
            // 401
            AppError::InvalidAccessToken
            | AppError::MissingAccessToken
            | AppError::AccessTokenExpired
            | AppError::InvalidCredentials
            | AppError::SuperuserScopeRequired => GetResponseError::Unauthorized(Json(e.into())),
            // 404
            AppError::ObjectNotFound | AppError::ResourceNotFound(_) => {
                GetResponseError::NotFound(Json(e.into()))
            }
            // 500
            _ => GetResponseError::InternalError(Json(e.into())),
        }
    }
}

#[derive(Object)]
#[oai(inline)]
pub struct PaginatedResponse<T: ParseFromJSON + ToJSON + Send + Sync> {
    pub items: Vec<T>,
    pub offset: u32,
    pub total: u32,
}

#[derive(ApiResponse)]
pub enum GetListResponseSuccess<T: ParseFromJSON + ToJSON + Send + Sync> {
    /// Request completed successfully.
    #[oai(status = 200)]
    OK(Json<ResponseSuccess<PaginatedResponse<T>>>),
}

impl<T: ParseFromJSON + ToJSON + Send + Sync> GetListResponseSuccess<T> {
    pub fn new(items: Vec<T>, offset: u32, total: u32) -> Self {
        let result = PaginatedResponse {
            items,
            offset,
            total,
        };
        GetListResponseSuccess::OK(Json(ResponseSuccess::new(result)))
    }
}

#[derive(ApiResponse)]
pub enum GetListResponseError {
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

impl From<AppError> for GetListResponseError {
    fn from(e: AppError) -> Self {
        match e {
            // 400
            AppError::MethodNotAllowed
            | AppError::MissingContentType
            | AppError::UnsupportedContentType(_)
            | AppError::MalformedRequestPayload
            | AppError::MalformedRequestParam => GetListResponseError::BadRequest(Json(e.into())),
            // 401
            AppError::InvalidAccessToken
            | AppError::MissingAccessToken
            | AppError::AccessTokenExpired
            | AppError::InvalidCredentials
            | AppError::SuperuserScopeRequired => {
                GetListResponseError::Unauthorized(Json(e.into()))
            }
            // 500
            _ => GetListResponseError::InternalError(Json(e.into())),
        }
    }
}
