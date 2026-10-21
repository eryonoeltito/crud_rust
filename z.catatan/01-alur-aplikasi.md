# 01 — Alur aplikasi dan MVC

## Gambaran sederhana

```text
Postman / frontend
        ↓ HTTP method + URL + JSON
Routes (memilih handler)
        ↓
Controller (ambil input, validasi, tangani hasil/error)
        ↓
Model (jalankan SQL melalui SQLx)
        ↓
PostgreSQL
        ↑ hasil query
Controller → View (bentuk JSON) → respons HTTP → pengguna
```

### Kenapa dipisah?
- **Routes** fokus memetakan URL dan method ke fungsi.
- **Controller** mengatur alur request: membaca input, validasi, memanggil model, menentukan status respons.
- **Model** berhubungan dengan data dan query SQL.
- **View** menyusun bentuk respons yang dilihat pemakai.

Pemisahan ini membuat perubahan lebih terarah. Misalnya, mengubah format tampilan harga tidak perlu mengubah query SQL.

### Kalau semuanya ditaruh di `main.rs`?
Aplikasi kecil mungkin tetap bisa berjalan, tetapi file utama cepat penuh. Query, validasi, routing, dan format respons bercampur, sehingga lebih sulit dibaca, diuji, dan diperbaiki. MVC bukan syarat agar Rust bisa berjalan; ini cara mengatur kode.

## Contoh alur `GET /barang/5`
1. Client mengirim `GET /barang/5`.
2. Route `/barang/{id}` memilih `detail_barang`.
3. `Path(id): Path<i32>` mengambil `5` dari URL.
4. `State(pool): State<PgPool>` mengambil pool database yang dibagikan aplikasi.
5. Controller memanggil `Barang::berdasarkan_id(&pool, id)`.
6. Model menjalankan `SELECT ... WHERE id = $1` dengan parameter `5`.
7. Jika ditemukan, data dibuat menjadi JSON. Jika tidak, controller mengirim 404.
8. Jika query gagal, controller mencatat detail error di log dan mengirim 500 dengan pesan umum.

## Istilah yang sering ditanya
- **API**: pintu komunikasi agar program lain bisa meminta atau mengubah data.
- **Endpoint**: alamat + method tertentu, misalnya `GET /barang`.
- **Handler**: fungsi yang dipanggil ketika route cocok dengan request.
- **Request body**: data yang dikirim, biasanya JSON pada POST/PUT/PATCH.
- **Response**: jawaban server, berisi status HTTP dan data/pesan.
- **CRUD**: Create, Read, Update, Delete.

### Jika tidak memakai View?
Controller dapat langsung membuat JSON, seperti `detail_barang` pada source saat ini. Itu valid untuk proyek kecil, tetapi jika banyak handler menyusun format sendiri, respons mudah tidak konsisten. View membantu menggunakan format yang sama di beberapa handler.
