# 03 — Memahami `src/main.rs`

## 1. Menjalankan fungsi async
```rust
#[tokio::main]
async fn main() { ... }
```
- `async fn main` memungkinkan penggunaan `.await` untuk operasi asynchronous seperti koneksi DB dan server.
- `#[tokio::main]` menyiapkan runtime Tokio yang menjalankan fungsi async tersebut.
- **Tanpa runtime**, kode async tidak bisa dijalankan begitu saja dari `main` biasa; perlu runtime lain atau pendekatan sinkron.

## 2. Membaca `.env`
```rust
dotenvy::dotenv().ok();
let database_url = std::env::var("DATABASE_URL")
    .expect("DATABASE_URL tidak ditemukan di file .env");
```
`dotenv()` mencoba memuat variabel dari `.env`; `.ok()` mengabaikan hasil error pemanggilan tersebut. Setelah itu `var` mengambil `DATABASE_URL`. `expect` menghentikan program dengan pesan jelas jika variabel tidak tersedia.

**Kenapa URL DB tidak ditulis langsung?** Agar kredensial dan konfigurasi tidak ditanam di source. Kalau URL ditulis literal, orang yang mendapat kode bisa melihat password. Alternatifnya, set `DATABASE_URL` langsung di environment sistem/container. Jangan pernah menaruh password asli di catatan atau repository.

## 3. Pool database
```rust
let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect(&database_url)
    .await
    .expect("Gagal terhubung ke PostgreSQL");
```
Pool mengelola sekumpulan koneksi agar setiap request tidak perlu membuka koneksi database baru dari nol. `max_connections(5)` membatasi jumlah koneksi pool sampai lima; bukan membatasi pengguna API menjadi lima. Request dapat menunggu bila koneksi sedang dipakai.

**Jika tidak memakai pool?** Harus mengelola koneksi secara manual atau membuka koneksi baru berulang kali, yang menambah overhead dan pengelolaan error. Angka 5 hanyalah pilihan konfigurasi, bukan angka wajib.

**max_connections(5)** membatasi maksimal lima koneksi database yang dapat digunakan secara bersamaan. Jadi, bukan membatasi jumlah pengguna atau request. Jika semua koneksi sedang digunakan, request yang membutuhkan koneksi database akan menunggu sampai ada koneksi yang tersedia.

## 4. Inisialisasi tabel
Setelah pool berhasil terhubung, `main` memanggil `database::initialize(&pool).await`. Fungsi tersebut berada di `src/database.rs` dan memakai pool yang sama untuk menjalankan `CREATE TABLE IF NOT EXISTS` bagi tabel `barang` dan `users`.

Inisialisasi terjadi sebelum router, TCP listener, dan server Axum dijalankan. Jika salah satu perintah gagal, startup berhenti dengan pesan yang menyebut tabel dan detail error SQLx. Rincian kolom serta batasan `CREATE TABLE IF NOT EXISTS` dijelaskan di [17-database-rs.md](17-database-rs.md).

Tabel `riwayat_stok` tidak dibuat oleh inisialisasi ini karena source aplikasi saat ini tidak memiliki query atau model yang menggunakannya dan DDL-nya belum tersedia.

## 5. Router dan state
```rust
let app = Router::new()
    .route("/health", get(health_check))
    .merge(barang_routes::routes())
    .with_state(pool);
```
- `Router::new()` membuat router.
- `.route(...)` mendaftarkan endpoint pemeriksaan kesehatan.
- `.merge(...)` menggabungkan route barang agar semua endpoint berada di aplikasi yang sama.
- `.with_state(pool)` membagikan pool ke handler yang meminta `State<PgPool>`.

**Jika route barang tidak di-merge?** Router barang tidak menjadi bagian dari `app` yang diserve, sehingga endpoint barang tidak tersedia dari server utama.

**Jika state tidak dipasang?** Handler yang memerlukan `State<PgPool>` tidak mendapat state yang dibutuhkan dan tipe/router tidak akan cocok.

**with_state(pool)** berfungsi untuk memberikan data bersama berupa pool koneksi database kepada router Axum. Dengan begitu, handler yang membutuhkan database dapat mengambil pool tersebut untuk menjalankan query tanpa harus membuat kumpulan koneksi baru setiap kali request.

Router menerima request HTTP dan menentukan handler mana yang akan dijalankan berdasarkan rute yang diakses. Beberapa handler membutuhkan koneksi database untuk menjalankan query. Koneksi database dikelola oleh pool, kemudian pool diberikan kepada router melalui with_state(pool). Selanjutnya, handler yang membutuhkan database mengambil pool melalui parameter State(pool): State<PgPool> agar dapat menjalankan query ke database.

Jadi, alasan utama menggunakan with_state(pool) pada project kamu adalah agar handler dapat mengakses pool yang sudah disiapkan, tanpa harus membuat koneksi database sendiri berulang kali.

## 6. TCP listener dan port
```rust
let listener = TcpListener::bind("127.0.0.1:3000").await?;
axum::serve(listener, app).await?;
```
Listener membuka alamat lokal pada port 3000; `axum::serve` mulai menerima request menggunakan listener tersebut. Port 3000 dapat diganti jika port itu bebas dan alamatnya sesuai kebutuhan. `127.0.0.1` hanya menerima koneksi dari komputer sendiri; jika perlu diakses dari perangkat lain, alamat bind harus dipertimbangkan dan diamankan.

Jika port sudah dipakai program lain, `bind` gagal. Server tidak berjalan hanya karena router sudah dibuat—listener dan `serve` diperlukan agar request benar-benar diterima.

axum::serve berfungsi untuk menjalankan server HTTP Axum menggunakan TcpListener dan router aplikasi, sehingga aplikasi dapat menerima dan menangani request dari client."

Ingat, bro: TcpListener menunggu koneksi, axum::serve menjalankan layanan server, sedangkan router menentukan handler yang sesuai.

## 7. Health check
`GET /health` mengembalikan JSON status dan pesan. Endpoint ini membantu mengecek apakah server merespons, tetapi **tidak otomatis membuktikan semua query database berhasil**.
