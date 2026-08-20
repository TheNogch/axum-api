use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier, password_hash::{SaltString, rand_core::OsRng}};

use crate::error::AppError;

pub fn hash_password(
    password: &str
) -> Result<String, AppError>{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let hash = argon2
                .hash_password(password.as_bytes(), &salt)
                .map_err(|_| AppError::InternalServerError("no se puedo hashear la contraseña".into()))?;

    Ok(hash.to_string())
}

pub fn verify_password(
    password: &str,
    hash: &str
) -> Result<bool, AppError>{
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|_| AppError::InternalServerError("hash almacenado invalido".into()))?;

    let argon2 = Argon2::default();

    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}