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
        pool: &PgPool,
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
        .fetch_optional(pool)
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
    ) -> Result<Option<i64>, UserError> {
        let Some((user_id, password_hash, aktif)) = sqlx::query_as::<_, (i64, String, bool)>(
            "SELECT id, password, aktif FROM users WHERE username = $1",
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
            Ok(Some(user_id))
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

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use sqlx::postgres::PgPoolOptions;

    use super::*;

    fn buat_hash_uji(password: &str) -> String {
        let garam = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &garam)
            .unwrap()
            .to_string()
    }

    #[test]
    fn login_akun_aktif_dengan_password_benar_diizinkan() {
        let hash = buat_hash_uji("password-uji");

        assert!(boleh_login(
            verifikasi_password("password-uji", &hash),
            true
        ));
    }

    #[test]
    fn login_akun_nonaktif_ditolak_meski_password_benar() {
        let hash = buat_hash_uji("password-uji");

        let login_diizinkan = boleh_login(verifikasi_password("password-uji", &hash), false);

        assert!(!login_diizinkan);
    }

    #[test]
    fn login_dengan_password_salah_ditolak() {
        let hash = buat_hash_uji("password-uji");

        assert!(!boleh_login(
            verifikasi_password("password-salah", &hash),
            true
        ));
    }

    #[tokio::test]
    #[ignore = "memerlukan database uji terpisah dengan skema RBAC hasil migration"]
    async fn login_database_mengizinkan_akun_aktif_dan_menolak_akun_nonaktif() {
        let alamat_database = std::env::var("DATABASE_URL_TEST")
            .expect("atur DATABASE_URL_TEST ke database uji terpisah");
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&alamat_database)
            .await
            .expect("gagal terhubung ke database uji");
        let waktu_sekarang = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("waktu sistem harus setelah UNIX epoch")
            .as_nanos();
        let nama_aktif = format!("uji_aktif_{waktu_sekarang}");
        let nama_nonaktif = format!("uji_nonaktif_{waktu_sekarang}");
        let hash_aktif = buat_hash_uji("password-uji");
        let hash_nonaktif = buat_hash_uji("password-uji");

        let mut transaksi = pool.begin().await.expect("gagal memulai transaksi uji");
        let id_aktif = sqlx::query_scalar::<_, i64>(
            "INSERT INTO public.users (username, password, role, gudang_id, aktif)
             VALUES ($1, $2, 'petugas_gudang', 1, TRUE)
             RETURNING id",
        )
        .bind(&nama_aktif)
        .bind(hash_aktif)
        .fetch_one(&mut *transaksi)
        .await
        .expect("gagal membuat akun aktif sementara");
        sqlx::query(
            "INSERT INTO public.users (username, password, role, gudang_id, aktif)
             VALUES ($1, $2, NULL, NULL, FALSE)",
        )
        .bind(&nama_nonaktif)
        .bind(hash_nonaktif)
        .execute(&mut *transaksi)
        .await
        .expect("gagal membuat akun pending sementara");

        assert_eq!(
            User::cek_login(&mut transaksi, &nama_aktif, "password-uji")
                .await
                .unwrap(),
            Some(id_aktif)
        );
        assert_eq!(
            User::cek_login(&mut transaksi, &nama_nonaktif, "password-uji")
                .await
                .unwrap(),
            None
        );
        assert_eq!(
            User::cek_login(&mut transaksi, &nama_aktif, "password-salah")
                .await
                .unwrap(),
            None
        );

        transaksi
            .rollback()
            .await
            .expect("gagal membatalkan transaksi uji");
    }
}
