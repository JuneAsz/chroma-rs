use colored::Colorize;

pub fn print_palette(centroids: &[(u8, u8, u8)]) {
    println!("{}", "Extracted Palette".bold());

    for (_, &(r, g, b)) in centroids.iter().enumerate() {
        let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);

        println!(
            "{} {} {}",
            format!("RGB({}, {}, {})", r, g, b)
                .bold()
                .truecolor(r, g, b),
            hex.bold(),
            "████████".truecolor(r, g, b)
        );
    }
}
