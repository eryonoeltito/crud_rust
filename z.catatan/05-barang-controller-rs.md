# 05 — `src/controllers/barang_controller.rs`

Controller adalah pengatur alur request. Ia menerima input dari Axum, melakukan validasi, memanggil model, lalu menentukan respons.

## Extractor Axum
```rust
State(pool): State<PgPool>,
Path(id): Path<i32>,
Json(data): Json<BarangBaru>,
```
- `State` mengambil pool yang dipasang lewat `.with_state(pool)`.
- `Path` mengambil parameter URL, misalnya `/barang/7` → `id = 7`.
- `Json` membaca body JSON dan mengubahnya menjadi struct.

Tanpa extractor ini, handler harus membaca dan mem-parsing request secara manual atau memakai pendekatan lain. Extractor membuat kode lebih ringkas dan tipe input lebih jelas.

## Penanganan error SQLx
Pola source:
```rust
.map_err(|error| {
    eprintln!("Gagal mengambil data barang: {error}");
    (StatusCode::INTERNAL_SERVER_ERROR,
     "Gagal mengambil data barang".to_string())
})?;
```
`map_err` mengubah jenis error SQLx menjadi pasangan status HTTP dan pesan yang dipahami client. `eprintln!` menyimpan detail untuk log pengembang, sedangkan pesan ke client dibuat umum agar detail internal database tidak dibocorkan. `?` menghentikan alur saat error dan mengembalikan error tersebut.

**Jika langsung mengembalikan detail error SQL ke client?** Bisa membantu debugging, tetapi dapat membocorkan nama tabel, query, atau detail infrastruktur. Untuk production, log perlu dikelola dengan baik dan tidak berisi rahasia.

## Status HTTP yang digunakan
- `200 OK`: request berhasil, misalnya GET, PUT/PATCH, DELETE yang sukses.
- `201 CREATED`: barang berhasil dibuat oleh POST.
- `400 BAD REQUEST`: input tidak lolos validasi.
- `404 NOT FOUND`: ID tidak ditemukan.
- `500 INTERNAL SERVER ERROR`: kegagalan internal seperti query database.

Tanpa status yang tepat, client lebih sulit membedakan kesalahan input, data yang tidak ada, dan masalah server.

## Validasi create/PUT
`data.validasi()` mengembalikan `Ok(())` atau `Err(Vec<String>)`. Pada error, `errors.join("; ")` menggabungkan pesan menjadi satu string. Contoh: `Nama barang tidak boleh kosong; Stok barang tidak boleh negatif`.

## Validasi PATCH
```rust
if data.stok.is_some_and(|stok| stok < 0) { ... }
```
Pemeriksaan hanya dilakukan jika `stok` dikirim. `as_ref()` pada nama meminjam isi `Option<String>` sehingga nilai tetap tersedia untuk query. `is_some_and` menjalankan kondisi jika `Some`; untuk `None`, hasilnya false.

Kalau semua field PATCH divalidasi seperti field wajib, request `{ "stok": 12 }` akan ditolak karena nama dan harga tidak dikirim—padahal PATCH memang untuk perubahan sebagian.

## `match Some / None`
Model mengembalikan `Option<Barang>` untuk operasi berdasarkan ID. `Some(barang)` berarti baris ditemukan; `None` berarti tidak ditemukan. Controller mengubah `None` menjadi 404.

Jika tidak memeriksa `None`, aplikasi berisiko memberi respons sukses palsu atau tidak memberi tahu client bahwa ID salah.

## Catatan respons detail
Pada source ini, `detail_barang` menyusun JSON langsung, sedangkan create, PUT, dan PATCH menggunakan `barang_view::detail_barang_response`. Keduanya dapat berjalan, tetapi berpotensi membuat format berbeda jika satu tempat diubah dan tempat lain terlupa. Untuk konsistensi, detail GET bisa diarahkan memakai fungsi View yang sama.

## Delete
`Barang::hapus` mengembalikan boolean. `true` → pesan berhasil; `false` → 404. Penghapusan yang berkaitan dengan riwayat stok juga dipengaruhi aturan foreign key database. Controller sendiri tidak memblokir penghapusan karena riwayat; jika aturan bisnis melarangnya, perlu validasi/constraint yang sesuai.
