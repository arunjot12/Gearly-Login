use crate::{
    models::{NewSignupShopkeepers, NewUsers, SignupShopkeepers, Users},
    schema::{shopkeepers, users},
    signup::handler::AppError::{
        Database, DatabaseInteractError, Internal, InvalidCreditionals, NotFound, ThreadError,
        UserAlreadyExists,
    },
};
use axum::response::IntoResponse;
use diesel::{insert_into, mysql::MysqlConnection, prelude::*};
use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("invalid creditionals")]
    InvalidCreditionals,

    #[error("User Not Found")]
    NotFound,

    #[error("Database error")]
    Database(#[from] diesel::result::Error),

    #[error("Internal Server Error")]
    Internal,

    #[error("User Already Exist")]
    UserAlreadyExists,

    #[error("InteractError Error")]
    DatabaseInteractError(#[from] deadpool_diesel::InteractError),

    #[error("Tokio Error")]
    ThreadError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            InvalidCreditionals => {
                (StatusCode::UNAUTHORIZED, "Invalid creditionals".to_string()).into_response()
            }
            UserAlreadyExists => {
                (StatusCode::CONFLICT, "User already existed".to_string()).into_response()
            }
            NotFound => (StatusCode::NOT_FOUND, "User not found".to_string()).into_response(),
            Database(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{e}.to_string()",),
            )
                .into_response(),
            Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            )
                .into_response(),
            DatabaseInteractError(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("{e}.to_string()"),
            )
                .into_response(),
            ThreadError(s) => (format!("{s}.to_string")).into_response(),
        }
    }
}

pub fn handle_customer_signup(
    connection: &mut MysqlConnection,
    customer: &NewUsers,
) -> Result<String, AppError> {
    let check_customer_number = users::table
        .select(Users::as_select())
        .filter(users::phone_number.eq(&customer.phone_number))
        .first(connection)
        .optional();

    match check_customer_number {
        Ok(Some(_)) => return Err(AppError::UserAlreadyExists), // You need to define this error
        Ok(None) => {}                                          // Good, no user found, proceed
        Err(err) => return Err(AppError::Database(err)),
    }

    let insert_result = insert_into(users::table)
        .values(customer)
        .execute(connection);

    match insert_result {
        Ok(_) => Ok("✅ Shopkeeper registered successfully!".to_string()),
        Err(err) => return Err(AppError::Database(err)),
    }
}

pub fn handle_shopkeeper_signup(
    connection: &mut MysqlConnection,
    shopkeeper: &NewSignupShopkeepers,
) -> Result<String, AppError> {
    if let Some(ref phone) = shopkeeper.phone_number {
        let check_shopkeeper_number: Result<Option<SignupShopkeepers>, _> =
            shopkeepers::table
                .select(SignupShopkeepers::as_select())
                .filter(shopkeepers::phone_number.eq(phone))
                .first(connection)
                .optional();

        match check_shopkeeper_number {
            Ok(Some(_)) => return Err(AppError::UserAlreadyExists), // You need to define this error
            Ok(None) => {}                                          // Good, no user found, proceed
            Err(err) => return Err(AppError::Database(err)),
        }
    }

    let insert_result = insert_into(shopkeepers::table)
        .values(shopkeeper)
        .execute(connection);

    match insert_result {
        Ok(_) => Ok("✅ Shopkeeper registered successfully!".to_string()),
        Err(err) => return Err(AppError::Database(err)),
    }
}
