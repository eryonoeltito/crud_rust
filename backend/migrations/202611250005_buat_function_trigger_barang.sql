CREATE FUNCTION public.perbarui_waktu_barang()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $function$
BEGIN
    NEW.updated_at := CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$function$;

CREATE FUNCTION public.catat_perubahan_stok()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $function$
BEGIN
    IF OLD.stok IS DISTINCT FROM NEW.stok THEN
        INSERT INTO public.riwayat_stok (
            barang_id,
            stok_lama,
            stok_baru,
            waktu_perubahan,
            nama_barang,
            gudang_id
        )
        VALUES (
            NEW.id,
            OLD.stok,
            NEW.stok,
            CURRENT_TIMESTAMP,
            NEW.nama,
            NEW.gudang_id
        );
    END IF;

    RETURN NEW;
END;
$function$;

CREATE TRIGGER trigger_perbarui_waktu_barang
BEFORE UPDATE ON public.barang
FOR EACH ROW
EXECUTE FUNCTION public.perbarui_waktu_barang();

CREATE TRIGGER trigger_perubahan_stok
AFTER UPDATE OF stok ON public.barang
FOR EACH ROW
EXECUTE FUNCTION public.catat_perubahan_stok();
