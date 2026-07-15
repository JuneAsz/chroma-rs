use std::path::Path;

pub fn load_pixels(path: &Path) -> anyhow::Result<image::RgbImage> {
    Ok(image::ImageReader::open(path)?.decode()?.into_rgb8())
}
