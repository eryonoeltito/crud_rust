# Dokumentasi API Inventaris

Dokumentasi proyek dibagi menjadi delapan bagian. Setiap bagian memiliki folder bernomor dan README yang menjelaskan topik tersebut. Susunan ini mengikuti perjalanan memahami aplikasi: konfigurasi dan koneksi terlebih dahulu, kemudian request barang, sampai proses autentikasi.

| Bagian | Topik | Dokumentasi |
|---|---|---|
| 00 | Koneksi Database | [00-koneksi-database/README.md](00-koneksi-database/README.md) |
| 01 | Routes dan Handler | [01-routes-handler/README.md](01-routes-handler/README.md) |
| 02 | Listener | [02-listener/README.md](02-listener/README.md) |
| 03 | CRUD Barang | [03-crud-barang/README.md](03-crud-barang/README.md) |
| 04 | Format Harga | [04-format-harga/README.md](04-format-harga/README.md) |
| 05 | Objek JSON berupa JSON Array | [05-json-array/README.md](05-json-array/README.md) |
| 06 | Login dan Logout | [06-login-logout/README.md](06-login-logout/README.md) |
| 07 | Session, Cookie, dan Middleware | [07-session-cookie-middleware/README.md](07-session-cookie-middleware/README.md) |

## Ringkasan alur aplikasi

Client (misalnya aplikasi frontend atau Postman) mengirim request HTTP ke listener. Router mencocokkan method dan alamat lalu meneruskan request ke handler yang tepat. Untuk route barang, middleware memeriksa session sebelum handler dipanggil. Handler membaca input, memvalidasi, memanggil model untuk mengakses PostgreSQL, dan membentuk respons JSON. Response dikirim kembali melalui koneksi HTTP yang sama.

```text
Client → Listener → Router → Middleware (route barang) → Handler
                                                        ↓
Client ← Respons JSON ← View/handler ← Model ← PostgreSQL
```

`main.rs` menjadi titik awal untuk melihat bagaimana komponen disiapkan dan digabungkan. Folder `controllers`, `models`, `routes`, `views`, dan `middleware` membagi tanggung jawab supaya pembaca bisa menemukan kode yang mengatur satu pekerjaan tertentu. Mulai dari bagian 00 untuk memahami konfigurasi database, lalu ikuti nomor bagian hingga autentikasi.
