use num::complex::Complex;
use rayon::prelude::*;

use crate::{
    CX_SCALE, CY_SCALE, MAX_ITERATIONS, MAX_X, MAX_Y, MIN_X, MIN_Y, PALETTE_SIZE,
    SUPERSAMPLING_FACTOR, TIME_ESCAPE,
};
use image::*;
use std::path::Path;

pub fn generate_mandelbrot_fractal(
    name: &str,
    width: u64,
    height: u64,
    cx_offset: f64,
    cy_offset: f64,
) {
    let palette = create_palette();

    // Create a flat buffer to store pixel colors
    let mut pixel_buffer = vec![Rgb([0u8, 0u8, 0u8]); (width * height) as usize];

    pixel_buffer
        .par_chunks_mut(width as usize)
        .enumerate()
        .for_each(|(y, row)| {
            for x in 0..width as usize {
                let mut total_color: [u64; 3] = [0, 0, 0];

                // Sample multiple points within the pixel for anti-aliasing
                for sub_x in 0..SUPERSAMPLING_FACTOR {
                    for sub_y in 0..SUPERSAMPLING_FACTOR {
                        let cx =
                            (x as u64 * SUPERSAMPLING_FACTOR + sub_x) as f64 * CX_SCALE + cx_offset;
                        let cy =
                            (y as u64 * SUPERSAMPLING_FACTOR + sub_y) as f64 * CY_SCALE + cy_offset;

                        // Is inside bounding box?
                        if cx >= *MIN_X && cx <= *MAX_X && cy >= *MIN_Y && cy <= *MAX_Y {
                            // Perform the iteration
                            let c = Complex::new(cx, cy);
                            let mut z = Complex::new(0.0, 0.0);
                            let mut i = 0;

                            while z.norm() <= TIME_ESCAPE && i < MAX_ITERATIONS {
                                z = z * z + c;
                                i += 1;
                            }

                            let color_index = if i == MAX_ITERATIONS {
                                Rgb([0, 0, 0]) // Black for points inside the set
                            } else {
                                palette[i as usize % palette.len()] // Color from the palette for points outside the set
                            };

                            // Accumulate the color components
                            total_color[0] += color_index.0[0] as u64;
                            total_color[1] += color_index.0[1] as u64;
                            total_color[2] += color_index.0[2] as u64;
                        }
                    }
                }

                // Average the color components
                let num_samples = SUPERSAMPLING_FACTOR * SUPERSAMPLING_FACTOR;
                total_color[0] /= num_samples;
                total_color[1] /= num_samples;
                total_color[2] /= num_samples;

                // Map the averaged color to RGB values
                let color = Rgb([
                    total_color[0] as u8,
                    total_color[1] as u8,
                    total_color[2] as u8,
                ]);
                row[x] = color;
            }
        });

    // Save the pixel buffer as an image
    let imgbuf: ImageBuffer<Rgb<u8>, _> =
        ImageBuffer::from_fn(width as u32, height as u32, |x, y| {
            pixel_buffer[(y as u64 * width + x as u64) as usize]
        });

    let path = Path::new(name);
    let _ = imgbuf.save(&path);
}

fn create_palette() -> Vec<Rgb<u8>> {
    (0..PALETTE_SIZE)
        .into_par_iter()
        .map(|i| {
            let phase = (i as f64 / PALETTE_SIZE as f64) * 6.0;
            let (r, g, b) = match phase {
                0.0..=1.0 => (255, (phase * 255.0) as u8, 0),
                1.0..=2.0 => (((2.0 - phase) * 255.0) as u8, 255, 0),
                2.0..=3.0 => (0, 255, ((phase - 2.0) * 255.0) as u8),
                3.0..=4.0 => (0, ((4.0 - phase) * 255.0) as u8, 255),
                4.0..=5.0 => (((phase - 4.0) * 255.0) as u8, 0, 255),
                _ => (255, 0, ((6.0 - phase) * 255.0) as u8),
            };
            Rgb([r, g, b])
        })
        .collect()
}
