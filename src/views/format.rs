use rust_decimal::Decimal;

pub fn format_nominal(harga: Decimal) -> String {
    let nominal = format!("{harga:.2}");
    let (angka, desimal) = nominal
        .split_once('.')
        .expect("format harga selalu menyertakan dua angka desimal");
    let (tanda, angka) = angka
        .strip_prefix('-')
        .map_or(("", angka), |angka| ("-", angka));

    let mut angka_terbalik = String::new();
    for (i, digit) in angka.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            angka_terbalik.push('.');
        }
        angka_terbalik.push(digit);
    }
    let angka_format = angka_terbalik.chars().rev().collect::<String>();

    format!("{tanda}Rp{angka_format},{desimal}")
}
