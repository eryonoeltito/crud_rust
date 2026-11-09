use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use sqlx::{PgConnection, PgPool};

pub struct User;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AkunTerverifikasi {
    pub id: i64,
    pub role: Option<String>,
    pub gudang_id: Option<i64>,
}

#[derive(Debug)]
pub enum UserError {
    Database(sqlx::Error),
    PasswordHash(argon2::password_hash::Error),
}

impl std::fmt::Display for UserError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(error) => write!(formatter, "database error: {error}"),
            Self::PasswordHash(error) => write!(formatter, "password hash error: {error}"),
        }
    }
}

impl From<sqlx::Error> for UserError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

impl User {
    pub async fn ambil_akun_terverifikasi(
        koneksi: &mut PgConnection,
        id: i64,
    ) -> Result<Option<AkunTerverifikasi>, sqlx::Error> {
        let akun = sqlx::query_as::<_, (i64, Option<String>, Option<i64>)>(
            "SELECT id, role, gudang_id
             FROM public.users
             WHERE id = $1
               AND aktif = TRUE
               AND (
                   (role = 'admin' AND gudang_id IS NULL)
                   OR (role = 'petugas_gudang' AND gudang_id IS NOT NULL)
               )",
        )
        .bind(id)
        .fetch_optional(&mut *koneksi)
        .await?;

        Ok(akun.map(|(id, role, gudang_id)| AkunTerverifikasi {
            id,
            role,
            gudang_id,
        }))
    }

    pub async fn register(
        pool: &PgPool,
        username: &str,
        password: &str,
    ) -> Result<bool, UserError> {
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(UserError::PasswordHash)?
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
        koneksi: &mut PgConnection,
        username: &str,
        password: &str,
    ) -> Result<Option<(i64, String)>, UserError> {
        let Some((user_id, password_hash, aktif, role)) =
            sqlx::query_as::<_, (i64, String, bool, Option<String>)>(
                "SELECT id, password, aktif, role FROM users WHERE username = $1",
            )
            .bind(username)
            .fetch_optional(&mut *koneksi)
            .await?
        else {
            return Ok(None);
        };

        let password_cocok = verifikasi_password(password, &password_hash);
        let login_diizinkan = boleh_login(password_cocok, aktif);

        if login_diizinkan {
            let role = role.filter(|role| role_login_diizinkan(Some(role)));
            Ok(role.map(|role| (user_id, role)))
        } else {
            Ok(None)
        }
    }
}

fn verifikasi_password(password: &str, password_hash: &str) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(password_hash) else {
        return false;
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

fn boleh_login(password_cocok: bool, aktif: bool) -> bool {
    password_cocok && aktif
}

fn role_login_diizinkan(role: Option<&str>) -> bool {
    matches!(role, Some("admin" | "petugas_gudang"))
}

