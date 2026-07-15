use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub path: PathBuf,

    #[arg(short, long, default_value_t = 20)]
    pub sample_size: u32,

    #[arg(short, default_value_t = 20)]
    pub k: u16,
}
