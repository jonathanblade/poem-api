use bcrypt::BcryptError;
use jsonwebtoken::errors::Error as JwtError;
use poem::{
    error::ResponseError, http::header::CONTENT_TYPE, http::StatusCode, Body, IntoResponse,
    Response,
};
use serde_json::json;
use sqlx::Error as SqlxError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum AppError {
    // 400
    #[error("Method not allowed.")]
    MethodNotAllowed,
    #[error("Missing request content type.")]
    MissingContentType,
    #[error("Request content type `{0}` unsupported.")]
    UnsupportedContentType(String),
    #[error("Malformed request payload.")]
    MalformedRequestPayload,
    #[error("Malformed request parameter.")]
    MalformedRequestParam,
    // 401
    #[error("Invalid access token.")]
    InvalidAccessToken,
    #[error("Missing access token.")]
    MissingAccessToken,
    #[error("Access token expired.")]
    AccessTokenExpired,
    #[error("Invalid credentials.")]
    InvalidCredentials,
    #[error("Superuser scope required.")]
    SuperuserScopeRequired,
    // 404
    #[error("Object not found.")]
    ObjectNotFound,
    #[error("Resource at `{0}` not found.")]
    ResourceNotFound(String),
    // 500
    #[error("Object already exists.")]
    ObjectAlreadyExists,
    #[error("Database returned error code: {0}.")]
    DatabaseError(String),
    #[error("Internal server error.")]
    InternalError,
    #[error("Database connection pool timed out.")]
    PoolTimedOut,
}

impl From<SqlxError> for AppError {
    fn from(e: SqlxError) -> Self {
        match e {
            SqlxError::PoolTimedOut => AppError::PoolTimedOut,
            SqlxError::RowNotFound => AppError::ObjectNotFound,
            SqlxError::Database(e) => match e.code() {
                Some(code) => {
                    if code == "23505" {
                        return AppError::ObjectAlreadyExists;
                    }
                    AppError::DatabaseError(code.to_string())
                }
                None => AppError::InternalError,
            },
            _ => AppError::InternalError,
        }
    }
}

impl From<BcryptError> for AppError {
    fn from(_: BcryptError) -> Self {
        AppError::InternalError
    }
}

impl From<JwtError> for AppError {
    fn from(_: JwtError) -> Self {
        AppError::InvalidAccessToken
    }
}

impl ResponseError for AppError {
    fn status(&self) -> StatusCode {
        match self {
            // 400
            AppError::MethodNotAllowed
            | AppError::MissingContentType
            | AppError::UnsupportedContentType(_)
            | AppError::MalformedRequestPayload
            | AppError::MalformedRequestParam => StatusCode::BAD_REQUEST,
            // 401
            AppError::InvalidAccessToken
            | AppError::MissingAccessToken
            | AppError::AccessTokenExpired
            | AppError::InvalidCredentials
            | AppError::SuperuserScopeRequired => StatusCode::UNAUTHORIZED,
            // 404
            AppError::ObjectNotFound | AppError::ResourceNotFound(_) => StatusCode::NOT_FOUND,
            // 500
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn as_response(&self) -> Response {
        build_response_error(self.status(), self.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        build_response_error(self.status(), self.to_string())
    }
}

fn build_response_error(status: StatusCode, reason: String) -> Response {
    let body = Body::from_json(json!({
        "status": "error".to_string(),
        "reason": reason,
    }))
    .unwrap();
    Response::builder()
        .header(CONTENT_TYPE, "application/json; charset=utf-8")
        .status(status)
        .body(body)
}
