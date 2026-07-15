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

pub fn find_closest_centroid(pixel: PixelColor, centroids: &[(u8, u8, u8)]) -> (usize, u32) {
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
    (closest_index, min_distance)
}

pub fn run_worker_bytes(bytes: &[u8], k: usize, centroids: &[(u8, u8, u8)]) -> WorkerResult {
    let mut accumulators = vec![ColorAccumulation::default(); k];

    let mut farthest_pixel = (0, 0, 0);
    let mut farthest_distance = 0;

    for rgb in bytes.chunks_exact(3) {
        let pixel = PixelColor {
            r: rgb[0],
            g: rgb[1],
            b: rgb[2],
        };

        let (index, distance) = find_closest_centroid(pixel, centroids);
        if distance > farthest_distance {
            farthest_distance = distance;
            farthest_pixel = (rgb[0], rgb[1], rgb[2]);
        }

        let bucket = &mut accumulators[index];
        bucket.reds += rgb[0] as u64;
        bucket.greens += rgb[1] as u64;
        bucket.blues += rgb[2] as u64;
        bucket.pixel_count += 1;
    }

    WorkerResult {
        accumulators,
        farthest_pixel,
        farthest_distance,
    }
}

pub fn run(img: image::RgbImage, k: usize, iterations: u32, centroids: &mut Vec<(u8, u8, u8)>) {
    assert_eq!(k, centroids.len(), "k must match the number of centroids");

    let pixels = img.as_raw();

    const PIXELS_PER_CHUNK: usize = 4096;
    const BYTES_PER_CHUNK: usize = PIXELS_PER_CHUNK * 3;

    for _ in 0..iterations {
        let workers: Vec<_> = pixels
            .par_chunks(BYTES_PER_CHUNK)
            .map(|chunk| run_worker_bytes(chunk, k, centroids.as_slice()))
            .collect();

        let mut merged_accumulators = vec![ColorAccumulation::default(); k];

        let mut farthest_pixel = (0u8, 0u8, 0u8);
        let mut farthest_distance = 0u32;

        for worker in workers {
            for (merged, worker_bucket) in merged_accumulators
                .iter_mut()
                .zip(worker.accumulators.iter())
            {
                merged.reds += worker_bucket.reds;
                merged.greens += worker_bucket.greens;
                merged.blues += worker_bucket.blues;
                merged.pixel_count += worker_bucket.pixel_count;
            }

            if worker.farthest_distance > farthest_distance {
                farthest_distance = worker.farthest_distance;
                farthest_pixel = worker.farthest_pixel;
            }
        }

        for (centroid, bucket) in centroids.iter_mut().zip(merged_accumulators.iter()) {
            if bucket.pixel_count > 0 {
                *centroid = (
                    (bucket.reds / bucket.pixel_count) as u8,
                    (bucket.greens / bucket.pixel_count) as u8,
                    (bucket.blues / bucket.pixel_count) as u8,
                );
            } else {
                *centroid = farthest_pixel;
            }
        }
    }
}
