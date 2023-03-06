pub fn hash_argon(password: String) -> String {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };

    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    return argon2
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string();
}

pub fn verify_hash(password: String, hash: String) -> bool {
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };

    let parsed_hash = PasswordHash::new(&hash);
    return Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash.unwrap())
        .is_ok();
}
