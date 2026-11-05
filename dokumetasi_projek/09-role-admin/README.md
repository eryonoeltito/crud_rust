# 09 — Role Admin

Admin adalah pengguna aktif dengan role `admin`. Admin bertanggung jawab mengelola inventaris lintas gudang dan memastikan akun petugas memiliki role, gudang tugas, serta status aktif yang benar. Middleware membaca status akun, role, dan gudang dari database setiap kali route terlindungi digunakan.

## Tujuan Pembelajaran

- Menjelaskan tanggung jawab admin pada pengelolaan inventaris beberapa gudang.
- Mengidentifikasi hak akses admin pada data barang dan riwayat stok.
- Menjelaskan pengaturan role, gudang, dan status aktif akun petugas serta batas API saat ini.
- Menelusuri contoh alur admin dari login sampai memeriksa riwayat stok.

## Hak akses admin

Route barang dan riwayat stok dilindungi JWT. Setelah token lolos validasi, middleware mengambil akun aktif dari database dan meneruskan `AkunTerverifikasi` kepada handler. Handler memberi akses admin untuk membaca, membuat, mengubah, dan menghapus barang di semua gudang.

JWT memuat claim `role: "admin"` sebagai informasi role saat token diterbitkan. Middleware tetap mengambil role dan gudang terbaru dari database untuk menentukan hak akses. Karena itu, perubahan role atau status akun di database berlaku pada request berikutnya meski token masih memuat nilai lama.

| Fitur | Akses admin |
|---|---|
| Daftar dan detail barang | Melihat barang dari semua gudang |
| Membuat barang | Memilih gudang dengan mengirim `gudang_id` |
| Mengubah atau menghapus barang | Berlaku untuk barang dari semua gudang |
| `GET /riwayat-stok` | Melihat seluruh riwayat, termasuk baris dengan `barang_id` bernilai `NULL` |
| `GET /barang/{id}/riwayat-stok` | Melihat riwayat barang tersebut dari semua gudang |

Endpoint barang yang tersedia adalah `GET /barang`, `GET /barang/{id}`, `POST /barang`, `PUT /barang/{id}`, `PATCH /barang/{id}`, dan `DELETE /barang/{id}`. Kedua endpoint riwayat adalah `GET /riwayat-stok` dan `GET /barang/{id}/riwayat-stok`.

## Pengelolaan akun petugas

Admin perlu memastikan akun petugas memiliki role `petugas_gudang`, `gudang_id` yang sesuai, dan status aktif. Middleware hanya meneruskan akun aktif dengan kombinasi role dan gudang yang valid.

API saat ini tidak menyediakan endpoint untuk mengubah role, menetapkan gudang, atau mengaktifkan akun. `POST /register` hanya menerima username dan password; pengaturan akses akun perlu dilakukan melalui proses administrasi database di luar endpoint tersebut. Jangan menganggap body request petugas atau isi JWT sebagai sumber role dan gudang.

## Contoh alur penggunaan

1. Admin login melalui `POST /login` dan menerima Bearer token jika akunnya aktif.
2. Admin meminta `GET /barang` dengan token untuk melihat barang dari semua gudang.
3. Admin menambahkan barang ke gudang tertentu dengan `POST /barang` dan `gudang_id` yang valid.
4. Admin meminta `GET /riwayat-stok` untuk meninjau seluruh perubahan stok, termasuk riwayat yang tidak terkait dengan barang (`barang_id = NULL`).

Contoh body untuk membuat barang:

```json
{
  "nama": "Kabel HDMI",
  "stok": 12,
  "harga": 75000,
  "gudang_id": 2
}
```

Request barang dan riwayat harus menyertakan header `Authorization: Bearer <token-admin>`. Tanpa token yang valid atau jika akun tidak lagi aktif, akses ditolak.
