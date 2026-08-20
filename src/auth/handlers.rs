use axum::{Json, extract::State};

use crate::{auth::{jwt::create_token, models::{LoginRequest, LoginResponse}, password::verify_password}, error::AppError, state::AppState};


pub async  fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginRequest>
) -> Result<Json<LoginResponse>, AppError> {
    let user = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE username = $1",
        input.username
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    let valid = verify_password(&input.password, &user.password_hash)?;

    if !valid {
        return  Err(AppError::Unauthorized);
    }

    let token = create_token(user.id, state.jwt_secret.as_bytes())?;

    Ok(Json(LoginResponse{ token }))
}