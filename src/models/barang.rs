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
    pub async fn semua(pool: &PgPool) -> Result<Vec<Barang>, sqlx::Error> {
        let barang =
            sqlx::query_as::<_, Barang>("SELECT id, nama, stok, harga FROM barang ORDER BY id")
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

    pub async fn buat(pool: &PgPool, data: BarangBaru) -> Result<Barang, sqlx::Error> {
        let barang = sqlx::query_as::<_, Barang>(
            "INSERT INTO barang (nama, stok, harga)
                VALUES ($1, $2, $3::numeric)
                RETURNING id, nama, stok, harga",
        )
        .bind(data.nama)
        .bind(data.stok)
        .bind(data.harga)
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

    pub async fn hapus(pool: &PgPool, id: i32) -> Result<bool, sqlx::Error> {
        let hasil = sqlx::query("DELETE FROM barang WHERE id = $1 RETURNING id")
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(hasil.is_some())
    }
}
