# 15 — Penjelasan konfigurasi session di `src/main.rs`

Dokumen ini berfokus pada bagian `main.rs` yang memasang session manager. Pertahankan bagian lain dari file utama, seperti konfigurasi database, alamat listener, dan pemanggilan `axum::serve`.

## Import terkait session

```rust
use tower_sessions::{Expiry, MemoryStore, SessionManagerLayer};
```

- `Expiry`: tipe untuk menentukan kapan cookie/session dianggap berakhir sesuai kebijakan expiry.
- `MemoryStore`: penyimpanan session di memori proses server.
- `SessionManagerLayer`: layer Tower yang mengelola session pada request dan response.

Tipe-tipe ini berasal dari crate `tower-sessions`. Di `main.rs`, session tidak disimpan ke tabel PostgreSQL; pool PostgreSQL di bagian atas tetap digunakan oleh fitur database seperti login dan CRUD.

## Membuat penyimpanan dan layer session

```rust
let session_store = MemoryStore::default();
```
- `let` membuat variabel.
- `MemoryStore::default()` membuat penyimpanan session baru di memori.
- Store ini menyimpan record session di proses server. Cookie client hanya membawa ID session yang dipakai untuk mencari record tersebut.
- Data ini tidak persisten: session yang tersimpan akan hilang saat server dimulai ulang. Beberapa proses server juga tidak berbagi store masing-masing.

```rust
let session_layer = SessionManagerLayer::new(session_store)
    .with_secure(false)
    .with_expiry(Expiry::OnSessionEnd);
```
- `SessionManagerLayer::new(session_store)` membuat pengelola session yang menggunakan store tadi.
- `.with_secure(false)` mengatur agar cookie tidak memerlukan HTTPS. Ini memungkinkan pengujian melalui HTTP lokal pada `127.0.0.1:3000`. Untuk aplikasi yang diakses lewat jaringan publik, cookie biasanya harus memakai HTTPS dan atribut `Secure` diaktifkan.
- `.with_expiry(Expiry::OnSessionEnd)` membuat cookie menjadi cookie sesi browser: konfigurasi ini tidak memberi `Max-Age` atau waktu kedaluwarsa tetap seperti beberapa jam/hari. Browser menentukan kapan sesi browser berakhir. Fitur pemulihan sesi browser dapat membuat perilakunya berbeda dari sekadar menutup jendela.
- Pengaturan tersebut mengatur masa hidup cookie di client. Ia tidak sama dengan menghapus record store secara langsung; pengelolaan record berada pada session store.

Nama cookie default dari `tower-sessions` adalah `id`. Nilainya adalah ID acak session, bukan username atau password. Nilai session seperti username disimpan oleh server di `MemoryStore`.

```rust
let app = Router::new()
    .route("/health", get(health_check))
    .merge(barang_routes::routes())
    .merge(auth_routes::routes())
    .with_state(pool)
    .layer(session_layer);
```
- `Router::new()` membuat router utama.
- `.route("/health", get(health_check))` menambahkan endpoint pemeriksaan server.
- `.merge(barang_routes::routes())` menggabungkan route CRUD barang.
- `.merge(auth_routes::routes())` menggabungkan route login/logout.
- `.with_state(pool)` menyediakan pool PostgreSQL kepada handler.
- `.layer(session_layer)` memasang pengelola session pada aplikasi sehingga handler/middleware dapat menerima `Session`.

`/health` juga berada di dalam router yang memakai layer ini, tetapi handler health check tidak perlu membaca session. Route yang membutuhkan data login mengakses `Session` lewat extractor atau middleware.

## Alur request login dan request berikutnya

1. Client mengirim `POST /login` dengan username dan password.
2. Handler login memeriksa kredensial ke PostgreSQL. Setelah valid, handler menjalankan `session.insert("username", input.username)`.
3. Saat response diselesaikan, `SessionManagerLayer` menyimpan perubahan session melalui `MemoryStore` dan mengirim cookie berisi ID session, biasanya lewat header `Set-Cookie`.
4. Client menyimpan cookie. Pada request selanjutnya, misalnya `GET /barang`, client mengirimkannya lewat header `Cookie`.
5. Layer membaca ID dari cookie dan memuat record session dari store. Middleware `wajib_login` lalu membaca nilai `username`; jika tidak ada, request ditolak dengan `401`, dan jika ada, request diteruskan ke handler barang.
6. Saat logout, handler menjalankan `session.flush()`, yang menghapus data session dari store dan membuat session lama tidak dapat dipakai lagi.

Ringkasnya:

```text
POST /login → session.insert → store menyimpan session → response Set-Cookie
request berikutnya + Cookie → layer memuat session → middleware memeriksa username
```

## Catatan konsep

Session dan cookie bukan hal yang sama:
- **Session**: data status login yang disimpan di sisi server.
- **Cookie**: cara client mengirim pengenal session ke server pada request berikutnya. Cookie bukan tempat penyimpanan password atau data barang.
- **JWT**: token yang berisi klaim dan ditandatangani; proyek ini tidak menggunakan JWT.

Dengan `MemoryStore`, server mencari session berdasarkan ID cookie. Jadi jika client hanya menyalin cookie ke client lain ketika server masih berjalan, client itu dapat bertindak sebagai session yang sama; cookie harus diperlakukan sebagai kredensial rahasia.

## Perhatian

Dokumentasi ini menjelaskan konfigurasi yang digunakan dalam proyek. Periksa lagi source aktif sebelum mengubah versi crate atau pengaturan expiry, karena API crate dapat berubah antarversi.
