use poem::web::Data;
use poem_openapi::{payload::Json, OpenApi};

use super::Tag;
use crate::context::AppContext;
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
        ctx: Data<&AppContext>,
        credentials: Json<Credentials>,
    ) -> Result<PostResponseSuccess<AccessToken>, PostResponseError> {
        let token = AuthService::sign_in(&ctx.db_pool, credentials.0).await?;
        let resp = PostResponseSuccess::new(token);
        Ok(resp)
    }
}
