use crate::models::NewSignupShopkeepers;
use argon2::{Argon2, PasswordHasher, password_hash::SaltString};
use rand_core::OsRng;
use validator::ValidateEmail;

pub async fn check_signup_shopkeeper(
    req: NewSignupShopkeepers,
) -> Result<NewSignupShopkeepers, String> {
    if let Some(ref email) = req.email {
        if !email.validate_email() {
            return Err("Invalid email format".into());
        }
    }
    if let Some(ref phone) = req.phone_number {
        if phone.len() != 10 {
            return Err("Phone number should be 10 digits".into());
        }
    }

    let mut shopkeeper = req;

    if let Some(password) = shopkeeper.password {
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

        shopkeeper.password = Some(hashed_password);
    }

    Ok(shopkeeper)
}
