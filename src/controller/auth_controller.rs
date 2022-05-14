use poem::web::Data;
use poem_openapi::{payload::Json, OpenApi};
use sqlx::SqlitePool;

use super::Tag;
use crate::response::{PostResponseError, PostResponseSuccess};
use crate::scheme::{AccessToken, Credentials};
use crate::service::auth_service::AuthService;

pub struct AuthController;

#[OpenApi(prefix_path = "/auth", tag = "Tag::Auth")]
impl AuthController {
    /// Create new access token.
    #[oai(path = "/token", method = "post")]
    async fn sign_in(
        &self,
        pool: Data<&SqlitePool>,
        credentials: Json<Credentials>,
    ) -> Result<PostResponseSuccess<AccessToken>, PostResponseError> {
        let token = AuthService::sign_in(pool.0, credentials.0).await?;
        let resp = PostResponseSuccess::new(token);
        Ok(resp)
    }
}
