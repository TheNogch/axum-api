mod error;
mod config;
mod state;
mod users;
mod auth;

use std::sync::Arc;

use config::Config;
use state::AppState;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main(){
    let config = Config::from_env();

    let pool = PgPoolOptions::new()
                .max_connections(10)
                .connect(&config.database_url)
                .await
                .expect("No se pudo conectar a la base de datos");

    let  state = AppState { 
        pool,
        jwt_secret: Arc::new(config.jwt_secret), 
    };

    let app = axum::Router::new()
                .nest("/users", users::routes::router())
                .nest("/auth", auth::routes::router())
                .with_state(state);
    
    let addr = format!("{}:{}", config.server_host, config.server_port);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("No se pudo enlazar el puerto");
    axum::serve(listener, app).await.expect("Error al ejecutar el servidor");
}