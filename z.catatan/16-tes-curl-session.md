# 16. Pengujian API Session Menggunakan cURL

Dokumentasi ini berisi kumpulan perintah `curl` untuk menguji API Inventaris Rust Axum yang menggunakan session biasa, bukan JWT.

## 1. Persiapan

Pastikan API sedang berjalan. Buka terminal pertama:

```bash
cd ~/inventaris_api
cargo run
```

Jika berhasil, server berjalan di:

```text
http://127.0.0.1:3000
```

Buka terminal kedua untuk menjalankan seluruh pengujian berikut.

**Keterangan:**
- `curl`: mengirim HTTP request ke API.
- `-i`: menampilkan status HTTP dan response header.
- `-X POST`: mengirim request menggunakan method POST.
- `-H`: menambahkan HTTP header.
- `-d`: mengirim data request, biasanya JSON.
- `-c cookies.txt`: menyimpan cookie dari response ke file.
- `-b cookies.txt`: mengirim cookie yang tersimpan dalam file.
- `-s`: menampilkan response tanpa informasi progress transfer.

> Gunakan username dan password yang benar-benar tersedia di tabel `users` milik database kamu. Contoh di bawah menggunakan `admin` dan `admin123`.

---

## 2. Menguji endpoint kesehatan API

Memeriksa apakah API merespons.

```bash
curl -i http://127.0.0.1:3000/health
```

Hasil yang diharapkan: `200 OK` dengan pesan bahwa API berhasil berjalan.

## 3. Menguji akses barang tanpa login

Kirim request tanpa cookie session.

```bash
curl -i http://127.0.0.1:3000/barang
```

Hasil yang diharapkan:

- Status: `401 Unauthorized`
- Pesan: `Silakan login terlebih dahulu`

**Tujuan:** memastikan middleware menolak request yang belum memiliki session login aktif.

## 4. Menguji login dengan username dan password benar

```bash
curl -i -c cookies.txt \
  -X POST http://127.0.0.1:3000/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123"}'
```

Hasil yang diharapkan:

- Status: `200 OK`
- Pesan: `Login berhasil`
- Session: `aktif`
- Header response mengirim cookie session melalui `Set-Cookie`.
- Cookie disimpan ke `cookies.txt`.

**Tujuan:** menguji apakah API menerima kredensial yang benar dan memberikan session kepada klien.

## 5. Menguji login dengan password salah

```bash
curl -i \
  -X POST http://127.0.0.1:3000/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"salah123"}'
```

Hasil yang diharapkan:

- Status: `401 Unauthorized`
- Pesan: `Username atau password salah`

**Tujuan:** memastikan API menolak kredensial yang salah.

Pengujian ini tidak menggunakan `-c cookies.txt` agar cookie lama tidak sengaja ditimpa.

## 6. Mengakses daftar barang setelah login

Setelah login berhasil dan cookie tersimpan, jalankan:

```bash
curl -i -b cookies.txt \
  http://127.0.0.1:3000/barang
```

Hasil yang diharapkan:

- Status: `200 OK`
- Response berisi daftar barang dari database.

**Tujuan:** membuktikan bahwa cookie session dikirim kembali saat mengakses endpoint yang dilindungi.

## 7. Menguji GET detail barang

Ganti `1` dengan ID barang yang tersedia di database.

```bash
curl -i -b cookies.txt \
  http://127.0.0.1:3000/barang/1
```

Hasil yang diharapkan:
- `200 OK` jika barang ditemukan.
- `404 Not Found` jika ID barang tidak ditemukan.

## 8. Menguji POST — menambahkan barang

```bash
curl -i -b cookies.txt \
  -X POST http://127.0.0.1:3000/barang \
  -H "Content-Type: application/json" \
  -d '{"nama":"Mouse Baru","stok":10,"harga":75000}'
```

Hasil yang diharapkan:
- `201 Created` jika barang berhasil ditambahkan.
- Response berisi informasi barang yang dibuat.

Catat ID barang baru dari response. ID tersebut dapat digunakan untuk pengujian PUT, PATCH, GET detail, dan DELETE.

## 9. Menguji PUT — mengganti data barang

Ganti `ID_BARANG` dengan ID barang yang ingin diperbarui.

```bash
curl -i -b cookies.txt \
  -X PUT http://127.0.0.1:3000/barang/ID_BARANG \
  -H "Content-Type: application/json" \
  -d '{"nama":"Mouse Gaming","stok":20,"harga":150000}'
```

PUT digunakan untuk mengganti seluruh field yang dikelola oleh endpoint tersebut, yaitu `nama`, `stok`, dan `harga`.

Hasil yang diharapkan:
- `200 OK` jika pembaruan berhasil.
- `404 Not Found` jika barang tidak ditemukan.

## 10. Menguji PATCH — memperbarui sebagian data

Contoh mengubah stok saja:

```bash
curl -i -b cookies.txt \
  -X PATCH http://127.0.0.1:3000/barang/ID_BARANG \
  -H "Content-Type: application/json" \
  -d '{"stok":25}'
```

Contoh mengubah nama dan harga saja:

```bash
curl -i -b cookies.txt \
  -X PATCH http://127.0.0.1:3000/barang/ID_BARANG \
  -H "Content-Type: application/json" \
  -d '{"nama":"Mouse Wireless","harga":125000}'
```

PATCH digunakan untuk mengubah hanya field yang dikirim. Field lain yang tidak dikirim seharusnya tetap seperti sebelumnya.

## 11. Menguji DELETE — menghapus barang

**Perhatian:** perintah ini benar-benar menghapus data. Gunakan ID barang percobaan yang memang boleh dihapus.

```bash
curl -i -b cookies.txt \
  -X DELETE http://127.0.0.1:3000/barang/ID_BARANG
```

Hasil yang diharapkan:
- `200 OK` jika penghapusan berhasil.
- `404 Not Found` jika barang tidak ditemukan.

## 12. Menguji logout

```bash
curl -i -b cookies.txt \
  -X POST http://127.0.0.1:3000/logout
```

Hasil yang diharapkan:
- Status: `200 OK`
- Pesan: `Logout berhasil`
- Response menghapus atau mengosongkan cookie session pada klien.

## 13. Menguji akses barang setelah logout

```bash
curl -i -b cookies.txt \
  http://127.0.0.1:3000/barang
```

Hasil yang diharapkan:
- Status: `401 Unauthorized`
- Pesan: `Silakan login terlebih dahulu`

**Tujuan:** memastikan session yang sudah di-logout tidak dapat digunakan lagi untuk mengakses endpoint barang.

## 14. Menguji ulang login setelah logout

Login lagi untuk mendapatkan session baru.

```bash
curl -i -c cookies.txt \
  -X POST http://127.0.0.1:3000/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123"}'
```

Kemudian coba akses daftar barang:

```bash
curl -i -b cookies.txt \
  http://127.0.0.1:3000/barang
```

Jika login berhasil, daftar barang seharusnya dapat diakses kembali.

## 15. Menghapus file cookie pengujian

Setelah seluruh pengujian selesai:

```bash
rm -f cookies.txt
```

Perintah tersebut hanya menghapus file `cookies.txt` pada direktori terminal saat ini. Perintah ini bukan pengganti endpoint logout di server.

---

## 16. Urutan pengujian yang disarankan

| Urutan | Pengujian | Hasil utama |
|---|---|---|
| 1 | GET `/health` | `200 OK` |
| 2 | GET `/barang` tanpa login | `401 Unauthorized` |
| 3 | POST `/login` dengan password salah | `401 Unauthorized` |
| 4 | POST `/login` dengan kredensial benar | `200 OK` dan cookie tersimpan |
| 5 | GET `/barang` dengan cookie | `200 OK` |
| 6 | GET detail barang | `200 OK` atau `404` |
| 7 | POST barang | `201 Created` |
| 8 | PUT barang | `200 OK` |
| 9 | PATCH barang | `200 OK` |
| 10 | DELETE barang percobaan | `200 OK` |
| 11 | POST `/logout` | `200 OK` |
| 12 | GET `/barang` setelah logout | `401 Unauthorized` |

## 17. Alur kerja session

```text
Klien mengirim username dan password
                 |
                 v
       POST /login ke Axum
                 |
                 v
      Model memeriksa database
                 |
          +------+------+
          |             |
     Kredensial       Kredensial
        benar           salah
          |               |
          v               v
     Session aktif    HTTP 401
          |
          v
   Cookie disimpan klien
          |
          v
 Cookie dikirim saat request
          |
          v
 Middleware memeriksa session
          |
      +---+---+
      |       |
    Aktif   Tidak aktif
      |       |
      v       v
  Akses API  HTTP 401
      |
      v
   POST /logout
      |
      v
 Session diakhiri
      |
      v
 Request berikutnya ditolak
```

**Kesimpulan:** `cookies.txt` hanya membantu cURL menyimpan dan mengirim cookie. Data session dikelola oleh aplikasi server. Middleware `wajib_login` memeriksa session sebelum mengizinkan request menuju endpoint barang.