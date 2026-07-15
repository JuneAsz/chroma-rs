use crate::models::{ColorAccumulation, PixelColor, WorkerResult};
use rayon::prelude::*;
pub fn init_centroids(img: &image::RgbImage, k: usize) -> Vec<(u8, u8, u8)> {
    let total = (img.width() * img.height()) as usize;
    let step = (total / k).max(1);
    img.pixels()
        .step_by(step)
        .take(k)
        .map(|p| (p.0[0], p.0[1], p.0[2]))
        .collect()
}
pub fn calculate_distance(p1: PixelColor, p2: PixelColor) -> u32 {
    let r = p1.r as i32 - p2.r as i32;
    let g = p1.g as i32 - p2.g as i32;
    let b = p1.b as i32 - p2.b as i32;
    (r * r + g * g + b * b) as u32
}
pub fn find_closest_centroid(pixel: PixelColor, centroids: &[(u8, u8, u8)]) -> usize {
    let mut min_distance = u32::MAX;
    let mut closest_index = 0;
    for (index, &centroid) in centroids.iter().enumerate() {
        let centroid_pixel = PixelColor {
            r: centroid.0,
            g: centroid.1,
            b: centroid.2,
        };
        let distance = calculate_distance(pixel, centroid_pixel);
        if distance < min_distance {
            min_distance = distance;
            closest_index = index;
        }
    }
    closest_index
}
pub fn run_worker(pixels: &[image::Rgb<u8>], k: usize, centroids: &[(u8, u8, u8)]) -> WorkerResult {
    let mut accumulators: Vec<ColorAccumulation> = vec![Default::default(); k];
    let mut farthest_pixel = (0u8, 0u8, 0u8);
    let mut farthest_distance = 0;
    for pixel in pixels {
        let p = PixelColor {
            r: pixel.0[0],
            g: pixel.0[1],
            b: pixel.0[2],
        };
        let accumulator_index = find_closest_centroid(p, centroids);
        let centroid_pixel = PixelColor {
            r: centroids[accumulator_index].0,
            g: centroids[accumulator_index].1,
            b: centroids[accumulator_index].2,
        };
        let dist = calculate_distance(p, centroid_pixel);
        if dist > farthest_distance {
            farthest_distance = dist;
            farthest_pixel = (p.r, p.g, p.b);
        }
        accumulators[accumulator_index].reds += p.r as u64;
        accumulators[accumulator_index].greens += p.g as u64;
        accumulators[accumulator_index].blues += p.b as u64;
        accumulators[accumulator_index].pixel_count += 1;
    }
    WorkerResult {
        accumulators,
        farthest_pixel,
        farthest_distance: farthest_distance,
    }
}
pub fn run(img: image::RgbImage, k: usize, iterations: u32, centroids: &mut Vec<(u8, u8, u8)>) {
    let pixel_slice: Vec<image::Rgb<u8>> = img.pixels().copied().collect();
    for _ in 0..iterations {
        let workers: Vec<WorkerResult> = pixel_slice
            .par_chunks(10000)
            .map(|chunk| run_worker(chunk, k, centroids))
            .collect();
        let mut merged_accumulators: Vec<ColorAccumulation> = vec![Default::default(); k];
        let mut farthest_pixel = (0u8, 0u8, 0u8);
        let mut farthest_distance = 0u32;
        for worker in workers {
            for i in 0..k {
                merged_accumulators[i].reds += worker.accumulators[i].reds;
                merged_accumulators[i].greens += worker.accumulators[i].greens;
                merged_accumulators[i].blues += worker.accumulators[i].blues;
                merged_accumulators[i].pixel_count += worker.accumulators[i].pixel_count;
            }
            if worker.farthest_distance > farthest_distance {
                farthest_distance = worker.farthest_distance;
                farthest_pixel = worker.farthest_pixel;
            }
        }
        for i in 0..k {
            let bucket = &merged_accumulators[i];
            if bucket.pixel_count > 0 {
                let new_r = (bucket.reds / bucket.pixel_count) as u8;
                let new_g = (bucket.greens / bucket.pixel_count) as u8;
                let new_b = (bucket.blues / bucket.pixel_count) as u8;
                centroids[i] = (new_r, new_g, new_b);
            } else {
                centroids[i] = farthest_pixel;
            }
        }
    }
}
