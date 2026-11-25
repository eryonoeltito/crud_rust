CREATE TABLE public.riwayat_stok (
    id SERIAL NOT NULL,
    barang_id INTEGER,
    stok_lama INTEGER NOT NULL,
    stok_baru INTEGER NOT NULL,
    waktu_perubahan TIMESTAMP WITHOUT TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    nama_barang TEXT NOT NULL,
    gudang_id BIGINT NOT NULL,
    CONSTRAINT riwayat_stok_pkey PRIMARY KEY (id),
    CONSTRAINT riwayat_stok_stok_lama_check CHECK (stok_lama >= 0),
    CONSTRAINT riwayat_stok_stok_baru_check CHECK (stok_baru >= 0),
    CONSTRAINT riwayat_stok_barang_id_fkey
        FOREIGN KEY (barang_id) REFERENCES public.barang(id) ON DELETE SET NULL,
    CONSTRAINT riwayat_stok_gudang_id_fkey
        FOREIGN KEY (gudang_id) REFERENCES public.gudang(id) ON DELETE RESTRICT
);
