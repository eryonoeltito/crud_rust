use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use sqlx::PgPool;

pub struct User;

#[derive(Debug)]
pub enum UserError {
    Database,
    PasswordHash,
}

impl From<sqlx::Error> for UserError {
    fn from(_: sqlx::Error) -> Self {
        Self::Database
    }
}

impl User {
    pub async fn register(
        pool: &PgPool,
        username: &str,
        password: &str,
    ) -> Result<bool, UserError> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| UserError::PasswordHash)?
            .to_string();

        let inserted = sqlx::query_scalar::<_, i32>(
            "INSERT INTO users (username, password)
             VALUES ($1, $2)
             ON CONFLICT (username) DO NOTHING
             RETURNING 1",
        )
        .bind(username)
        .bind(password_hash)
        .fetch_optional(pool)
        .await?;

        Ok(inserted.is_some())
    }

    pub async fn cek_login(
        pool: &PgPool,
        username: &str,
        password: &str,
    ) -> Result<bool, UserError> {
        let Some(password_hash) =
            sqlx::query_scalar::<_, String>("SELECT password FROM users WHERE username = $1")
                .bind(username)
                .fetch_optional(pool)
                .await?
        else {
            return Ok(false);
        };

        let Ok(parsed_hash) = PasswordHash::new(&password_hash) else {
            return Ok(false);
        };

        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
}
