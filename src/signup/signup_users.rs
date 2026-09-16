use crate::models::NewUsers;
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use rand_core::OsRng;
use validator::ValidateEmail;

pub async fn check_signup_user(req: NewUsers) -> Result<NewUsers, String> {
    if !req.email.validate_email() {
        return Err("Invalid email".into());
    }
    if req.phone_number.len() != 10 {
        return Err("Phone number should be 10 digits".into());
    }

    let password = req.password;

    let hashed_password = tokio::task::spawn_blocking(move || {
        let argon = Argon2::default();
        let salt = SaltString::generate(&mut OsRng);
        argon
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| e.to_string())
    })
    .await
    .map_err(|_| "Task panicked".to_string())??;

    Ok(NewUsers {
        first_name: req.first_name,
        last_name: req.last_name,
        username: req.username,
        email: req.email,
        phone_number: req.phone_number,
        password: hashed_password,
    })
}
