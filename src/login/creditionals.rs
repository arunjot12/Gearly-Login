use crate::{
    AppState,
    models::{Login, SignupShopkeepers, Users},
    schema::shopkeepers::{self, dsl::*},
    signup::handler::AppError,
};
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{Json, extract::State, http::StatusCode};
use diesel::prelude::*;

pub async fn login_shopkeeper(
    Json(payload): Json<Login>,
    State(state): State<AppState>,
) -> Result<Json<String>, AppError> {
    let connection = state
        .db
        .get()
        .await
        .expect("Failed to get DB connection from pool");

    let shopkeeper = connection
        .interact(move |conn| {
            shopkeepers::table
                .filter(
                    email
                        .eq(&payload.username_or_email)
                        .or(shopkeepers::username.eq(&payload.username_or_email)),
                )
                .first::<SignupShopkeepers>(conn)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        })
        .await
        .map_err(|e| AppError::DatabaseInteractError(e))?;

    let shopkeeper = match shopkeeper {
        Ok(shopkeeper) => shopkeeper,
        Err(_) => return Err(AppError::Internal),
    };

    let password_db = match shopkeeper.password {
        Some(hash) => hash,
        None => return Err(AppError::InvalidCreditionals),
    };

    // ✅ Wait for the task and handle the result
    tokio::task::spawn_blocking(move || {
        if !verify_password_details(&payload.password, &password_db) {
            return Err("Invalid Credentials".to_string());
        }
        Ok(())
    })
    .await
    .map_err(|_| AppError::ThreadError("Task panicked".to_string()))? // Handle join error
    .map_err(|e| AppError::ThreadError(e))?; // Handle the inner error from the closure

    let token = state
        .jwt
        .create_token(shopkeeper.id, "shopkeeper".to_string())
        .map_err(|_| AppError::ThreadError("Failed to create jwt".to_string()))?;

    Ok(Json(token))
}

#[axum::debug_handler]
pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<Login>,
) -> Result<Json<String>, AppError> {
    let connection = state
        .db
        .get()
        .await
        .expect("Failed to get DB connection from pool");

    let users = connection
        .interact(move |conn| {
            crate::schema::users::table
                .filter(
                    crate::schema::users::dsl::email
                        .eq(&payload.username_or_email)
                        .or(crate::schema::users::username.eq(&payload.username_or_email)),
                )
                .first::<Users>(conn)
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        })
        .await
        .map_err(|e| AppError::DatabaseInteractError(e))?;

    let user = match users {
        Ok(user) => user,
        Err(_) => return Err(AppError::Internal),
    };

    let password_db = match user.password {
        Some(hash) => hash,
        None => return Err(AppError::InvalidCreditionals),
    };

    // ✅ Wait for the task and handle the result
    tokio::task::spawn_blocking(move || {
        if !verify_password_details(&payload.password, &password_db) {
            return Err("Invalid Credentials".to_string());
        }
        Ok(())
    })
    .await
    .map_err(|_| AppError::ThreadError("Task panicked".to_string()))? // Handle join error
    .map_err(|e| AppError::ThreadError(e))?; // Handle the inner error from the closure

    let token = state
        .jwt
        .create_token(user.id, "Customer".to_string())
        .map_err(|_| AppError::ThreadError("Failed to create jwt".to_string()))?;

    println!(" The JWT token is {:?}", token);

    Ok(Json(token))
}

fn verify_password_details(other_password: &str, password_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(hash) => hash,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(other_password.as_bytes(), &parsed_hash)
        .is_ok()
}
