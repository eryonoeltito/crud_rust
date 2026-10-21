# 09 — `.gitignore` dan file rahasia

`.gitignore` memberi tahu Git file/folder mana yang tidak perlu dilacak. Contoh aturan:
```gitignore
.env
.env.*
!.env.example
/target/
```
- `.env`: mengabaikan konfigurasi lokal yang dapat berisi password database.
- `.env.*`: mengabaikan variasi seperti `.env.local`.
- `!.env.example`: tetap mengizinkan template tanpa rahasia asli.
- `/target/`: mengabaikan hasil build Rust yang bisa dibuat ulang.

**Mengapa penting?** Jika `.env` ikut dibagikan, orang lain bisa memperoleh kredensial database. Jika `target/` disertakan, ZIP menjadi lebih besar tanpa perlu menyertakan hasil build.

**Jika tidak memakai `.gitignore`?** Git dapat menampilkan file tersebut sebagai file baru yang siap ditambahkan. Itu tidak otomatis mengunggah file, tetapi risiko ikut ter-commit meningkat.

Penting: `.gitignore` hanya memengaruhi Git, bukan otomatis mengecualikan file dari ZIP manual. Saat membuat ZIP, gunakan opsi pengecualian lalu periksa isi arsip dengan `unzip -l nama.zip`. Periksa juga apakah password ditulis langsung di source. Jika rahasia sudah pernah masuk riwayat Git, menghapus file saat ini saja tidak menghapus riwayat lama; ganti/rotasi rahasianya.
