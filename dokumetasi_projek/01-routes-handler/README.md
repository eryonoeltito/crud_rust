# 01 — Routes dan Handler

Route menghubungkan kombinasi HTTP method dan path dengan fungsi handler. Path adalah alamat yang diminta, seperti `/barang`; method menyatakan jenis tindakan, seperti GET untuk membaca atau POST untuk membuat. Handler menjalankan pekerjaan request: mengambil input, memvalidasi, memanggil model, dan menentukan respons. Pemisahan ini memudahkan kita mengetahui apakah perubahan perlu dilakukan pada pemetaan URL atau pada logika pengolahan data.

## Routes

Pendaftaran route berada di `src/routes/`. `barang_routes.rs` mendaftarkan endpoint barang, sedangkan `auth_routes.rs` mendaftarkan register, login, dan logout. Masing-masing fungsi `routes()` menghasilkan router dengan state pool database yang diperlukan handler. Router-router itu digabungkan ke router utama di `src/main.rs`.

Contoh konsep:

```rust
.route("/barang", get(list_barang).post(tambah_barang))
```

Path `/barang` menerima GET dan POST, masing-masing diarahkan ke handler yang sesuai. Path `/barang/{id}` memiliki bagian dinamis; Axum mengambil nilainya melalui extractor `Path(id)`. Contohnya, request `GET /barang/12` memberikan angka `12` kepada handler detail. Routes barang dipasangi middleware `wajib_login`; routes autentikasi tidak berada di lapisan tersebut agar pengguna masih dapat mengirim request login.

## Handler

Handler berada di `src/controllers/`. Axum menyediakan extractor seperti `State(pool)` untuk state aplikasi, `Path(id)` untuk parameter URL, dan `Json(data)` untuk body JSON. Dengan extractor, handler menerima nilai bertipe yang sudah diparsing Axum, bukan membaca byte request secara manual. Handler kemudian memanggil model dan mengembalikan status HTTP beserta body.

Status yang umum digunakan proyek: `200 OK` untuk berhasil, `201 Created` untuk pembuatan data, `400 Bad Request` untuk input tidak valid, `401 Unauthorized` untuk belum login atau kredensial salah, `404 Not Found` untuk data tidak ada, `409 Conflict` untuk username yang sudah dipakai, dan `500 Internal Server Error` untuk kegagalan internal. Jika terjadi error query, handler mencatat detailnya di log server dan mengirim pesan umum kepada client supaya rincian internal tidak ikut dibocorkan.

Pembagian ini membuat route fokus pada pemetaan alamat, sementara alur request berada di handler. Sebagai contoh, request membuat barang melewati route POST, masuk ke handler `buat_barang`, divalidasi, lalu disimpan oleh model. Rincian operasi barang ada di [03 — CRUD Barang](../03-crud-barang/README.md), sedangkan autentikasi ada di [06 — Login dan Logout](../06-login-logout/README.md).
