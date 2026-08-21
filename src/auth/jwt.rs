use jsonwebtoken::{EncodingKey, Header, encode};
use time::{Duration, OffsetDateTime};
use uuid::Uuid;

use crate::{auth::claims::Claims, error::AppError};


pub fn create_token(
    user_id: Uuid,
    role_id: Option<Uuid>,
    secret: &[u8]
) -> Result<String, AppError> {
    let exp = (OffsetDateTime::now_utc() + Duration::hours(24)).unix_timestamp();

    let claims = Claims{
        sub: user_id,
        exp,
        role_id,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
        .map_err(|_| AppError::InternalServerError("no se pudo crear jwt token".into()))
}

