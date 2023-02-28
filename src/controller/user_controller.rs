use poem::web::Data;
use poem_openapi::{
    param::{Path, Query},
    payload::Json,
    OpenApi,
};

use super::Tag;
use crate::response::{
    GetListResponseError, GetListResponseSuccess, GetResponseError, GetResponseSuccess,
    PostResponseError, PostResponseSuccess,
};
use crate::scheme::{CreateUser, InsertUser, User};
use crate::service::auth_service::{superuser_scope, AuthService, Claims};
use crate::{context::AppContext, db::repo::UserRepo};

pub struct UserController;

#[OpenApi(prefix_path = "/user", tag = "Tag::User")]
impl UserController {
    /// Get users list.
    #[oai(path = "/", method = "get")]
    async fn get_users_list(
        &self,
        ctx: Data<&AppContext>,
        limit: Query<Option<u32>>,
        offset: Query<Option<u32>>,
        claims: Claims,
    ) -> Result<GetListResponseSuccess<User>, GetListResponseError> {
        superuser_scope(claims)?;
        let users = UserRepo::get_users_list(&ctx.db_pool, limit.0, offset.0).await?;
        let resp = GetListResponseSuccess::new(users.0, users.1, users.2);
        Ok(resp)
    }

    /// Get user by ID.
    #[oai(path = "/:id", method = "get")]
    async fn get_user_by_id(
        &self,
        ctx: Data<&AppContext>,
        id: Path<i32>,
        claims: Claims,
    ) -> Result<GetResponseSuccess<User>, GetResponseError> {
        superuser_scope(claims)?;
        let user = UserRepo::get_user_by_id(&ctx.db_pool, id.0).await?;
        let resp = GetResponseSuccess::new(user);
        Ok(resp)
    }

    /// Create new user.
    #[oai(path = "/", method = "post")]
    async fn create_user(
        &self,
        ctx: Data<&AppContext>,
        user: Json<CreateUser>,
        claims: Claims,
    ) -> Result<PostResponseSuccess<User>, PostResponseError> {
        superuser_scope(claims)?;
        let insert_user = InsertUser {
            username: user.0.username,
            password_hash: AuthService::hash_password(&user.0.password)?,
            is_superuser: 0,
        };
        let user = UserRepo::insert_user(&ctx.db_pool, insert_user).await?;
        let resp = PostResponseSuccess::new(user);
        Ok(resp)
    }
}
