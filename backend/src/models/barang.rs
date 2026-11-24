use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;

#[derive(Debug, Serialize, FromRow)]
pub struct Barang {
    pub id: i32,
    pub nama: String,
    pub stok: i32,
    pub harga: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct BarangBaru {
    pub nama: String,
    pub stok: i32,
    pub harga: Decimal,
}

#[derive(Debug, Deserialize)]
pub struct PermintaanBarangBaru {
    pub nama: String,
    pub stok: i32,
    pub harga: Decimal,
    pub gudang_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct BarangUpdate {
    pub nama: Option<String>,
    pub stok: Option<i32>,
    pub harga: Option<Decimal>,
}

impl BarangBaru {
    pub fn validasi(&self) -> Result<(), Vec<String>> {
        let mut errors: Vec<String> = Vec::new();

        // Validasi nama
        if self.nama.trim().is_empty() {
            errors.push("Nama barang tidak boleh kosong".to_string());
        }

        // Validasi stok
        if self.stok < 0 {
            errors.push("Stok barang tidak boleh negatif".to_string());
        }

        // Validasi harga
        if self.harga < Decimal::ZERO {
            errors.push("Harga barang tidak boleh negatif".to_string());
        }

        // Periksa apakah ada error
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

impl Barang {
    pub async fn gudang_ada(pool: &PgPool, gudang_id: i64) -> Result<bool, sqlx::Error> {
        sqlx::query_scalar(
            "SELECT EXISTS (
                SELECT 1 FROM public.gudang WHERE id = $1
             )",
        )
        .bind(gudang_id)
        .fetch_one(pool)
        .await
    }

    pub async fn semua(pool: &PgPool) -> Result<Vec<Barang>, sqlx::Error> {
        let barang =
            sqlx::query_as::<_, Barang>("SELECT id, nama, stok, harga FROM barang ORDER BY id")
                .fetch_all(pool)
                .await?;

        Ok(barang)
    }

    pub async fn di_gudang(pool: &PgPool, gudang_id: i64) -> Result<Vec<Barang>, sqlx::Error> {
        let barang = sqlx::query_as::<_, Barang>(
            "SELECT id, nama, stok, harga
             FROM public.barang
             WHERE gudang_id = $1
             ORDER BY id",
        )
        .bind(gudang_id)
        .fetch_all(pool)
        .await?;

        Ok(barang)
    }

    pub async fn berdasarkan_id(pool: &PgPool, id: i32) -> Result<Option<Barang>, sqlx::Error> {
        let barang = sqlx::query_as::<_, Barang>(
            "SELECT id, nama, stok, harga
                FROM barang
                WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(barang)
    }

    pub async fn berdasarkan_id_di_gudang(
        pool: &PgPool,
        id: i32,
        gudang_id: i64,
    ) -> Result<Option<Barang>, sqlx::Error> {
        let barang = sqlx::query_as::<_, Barang>(
            "SELECT id, nama, stok, harga
             FROM public.barang
             WHERE id = $1 AND gudang_id = $2",
        )
        .bind(id)
        .bind(gudang_id)
        .fetch_optional(pool)
        .await?;

        Ok(barang)
    }

    pub async fn buat(
        pool: &PgPool,
        data: BarangBaru,
        gudang_id: i64,
    ) -> Result<Barang, sqlx::Error> {
        let barang = sqlx::query_as::<_, Barang>(
            "INSERT INTO public.barang (nama, stok, harga, gudang_id)
                VALUES ($1, $2, $3::numeric, $4)
                RETURNING id, nama, stok, harga",
        )
        .bind(data.nama)
        .bind(data.stok)
        .bind(data.harga)
        .bind(gudang_id)
        .fetch_one(pool)
        .await?;

        
        Ok(barang)
    }

    pub async fn ubah(
        pool: &PgPool,
        id: i32,
        data: BarangBaru,
    ) -> Result<Option<Barang>, sqlx::Error> {
        let barang = sqlx::query_as::<_, Barang>(
            "UPDATE barang
                SET nama = $1,
                    stok = $2,
                    harga = $3::numeric
                WHERE id = $4
                RETURNING id, nama, stok, harga",
        )
        .bind(data.nama)
        .bind(data.stok)
        .bind(data.harga)
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(barang)
    }

    pub async fn ubah_di_gudang(
        pool: &PgPool,
        id: i32,
        gudang_id: i64,
        data: BarangBaru,
    ) -> Result<Option<Barang>, sqlx::Error> {
        let barang = sqlx::query_as::<_, Barang>(
            "UPDATE public.barang
             SET nama = $1,
                 stok = $2,
                 harga = $3::numeric
             WHERE id = $4 AND gudang_id = $5
             RETURNING id, nama, stok, harga",
        )
        .bind(data.nama)
        .bind(data.stok)
        .bind(data.harga)
        .bind(id)
        .bind(gudang_id)
        .fetch_optional(pool)
        .await?;

        Ok(barang)
    }

    pub async fn patch(
        pool: &PgPool,
        id: i32,
        data: BarangUpdate,
    ) -> Result<Option<Barang>, sqlx::Error> {
        let barang = sqlx::query_as::<_, Barang>(
            "UPDATE barang
                SET nama = COALESCE($1, nama),
                    stok = COALESCE($2, stok),
                    harga = COALESCE($3::numeric, harga)
                WHERE id = $4
                RETURNING id, nama, stok, harga",
        )
        .bind(data.nama)
        .bind(data.stok)
        .bind(data.harga)
        .bind(id)
        .fetch_optional(pool)
        .await?;

        Ok(barang)
    }

    pub async fn patch_di_gudang(
        pool: &PgPool,
        id: i32,
        gudang_id: i64,
        data: BarangUpdate,
    ) -> Result<Option<Barang>, sqlx::Error> {
        let barang = sqlx::query_as::<_, Barang>(
            "UPDATE public.barang
             SET nama = COALESCE($1, nama),
                 stok = COALESCE($2, stok),
                 harga = COALESCE($3::numeric, harga)
             WHERE id = $4 AND gudang_id = $5
             RETURNING id, nama, stok, harga",
        )
        .bind(data.nama)
        .bind(data.stok)
        .bind(data.harga)
        .bind(id)
        .bind(gudang_id)
        .fetch_optional(pool)
        .await?;

        Ok(barang)
    }

    pub async fn hapus(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
        let hasil = sqlx::query("DELETE FROM barang WHERE id = $1 RETURNING id")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(hasil.is_some())
    }

    pub async fn hapus_di_gudang(
        pool: &PgPool,
        id: i32,
        gudang_id: i64,
    ) -> Result<bool, sqlx::Error> {
        let hasil = sqlx::query(
            "DELETE FROM public.barang
             WHERE id = $1 AND gudang_id = $2
             RETURNING id",
        )
        .bind(id)
        .bind(gudang_id)
        .fetch_optional(pool)
        .await?;

        Ok(hasil.is_some())
    }
}
