use axum::{
    extract::State,
    routing::get,
    Json, Router,
};
use chrono::{NaiveDateTime};
use serde::Serialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_web_stack=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    // Set up connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Build our application with a single route
    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(health_check))
        .route("/widgets", get(list_widgets))
        .with_state(pool);

    // Run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(State(pool): State<PgPool>) -> &'static str {
    let row: (i32,) = sqlx::query_as("SELECT 1")
        .fetch_one(&pool)
        .await
        .unwrap();
    
    tracing::info!("Database query executed: SELECT {}", row.0);
    "Hello, world!"
}

async fn health_check() -> &'static str {
    "OK"
}

#[derive(Serialize, FromRow)]
struct Widget {
    id: Uuid,
    name: String,
    created_at: NaiveDateTime,
    deleted_at: Option<NaiveDateTime>,
}

async fn list_widgets(State(pool): State<PgPool>) -> Json<Vec<Widget>> {
    let widgets = sqlx::query_as::<_, Widget>("SELECT id, name, created_at, deleted_at FROM web_hs.widgets")
        .fetch_all(&pool)
        .await
        .unwrap();

    Json(widgets)
}
