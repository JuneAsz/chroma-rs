use clap::Parser;

use crate::args::Args;
use crate::img::load_pixels;
use crate::kmeans::{init_centroids, run};
use crate::print::print_palette;

mod args;
mod img;
mod kmeans;
mod models;
mod print;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let img = load_pixels(&args.path)?;
    let mut centroids = init_centroids(&img, args.k as usize);
    run(img, args.k as usize, args.sample_size, &mut centroids);

    print_palette(&centroids);

    Ok(())
}
