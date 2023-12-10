mod generate;
mod loader;

use crate::generate::generate_mandelbrot_fractal;
use lazy_static::lazy_static;
use rayon::prelude::*;
use std::process::Command;

// Width of the individual segments the large image is built from
static WIDTH: u64 = 2u64.pow(13);
// Max iterations of z^2+c per pixel
static MAX_ITERATIONS: u64 = 256;
// Supersampling factor for SSAA spatial anti-aliasing
static SUPERSAMPLING_FACTOR: u64 = 2;
// Time escape to speed up calculation, changes output of less important bits
static TIME_ESCAPE: f64 = 3.0;
// How many colors will be used
static PALETTE_SIZE: usize = 64;

// How much the image will be seperated into smaller parts
static SUBSEGMENTATION_FACTOR: u64 = 1;
// Format, png8 for 8 bit color space
static COLOR_FORMAT: &str = "png8";

static SCALE: f64 = 0.48;
static CX_SCALE: f64 = SCALE / (WIDTH * SUPERSAMPLING_FACTOR) as f64;
static CY_SCALE: f64 = CX_SCALE;

static MASTER_CX_OFFSET: f64 = -0.75;

lazy_static! {
    static ref MIN_X: f64 = -3.0;
    static ref MAX_X: f64 = 1.0;
    static ref MIN_Y: f64 = -3.0;
    static ref MAX_Y: f64 = 3.0;
}

fn generate_row(row_index: i32, cy_offset: f64, progress_loader: &mut loader::ProgressLoader) {
    let mut counter = 1;
    for i in (MIN_Y.clone() as i32)..(MAX_Y.clone() as i32) * SUBSEGMENTATION_FACTOR as i32 {
        progress_loader.semi_increment();
        generate_mandelbrot_fractal(
            &format!(".cache{}_{}.png", row_index, &counter),
            WIDTH / SUBSEGMENTATION_FACTOR,
            WIDTH / SUBSEGMENTATION_FACTOR,
            -SCALE / SUBSEGMENTATION_FACTOR as f64 * -i as f64 + MASTER_CX_OFFSET,
            -SCALE / SUBSEGMENTATION_FACTOR as f64 * -cy_offset,
        );
        progress_loader.increment();
        counter += 1;
    }
}

fn join_images() {
    (1..(MAX_Y.clone() as i32) * SUBSEGMENTATION_FACTOR as i32 * 2 + 1)
        .into_par_iter()
        .for_each(|i| {
            println!("Joining images for row {}", i);
            Command::new("convert")
                .arg("+append")
                .arg(format!(".cache{}_*.png", i))
                .arg(format!("{}:.cachefin_{}.png", COLOR_FORMAT, i))
                .output()
                .expect("failed to execute process");
        });

    println!("Joining rows");
    Command::new("convert")
        .arg("-append")
        .arg(format!(".cachefin_*.png"))
        .arg(format!("{}:fin.png", COLOR_FORMAT))
        .output()
        .expect("failed to execute process");
}

fn main() {
    if cfg!(windows) {
        std::process::exit(1);
    }

    let mut progress_loader = loader::ProgressLoader::new(6, 6);

    let cy_offset = if SUBSEGMENTATION_FACTOR > 1 {
        MAX_Y.clone() / SUBSEGMENTATION_FACTOR as f64
    } else {
        0.0
    };
    let mut counter = 1;
    for i in (MIN_Y.clone() as i32)..(MAX_Y.clone() as i32) * SUBSEGMENTATION_FACTOR as i32 {
        generate_row(counter, i as f64 + cy_offset, &mut progress_loader);
        counter += 1;
    }
    join_images();
    println!("Done!");
}
