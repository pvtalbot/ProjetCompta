use axum::{Router, extract::State, routing::get};
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL variable must be set in the .env file");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    println!("✅ Database connected successfully!");

    // The Router automatically infers Router<PgPool> thanks to .with_state(pool)
    let app = Router::new()
        .route("/", get(welcome))
        .route("/status", get(check_db_status))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("🚀 Server started on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn welcome() -> &'static str {
    "Welcome to the Accounting API!"
}

async fn check_db_status(State(pool): State<PgPool>) -> String {
    let version: (String,) = sqlx::query_as("SELECT version()")
        .fetch_one(&pool)
        .await
        .unwrap();

    format!("Database is responding! Postgres version: {}", version.0)
}