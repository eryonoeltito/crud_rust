use serde::Serialize;
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, FromRow)]
pub struct RiwayatStok {
    pub id: i32,
    pub barang_id: Option<i32>,
    pub stok_lama: i32,
    pub stok_baru: i32,
    pub waktu_perubahan: Option<String>,
}

impl RiwayatStok {
    pub async fn semua(pool: &PgPool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT id, barang_id, stok_lama, stok_baru,
                    waktu_perubahan::text AS waktu_perubahan
             FROM public.riwayat_stok
             ORDER BY waktu_perubahan DESC NULLS LAST",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn di_gudang(pool: &PgPool, gudang_id: i64) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT r.id, r.barang_id, r.stok_lama, r.stok_baru,
                    r.waktu_perubahan::text AS waktu_perubahan
             FROM public.riwayat_stok r
             JOIN public.barang b ON b.id = r.barang_id
             WHERE b.gudang_id = $1
             ORDER BY r.waktu_perubahan DESC NULLS LAST",
        )
        .bind(gudang_id)
        .fetch_all(pool)
        .await
    }

    pub async fn barang_ada(pool: &PgPool, barang_id: i32) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar(
            "SELECT EXISTS (
                SELECT 1 FROM public.barang WHERE id = $1
             )",
        )
        .bind(barang_id)
        .fetch_one(pool)
        .await
    }

    pub async fn barang_ada_di_gudang(
        pool: &PgPool,
        barang_id: i32,
        gudang_id: i64,
    ) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar(
            "SELECT EXISTS (
                SELECT 1 FROM public.barang WHERE id = $1 AND gudang_id = $2
             )",
        )
        .bind(barang_id)
        .bind(gudang_id)
        .fetch_one(pool)
        .await
    }

    pub async fn berdasarkan_barang(
        pool: &PgPool,
        barang_id: i32,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT id, barang_id, stok_lama, stok_baru,
                    waktu_perubahan::text AS waktu_perubahan
             FROM public.riwayat_stok
             WHERE barang_id = $1
             ORDER BY waktu_perubahan DESC NULLS LAST",
        )
        .bind(barang_id)
        .fetch_all(pool)
        .await
    }

    pub async fn berdasarkan_barang_di_gudang(
        pool: &PgPool,
        barang_id: i32,
        gudang_id: i64,
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Self>(
            "SELECT r.id, r.barang_id, r.stok_lama, r.stok_baru,
                    r.waktu_perubahan::text AS waktu_perubahan
             FROM public.riwayat_stok r
             JOIN public.barang b ON b.id = r.barang_id
             WHERE r.barang_id = $1 AND b.gudang_id = $2
             ORDER BY r.waktu_perubahan DESC NULLS LAST",
        )
        .bind(barang_id)
        .bind(gudang_id)
        .fetch_all(pool)
        .await
    }
}
