pub fn format_nominal(harga: f64) -> String {
        let nominal = format!("{:.2}", harga);
        let bagian: Vec<&str> = nominal.split('.').collect();

        let angka = bagian[0];
        let desimal = bagian[1];

        let mut hasil = String::new();

        for (i, digit) in angka.chars().rev().enumerate() {
            if i > 0 && i % 3 == 0 {
                hasil.push('.');
            }

            hasil.push(digit);
        }

        let angka_format: String = hasil.chars().rev().collect();

        format!("Rp{},{}", angka_format, desimal)
    }