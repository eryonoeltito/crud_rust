# Query Aktivasi Petugas Gudang

Query ini digunakan untuk mengaktifkan akun petugas gudang yang sebelumnya masih berstatus pending.

## 1. Aktivasi Petugas Gudang 1

```sql
BEGIN;

UPDATE public.users
SET
    role = 'petugas_gudang',
    gudang_id = 1,
    aktif = TRUE
WHERE username = 'petugas_gudang1'
  AND aktif = FALSE;

COMMIT;
```

## 2. Aktivasi Petugas Gudang 2

```sql
BEGIN;

UPDATE public.users
SET
    role = 'petugas_gudang',
    gudang_id = 2,
    aktif = TRUE
WHERE username = 'petugas_gudang2'
  AND aktif = FALSE;

COMMIT;
```

## 3. Aktivasi Petugas Gudang 3

```sql
BEGIN;

UPDATE public.users
SET
    role = 'petugas_gudang',
    gudang_id = 3,
    aktif = TRUE
WHERE username = 'petugas_gudang3'
  AND aktif = FALSE;

COMMIT;
```

## 4. Memeriksa Status Semua Petugas

```sql
SELECT
    id,
    username,
    role,
    gudang_id,
    aktif
FROM public.users
WHERE role = 'petugas_gudang'
ORDER BY gudang_id;
```

## 5. Menjalankan Query melalui Docker PostgreSQL

Simpan query yang diperlukan, kemudian jalankan melalui:

```bash
sudo docker exec -i postgresql psql \
  -X \
  -v ON_ERROR_STOP=1 \
  -U tito \
  -d inventaris_api
```

## Catatan Penting

- `role` menentukan jenis akses pengguna.
- `gudang_id` menentukan gudang yang boleh diakses.
- `aktif = TRUE` mengizinkan akun yang sudah ditetapkan untuk login.
- Setelah aktivasi atau perubahan role/gudang, pengguna harus login ulang untuk mendapatkan JWT baru.
- Query di atas hanya mengaktifkan akun yang masih `aktif = FALSE`. Jika akun sudah aktif, query tidak mengubahnya.