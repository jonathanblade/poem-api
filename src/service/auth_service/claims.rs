use poem::{http::header::AUTHORIZATION, Request, RequestBody, Result};
use poem_openapi::{
    registry::{MetaSecurityScheme, Registry},
    ApiExtractor, ApiExtractorType, ExtractParamOptions,
};
use serde::{Deserialize, Serialize};

use super::AuthService;
use crate::common::AppError;

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub user_id: i32,
    pub is_superuser: i32,
    pub iat: i64,
    pub exp: i64,
}

#[poem::async_trait]
impl<'a> ApiExtractor<'a> for Claims {
    type ParamType = ();
    type ParamRawType = ();

    const TYPE: ApiExtractorType = ApiExtractorType::SecurityScheme;
    const PARAM_IS_REQUIRED: bool = true;

    async fn from_request(
        req: &'a Request,
        _: &mut RequestBody,
        _: ExtractParamOptions<Self::ParamType>,
    ) -> Result<Self> {
        let token = req
            .headers()
            .get(AUTHORIZATION)
            .ok_or(AppError::MissingAccessToken)?
            .to_str()
            .map_err(|_| AppError::InternalError)?
            .strip_prefix("Bearer ")
            .ok_or(AppError::MissingAccessToken)?;
        let claims = AuthService::verify_access_token(token)?;
        Ok(claims)
    }

    fn register(registry: &mut Registry) {
        registry.create_security_scheme(
            "Access token",
            MetaSecurityScheme {
                ty: "http",
                description: None,
                name: None,
                key_in: None,
                scheme: Some("bearer"),
                bearer_format: None,
                flows: None,
                openid_connect_url: None,
            },
        )
    }

    fn security_scheme() -> Option<&'static str> {
        Some("Access token")
    }
}

pub fn superuser_scope(claims: Claims) -> Result<(), AppError> {
    if claims.is_superuser == 0 {
        return Err(AppError::SuperuserScopeRequired);
    }
    Ok(())
}
