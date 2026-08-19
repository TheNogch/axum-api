use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{error::AppError, state::AppState};
use super::models::{CreateUser, User, UpdateUser};

pub async fn list_users(
    State(state): State<AppState>
) -> Result<(StatusCode, Json<Vec<User>>), AppError> {

    let users = sqlx::query_as!(
        User,
        "SELECT * FROM users ORDER BY created_at DESC"
    )
    .fetch_all(&state.pool)
    .await?;

    Ok((StatusCode::OK, Json(users)))
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let id = Uuid::now_v7();

    let user = sqlx::query_as!(
        User,
        "INSERT INTO users (id, username, age) VALUES ($1, $2, $3) RETURNING id, username, age, created_at, updated_at",
        id,
        input.username,
        input.age,
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| match &e {
        sqlx::Error::Database(db_err) if db_err.is_unique_violation() => {
            AppError::Conflict(format!("el username '{}' ya existe", input.username))
        }
        _ => AppError::SqlxError(e),
    })?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<User>), AppError> {
    
    let user = sqlx::query_as!(
        User,
        "SELECT * FROM users where id = $1",
        id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateUser>,
) -> Result<(StatusCode, Json<User>), AppError> {
    let user = sqlx::query_as!(
        User,
        "UPDATE users SET username = COALESCE($1, username), age = COALESCE($2, age), updated_at = NOW() WHERE id = $3 RETURNING *",
        input.username,
        input.age,
        id
    )
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::OK, Json(user)))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>
) -> Result<StatusCode, AppError> {

    let result = sqlx::query!(
        "DELETE from users where id = $1",
        id
    )
    .execute(&state.pool)
    .await?;

    if result.rows_affected() == 0 {
        return  Err(AppError::NotFound);
    }
    
    Ok(StatusCode::NO_CONTENT)
}