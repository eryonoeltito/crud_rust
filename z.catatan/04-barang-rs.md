# 04 — `src/models/barang.rs`

Model mendefinisikan bentuk data dan operasi SQL untuk barang.

## Struct data
```rust
#[derive(Debug, Serialize, FromRow)]
pub struct Barang { pub id: i32, pub nama: String, pub stok: i32, pub harga: f64 }
```
- `struct` mengelompokkan beberapa nilai sebagai satu barang.
- `Serialize` memungkinkan data diserialisasi jika diperlukan.
- `FromRow` membantu SQLx memetakan kolom hasil query ke field struct.
- `pub` membuat field dapat diakses dari modul lain.

**Jika tanpa `FromRow`?** `query_as::<_, Barang>` tidak bisa memakai implementasi pemetaan otomatis yang disediakan derive tersebut; perlu implementasi/manual mapping atau cara query lain.

## Data masuk
`BarangBaru` memakai `Deserialize`, karena datang dari JSON request. `BarangUpdate` memakai `Option<T>` agar field boleh tidak dikirim saat PATCH.

Contoh PATCH:
```json
{"stok": 12}
```
`nama` dan `harga` menjadi `None`; stok menjadi `Some(12)`. Jika memakai `String`/`i32` biasa untuk semua field, JSON parsial yang tidak menyertakan field bisa gagal dideserialisasi.

## Validasi
`BarangBaru::validasi()` membuat `Vec<String>` untuk menampung semua kesalahan. `trim().is_empty()` menolak nama kosong atau hanya spasi. Stok dan harga negatif ditolak. Jika tidak ada kesalahan, hasilnya `Ok(())`; jika ada, `Err(errors)`.

**Mengapa kumpulkan error?** Pengguna bisa mengetahui beberapa kesalahan sekaligus. Tanpa `Vec`, program bisa berhenti pada kesalahan pertama dan pengguna harus memperbaiki satu per satu. Alternatif sederhana: langsung mengembalikan satu pesan error pertama.

Validasi di aplikasi meningkatkan pesan yang jelas, tetapi constraint database juga penting sebagai lapisan perlindungan kedua.

## Query dan parameter binding
Contoh:
```rust
sqlx::query_as::<_, Barang>(
    "SELECT id, nama, stok, harga::float8 AS harga FROM barang WHERE id = $1"
)
.bind(id)
.fetch_optional(pool)
.await?;
```
- `query_as` meminta hasil query dipetakan ke `Barang`.
- `$1` adalah parameter pertama PostgreSQL.
- `.bind(id)` memasukkan nilai ID sebagai parameter, bukan menggabungkan teks SQL.
- `fetch_optional` menghasilkan `Some(data)` bila ditemukan atau `None` bila tidak ada baris.
- `.await` menunggu operasi async; `?` meneruskan error ke pemanggil.

**Kenapa `.bind()`?** Lebih aman dan benar untuk nilai input dibanding membangun SQL seperti `format!("... WHERE id = {}", id)`, yang dapat memunculkan risiko SQL injection jika input teks digabungkan. Parameter binding juga menangani tipe nilai dengan tepat.

## Fungsi model
- `semua`: `fetch_all` mengambil semua baris dan mengembalikan `Vec<Barang>`; `ORDER BY id` membuat urutan hasil jelas.
- `berdasarkan_id`: `fetch_optional` karena ID mungkin tidak ada.
- `buat`: `INSERT ... RETURNING` memasukkan data sekaligus mengambil baris yang baru dibuat, termasuk ID yang dihasilkan DB.
- `ubah`: `UPDATE` mengganti seluruh field yang diminta PUT; `Option<Barang>` membedakan ditemukan/tidak.
- `patch`: `COALESCE($1, nama)` memilih nilai baru bila bukan NULL, selain itu mempertahankan kolom lama. Ini cocok untuk update sebagian.
- `hapus`: `DELETE ... RETURNING id` lalu `is_some()` menghasilkan `true` bila ada baris yang terhapus, `false` bila ID tidak ada.

**Mengapa `harga::float8 AS harga`?** Kolom database bertipe numeric, sedangkan field Rust di sini `f64`. Cast dan alias menyelaraskan tipe/nama hasil query dengan field model. Untuk sistem uang yang butuh presisi finansial tinggi, lebih baik pertimbangkan tipe decimal daripada `f64`; ini adalah kompromi representasi pada source saat ini.

## `fetch_one` vs `fetch_optional`
`fetch_one` cocok bila hasil satu baris wajib ada; jika tidak ada, operasi error. `fetch_optional` cocok untuk pencarian berdasarkan ID karena tidak ditemukan adalah kondisi yang perlu ditangani menjadi HTTP 404, bukan selalu dianggap error database.

Kolom harga di PostgreSQL menggunakan NUMERIC, sedangkan field harga pada struct Rust saya menggunakan f64. Karena tipe datanya berbeda, saya menggunakan ::float8 pada query untuk mengonversi nilai hasil query menjadi tipe yang sesuai dengan f64 di Rust.

NUMERIC adalah tipe data PostgreSQL untuk menyimpan angka dengan presisi desimal yang dapat ditentukan. Tipe ini cocok untuk harga karena angka desimalnya bisa disimpan secara tepat dalam batas presisi yang ditentukan.

Karena operasi DELETE bertujuan menghapus data, bukan mengambil baris-baris data. Oleh karena itu, saya menggunakan .execute(pool), sedangkan .fetch_all(pool) digunakan ketika ingin mengambil semua baris hasil query.

Saya menggunakan if/else karena variabel terhapus sudah berupa boolean, sehingga hanya perlu memeriksa apakah nilainya true atau false. Error database ditangani secara terpisah. Sebenarnya match juga bisa digunakan, tetapi if/else lebih sederhana untuk kondisi ini.