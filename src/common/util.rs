use argon2::{
    password_hash::SaltString, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
    Version,
};

use super::error::{Error, Result};

pub fn hash_password(pass: &str) -> Result<String> {
    let arg2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        argon2::Version::V0x13,
        Params::default(),
    );
    let salt: SaltString = SaltString::generate(&mut rand::rngs::OsRng);

    Ok(arg2
        .hash_password(pass.as_bytes(), &salt)
        .map_err(|err| Error::Message(err.to_string()))?
        .to_string())
}

pub fn verify_password(pass: &str, hashed_password: &str) -> bool {
    let arg2 = Argon2::new(
        argon2::Algorithm::Argon2id,
        Version::V0x13,
        Params::default(),
    );
    let Ok(hash) = PasswordHash::new(hashed_password) else {
        return false;
    };

    arg2.verify_password(pass.as_bytes(), &hash).is_ok()
}
