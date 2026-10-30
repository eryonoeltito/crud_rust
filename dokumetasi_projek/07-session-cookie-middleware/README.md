# 07 — Session, Cookie, dan Middleware

Session menyimpan status login di sisi server. Cookie membawa ID session dari client ke server pada request berikutnya. Middleware memeriksa session sebelum meneruskan request ke handler yang dilindungi. Ketiganya berhubungan, tetapi punya tugas berbeda: session adalah data login, cookie adalah mekanisme pengenal, dan middleware adalah pemeriksa akses.

## Konfigurasi session

Di `src/main.rs`, `SessionManagerLayer` dipasang pada router utama dan menggunakan `MemoryStore`. Store menyimpan data session di memori proses; cookie client berisi ID acak untuk menemukan data itu, bukan username atau password. Dengan penyimpanan memori, session hilang ketika proses server dimulai ulang. Beberapa instance server juga tidak berbagi store memori masing-masing, sehingga deployment multi-instance memerlukan store session bersama.

## Siklus request

```text
Login berhasil → username disimpan ke session → response Set-Cookie
Request berikutnya + Cookie → session layer memuat data → middleware memeriksa username
```

`wajib_login` berada di `src/middleware/auth_middleware.rs`. Middleware mengambil nilai `username` dari session. Jika session memuat username, middleware meneruskan request ke handler dengan `next.run(request)`. Jika tidak, middleware menghentikan request dan mengembalikan `401 Unauthorized` dalam bentuk JSON. Kegagalan membaca session menghasilkan error server, sebab aplikasi tidak dapat memastikan status login.

Middleware diterapkan pada router barang dengan `route_layer`, sehingga semua route dalam router tersebut (daftar, detail, tambah, ubah, dan hapus) melewati pemeriksaan. Route register/login/logout berada di router autentikasi yang digabungkan terpisah, sehingga login tetap dapat diakses sebelum ada session. Saat logout, `session.flush()` mengakhiri session yang tersimpan.

Pada konfigurasi sekarang, cookie tidak mensyaratkan HTTPS (`with_secure(false)`) untuk mendukung pengujian lokal HTTP, dan expiry memakai `OnSessionEnd` (cookie sesi browser). Konfigurasi cookie perlu disesuaikan untuk deployment publik, termasuk HTTPS dan atribut secure. Jangan membagikan cookie session karena siapa pun yang memilikinya dapat menggunakan session tersebut selama masih berlaku. Cookie adalah kredensial aktif, jadi perlakukan seperti rahasia.
