use sqlx::PgConnection;

pub async fn cabut_token(
    koneksi: &mut PgConnection,
    jti: &str,
    waktu_kedaluwarsa: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO public.revoked_tokens (jti, expires_at)
         VALUES ($1, to_timestamp($2::double precision))
         ON CONFLICT (jti) DO NOTHING",
    )
    .bind(jti)
    .bind(waktu_kedaluwarsa)
    .execute(&mut *koneksi)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::time::{SystemTime, UNIX_EPOCH};

    use sqlx::postgres::PgPoolOptions;

    use super::cabut_token;

    #[tokio::test]
    #[ignore = "memerlukan database uji terpisah dengan public.revoked_tokens tersedia"]
    async fn logout_berulang_tidak_membuat_duplikasi() {
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
            .as_secs() as i64;
        let jti = format!("uji-{}-{waktu_sekarang}", std::process::id());
        let waktu_kedaluwarsa = waktu_sekarang + 3600;

        let mut transaksi = pool.begin().await.expect("gagal memulai transaksi uji");
        cabut_token(&mut transaksi, &jti, waktu_kedaluwarsa)
            .await
            .expect("pencabutan pertama gagal");
        cabut_token(&mut transaksi, &jti, waktu_kedaluwarsa)
            .await
            .expect("pencabutan berulang gagal");

        let jumlah_baris = sqlx::query_scalar::<_, i64>(
            "SELECT count(*) FROM public.revoked_tokens WHERE jti = $1",
        )
        .bind(&jti)
        .fetch_one(&mut *transaksi)
        .await
        .expect("gagal menghitung baris token yang dicabut");

        let exp_tersimpan = sqlx::query_scalar::<_, i64>(
            "SELECT EXTRACT(EPOCH FROM expires_at)::BIGINT
             FROM public.revoked_tokens WHERE jti = $1",
        )
        .bind(&jti)
        .fetch_one(&mut *transaksi)
        .await
        .expect("gagal membaca waktu kedaluwarsa token");

        assert_eq!(jumlah_baris, 1);
        assert_eq!(exp_tersimpan, waktu_kedaluwarsa);

        transaksi
            .rollback()
            .await
            .expect("gagal membatalkan transaksi uji");
    }
}
