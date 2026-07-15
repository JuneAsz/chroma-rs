use std::ops::Add;

#[derive(Copy, Clone, Debug)]
pub struct PixelColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Default, Clone, Debug)]
pub struct ColorAccumulation {
    pub reds: u64,
    pub greens: u64,
    pub blues: u64,
    pub pixel_count: u64,
}

impl Add for ColorAccumulation {
    type Output = ColorAccumulation;

    fn add(self, other: ColorAccumulation) -> ColorAccumulation {
        ColorAccumulation {
            reds: self.reds + other.reds,
            greens: self.greens + other.greens,
            blues: self.blues + other.blues,
            pixel_count: self.pixel_count + other.pixel_count,
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct WorkerResult {
    pub accumulators: Vec<ColorAccumulation>,
    pub farthest_pixel: (u8, u8, u8),
    pub farthest_distance: u32,
}
