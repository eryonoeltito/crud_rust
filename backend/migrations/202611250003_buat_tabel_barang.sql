CREATE TABLE public.barang (
    id SERIAL NOT NULL,
    nama VARCHAR(100) NOT NULL,
    stok INTEGER NOT NULL DEFAULT 0,
    harga NUMERIC(12, 2) NOT NULL,
    created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    gudang_id BIGINT NOT NULL,
    CONSTRAINT barang_pkey PRIMARY KEY (id),
    CONSTRAINT barang_stok_check CHECK (stok >= 0),
    CONSTRAINT barang_harga_check CHECK (harga >= 0),
    CONSTRAINT barang_gudang_id_fkey
        FOREIGN KEY (gudang_id) REFERENCES public.gudang(id) ON DELETE RESTRICT
);

CREATE INDEX barang_gudang_id_idx ON public.barang USING btree (gudang_id);
