CREATE TABLE public.users (
    id BIGSERIAL NOT NULL,
    username VARCHAR(50) NOT NULL,
    password VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    role VARCHAR(30),
    gudang_id BIGINT,
    aktif BOOLEAN NOT NULL DEFAULT FALSE,
    CONSTRAINT users_pkey PRIMARY KEY (id),
    CONSTRAINT users_username_key UNIQUE (username),
    CONSTRAINT users_gudang_id_fkey
        FOREIGN KEY (gudang_id) REFERENCES public.gudang(id) ON DELETE RESTRICT,
    CONSTRAINT users_role_valid_check
        CHECK (role IS NULL OR role IN ('admin', 'petugas_gudang')),
    CONSTRAINT users_role_gudang_check
        CHECK (
            COALESCE(
                (role = 'admin' AND gudang_id IS NULL AND aktif = TRUE)
                OR (role = 'petugas_gudang' AND gudang_id IS NOT NULL AND aktif = TRUE)
                OR (role IS NULL AND gudang_id IS NULL AND aktif = FALSE),
                FALSE
            )
        )
);

CREATE INDEX users_gudang_id_idx ON public.users USING btree (gudang_id);
