use colored::Colorize;

pub fn print_palette(centroids: &[(u8, u8, u8)]) {
    for &(r, g, b) in centroids {
        let rgb = format!("RGB({:>3}, {:>3}, {:>3})", r, g, b);
        let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);

        println!(
            "{:<21} {:<7} {}",
            rgb.bold().truecolor(r, g, b),
            hex.bold(),
            "████████".truecolor(r, g, b),
        );
    }
}
