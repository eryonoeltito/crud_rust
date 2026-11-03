# Dokumentasi Pengujian API Inventaris

**Proyek:** `inventaris_api`  
**Metode pengujian:** Manual menggunakan `curl`  
**Alamat API lokal:** `http://127.0.0.1:3000`  
**Tanggal pengujian:** 3 November 2026


## 4.1 Pengujian Register

| Pengujian | Status | Hasil yang Diharapkan | Hasil yang Tertera |
|---|---|---|---|
| Register dengan data valid (`tes_register_01`) | Lulus | 201 Created dan pendaftaran berhasil | 201 Created, `{"message":"Pendaftaran berhasil","status":"success"}` |
| Register dengan username yang sudah digunakan | Lulus | 409 Conflict | 409 Conflict, `{"status":"error","message":"Username sudah digunakan"}` |
| Register dengan username berisi spasi saja | Lulus | 400 Bad Request | 400 Bad Request, `{"status":"error","message":"Username dan password wajib diisi"}` |
| Register dengan password kosong | Lulus | 400 Bad Request | 400 Bad Request, `{"status":"error","message":"Username dan password wajib diisi"}` |

## 4.2 Pengujian Login

| Pengujian | Status | Hasil yang Diharapkan | Hasil yang Tertera |
|---|---|---|---|
| Login dengan username dan password benar | Lulus | 200 OK dan token Bearer diterbitkan | 200 OK, `status: success`, `message: Login berhasil`, `token_type: Bearer`, `expires_in: 3600`; nilai token tidak dicantumkan |
| Login dengan password salah | Lulus | 401 Unauthorized | 401 Unauthorized, `{"status":"error","message":"Username atau password salah"}` |
| Login dengan username yang tidak terdaftar | Lulus | 401 Unauthorized | 401 Unauthorized, `{"status":"error","message":"Username atau password salah"}` |
| Login dengan format JSON rusak | Lulus | 400 Bad Request | 400 Bad Request, `{"status":"error","message":"Body JSON tidak valid"}` |

## 4.3 Pengujian JWT dan Middleware

| Pengujian | Status | Hasil yang Diharapkan | Hasil yang Tertera |
|---|---|---|---|
| GET `/barang` tanpa token | Lulus | 401 Unauthorized | 401 Unauthorized, `{"status":"error","message":"Token tidak ada atau tidak valid"}` |
| GET `/barang` dengan token palsu | Lulus | 401 Unauthorized | 401 Unauthorized, `{"status":"error","message":"Token tidak ada atau tidak valid"}` |
| GET `/barang` dengan token valid | Lulus | 200 OK dan data barang dapat diakses | 200 OK, middleware menerima token valid dan API menampilkan 9 data barang |

## 4.4 Pengujian GET Barang

| Pengujian | Status | Hasil yang Diharapkan | Hasil yang Tertera |
|---|---|---|---|
| GET detail barang ID 28 | Lulus | 200 OK dan data barang ditampilkan | 200 OK, `{"id":28,"nama":"Barang Tes CRUD","stok":10,"harga":"Rp150.000,50"}` |
| GET barang dengan ID 99999 | Lulus | 404 Not Found | 404 Not Found, `{"status":"error","message":"Barang dengan ID 99999 tidak ditemukan"}` |
| GET semua barang | Lulus | 200 OK dan daftar barang ditampilkan dalam JSON array | 200 OK, 10 barang ditampilkan, termasuk `Barang Tes CRUD` dengan ID 28 |

## 4.5 Pengujian POST Barang

| Pengujian | Status | Hasil yang Diharapkan | Hasil yang Tertera |
|---|---|---|---|
| POST barang dengan data valid | Lulus | 201 Created dan data baru dikembalikan | 201 Created, `{"id":28,"nama":"Barang Tes CRUD","stok":10,"harga":"Rp150.000,50"}` |
| POST barang dengan stok negatif | Lulus | 400 Bad Request | 400 Bad Request, `{"status":"error","message":"Stok barang tidak boleh negatif"}` |
| POST barang dengan harga negatif | Lulus | 400 Bad Request | 400 Bad Request, `{"status":"error","message":"Harga barang tidak boleh negatif"}` |
| POST barang dengan nama kosong/spasi | Lulus | 400 Bad Request | 400 Bad Request, `{"status":"error","message":"Nama barang tidak boleh kosong"}` |
| POST barang dengan JSON tidak valid | Lulus | 400 Bad Request | 400 Bad Request, `{"status":"error","message":"Body JSON tidak valid"}` |

## 4.6 Pengujian PUT Barang

| Pengujian | Status | Hasil yang Diharapkan | Hasil yang Tertera |
|---|---|---|---|
| PUT memperbarui seluruh data barang ID 28 | Lulus | 200 OK dan seluruh field diperbarui | 200 OK, `{"id":28,"nama":"Barang Tes PUT","stok":20,"harga":"Rp200.000,00"}` |
| PUT dengan ID 99999 | Lulus | 404 Not Found | 404 Not Found, `{"status":"error","message":"Barang dengan ID 99999 tidak ditemukan"}` |
| PUT dengan stok negatif | Lulus | 400 Bad Request | 400 Bad Request, `{"status":"error","message":"Stok barang tidak boleh negatif"}` |

## 4.7 Pengujian PATCH Barang

| Pengujian | Status | Hasil yang Diharapkan | Hasil yang Tertera |
|---|---|---|---|
| PATCH hanya mengubah stok barang ID 28 | Lulus | 200 OK; stok berubah, nama dan harga tetap | 200 OK, `{"id":28,"nama":"Barang Tes PUT","stok":35,"harga":"Rp200.000,00"}` |
| PATCH tanpa field (`{}`) | Lulus | 400 Bad Request | 400 Bad Request, `{"status":"error","message":"PATCH harus menyertakan minimal satu field untuk diperbarui"}` |
| PATCH dengan stok negatif | Lulus | 400 Bad Request | 400 Bad Request, `{"status":"error","message":"Stok barang tidak boleh negatif"}` |
| PATCH dengan ID 99999 | Lulus | 404 Not Found | 404 Not Found, `{"status":"error","message":"Barang dengan ID 99999 tidak ditemukan"}` |

## 4.8 Pengujian DELETE Barang

| Pengujian | Status | Hasil yang Diharapkan | Hasil yang Tertera |
|---|---|---|---|
| DELETE barang tes ID 28 | Lulus | 200 OK dan barang berhasil dihapus | 200 OK, `{"message":"Berhasil menghapus barang"}` |
| DELETE barang ID 28 yang sudah dihapus | Lulus | 404 Not Found | 404 Not Found, `{"status":"error","message":"Barang dengan ID 28 tidak ditemukan"}` |

## 4.9 Pengujian Validasi dan Error API

| Pengujian | Status | Hasil yang Diharapkan | Hasil yang Tertera |
|---|---|---|---|
| GET dengan parameter ID bukan angka (`abc`) | Lulus | 400 Bad Request | 400 Bad Request, `{"status":"error","message":"Parameter path tidak valid"}` |
| OPTIONS pada endpoint `/barang` | Lulus | 405 Method Not Allowed dan respons error JSON | 405 Method Not Allowed, `{"status":"error","message":"Method tidak diizinkan"}`; header `Allow: GET,HEAD,POST` |
| GET ke endpoint yang tidak terdaftar | Lulus | 404 Not Found dan respons error JSON | 404 Not Found, `{"status":"error","message":"Route tidak ditemukan"}` |

## 4.10 Rekapitulasi Hasil Pengujian

| Bagian | Jumlah Pengujian | Lulus |
|---|---:|---:|
| Register | 4 | 4 |
| Login | 4 | 4 |
| JWT dan Middleware | 3 | 3 |
| GET Barang | 3 | 3 |
| POST Barang | 5 | 5 |
| PUT Barang | 3 | 3 |
| PATCH Barang | 4 | 4 |
| DELETE Barang | 2 | 2 |
| Validasi dan Error API | 3 | 3 |
| **Total** | **31** | **31** |

**Kesimpulan:** Seluruh 31 pengujian manual yang dicatat dalam dokumen ini memperoleh hasil sesuai dengan perilaku API yang diharapkan.
