use axum::{Router, routing::post};

use crate::{auth::handlers::login, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
}