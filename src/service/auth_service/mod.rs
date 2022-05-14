mod claims;

pub use claims::{superuser_scope, Claims};

use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sqlx::SqlitePool;

use crate::common::AppError;
use crate::db::repo::UserRepo;
use crate::scheme::{AccessToken, Credentials};

pub struct AuthService;

impl AuthService {
    pub fn create_access_token(user_id: i32, is_superuser: i32) -> Result<AccessToken, AppError> {
        let iat = Utc::now();
        let exp = iat + Duration::seconds(3600);
        let iat = iat.timestamp_millis();
        let exp = exp.timestamp_millis();
        let claims = Claims {
            user_id,
            is_superuser,
            iat,
            exp,
        };
        let key = EncodingKey::from_secret("secret".as_ref());
        let header = Header::new(Algorithm::HS256);
        let token = encode(&header, &claims, &key)?;
        Ok(AccessToken {
            token,
            token_type: "Bearer".to_string(),
            issued_at: iat,
            expired_in: exp,
        })
    }

    pub fn verify_access_token(token: &str) -> Result<Claims, AppError> {
        let key = DecodingKey::from_secret("secret".as_ref());
        let validation = Validation::new(Algorithm::HS256);
        let data = decode::<Claims>(token, &key, &validation)?;
        let claims = data.claims;
        let now = Utc::now();
        if now.timestamp_millis() > claims.exp {
            return Err(AppError::AccessTokenExpired);
        }
        Ok(claims)
    }

    pub fn hash_password(pwd: &str) -> Result<String, AppError> {
        let hash = bcrypt::hash(pwd, bcrypt::DEFAULT_COST)?;
        Ok(hash)
    }

    pub fn is_valid_password(pwd: &str, pwd_hash: &str) -> bool {
        bcrypt::verify(pwd, pwd_hash).unwrap_or(false)
    }

    pub async fn sign_in(
        pool: &SqlitePool,
        credentials: Credentials,
    ) -> Result<AccessToken, AppError> {
        match UserRepo::get_user_by_username(pool, &credentials.username).await {
            Ok(user) => {
                if AuthService::is_valid_password(&credentials.password, &user.password_hash) {
                    let token = AuthService::create_access_token(user.id, user.is_superuser)?;
                    return Ok(token);
                }
                Err(AppError::InvalidCredentials)
            }
            Err(e) => match e {
                AppError::ObjectNotFound => Err(AppError::InvalidCredentials),
                _ => Err(e),
            },
        }
    }
}
