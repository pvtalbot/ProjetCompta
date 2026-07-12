use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::SaltString};
use axum::{Json, extract::State, http::StatusCode};
use sqlx::PgPool;

use crate::auth::models::{AuthResponse, LoginDto, RegisterDto, User};

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterDto>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .to_string();

    let user = sqlx::query_as::<_, User>(
        "insert into users (username, email, password_hash) values ($1, $2, $3)",
    )
    .bind(payload.username)
    .bind(payload.email)
    .bind(password_hash)
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginDto>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>("select * from users where email = $1")
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    let token = format!("dummy-jwt-token-for-{}", user.id);

    Ok(Json(AuthResponse {
        token,
        username: user.username,
    }))
}
