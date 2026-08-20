use axum::{extract::FromRequestParts, http::request::Parts};
use axum_extra::{TypedHeader, headers::{Authorization, authorization::Bearer}};
use jsonwebtoken::{DecodingKey, Validation, decode};

use crate::{auth::claims::Claims, error::AppError, state::AppState};

pub struct AuthUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AppError::Unauthorized)?;

        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?;

        Ok(AuthUser(token_data.claims))
    }
}