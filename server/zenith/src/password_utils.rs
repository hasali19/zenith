use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use eyre::eyre;

pub fn hash_password(password: &str) -> eyre::Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| eyre!(e))?
        .to_string())
}

pub fn verify_password(password: &str, hash: &str) -> eyre::Result<()> {
    let password_hash = PasswordHash::new(hash).map_err(|e| eyre!(e))?;

    Argon2::default()
        .verify_password(password.as_bytes(), &password_hash)
        .map_err(|e| eyre!(e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_simple() {
        let hash = hash_password("password123").unwrap();
        verify_password("password123", &hash).unwrap();
    }
}
