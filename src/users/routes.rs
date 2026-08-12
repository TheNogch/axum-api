use axum::{Router, routing::{get}};

use crate::state::AppState;
use super::handlers::{create_user, list_users, get_user};

pub fn router() -> Router<AppState>{
    Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/{id}", get(get_user))

}