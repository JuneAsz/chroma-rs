use std::path::Path;

use image::imageops;

pub fn load_pixels(path: &Path) -> anyhow::Result<image::RgbImage> {
    let image = image::ImageReader::open(path)?.decode()?;

    Ok(image
        .resize(800, 800, imageops::FilterType::Triangle)
        .into_rgb8())
}
