use image::codecs::png::PngEncoder;
use image::{ExtendedColorType, ImageEncoder, ImageError};
use num::Complex;
use std::fs::File;
use std::usize;

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex {re: 0.0, im: 0.0};
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    
        z = z * z + c;
    }

    None
}

fn pixel_to_complex_number(
    image_dimension: (usize, usize), 
    pixel_coordinate: (usize, usize),
    complex_upper_left: Complex<f64>,
    complex_right_bottom: Complex<f64>
) -> Complex<f64> {
    let complex_plane_width = complex_right_bottom.re - complex_upper_left.re;
    let complex_plane_height = complex_upper_left.im - complex_right_bottom.im;

    Complex { re: complex_upper_left.re 
        + (pixel_coordinate.0 as f64 / image_dimension.0 as f64) * complex_plane_width as f64, 
        im: complex_upper_left.im 
        - (pixel_coordinate.1 as f64 / image_dimension.1 as f64) * complex_plane_height as f64
    }
}

fn render(
    pixels: &mut [u8],
    image_dimension: (usize, usize),
    complex_upper_left: Complex<f64>,
    complex_right_bottom: Complex<f64>
) {
    assert!(pixels.len() == image_dimension.0 * image_dimension.1);
    for row in 0..image_dimension.1 {
        for column in 0..image_dimension.0 {
            let complex_number = pixel_to_complex_number(
                image_dimension, 
                (column, row), 
                complex_upper_left, 
                complex_right_bottom
            );
        
        pixels[row * image_dimension.0 + column] = match escape_time(complex_number, 255) {
            None => 0,
            Some(count) => 255 - count as u8
            }
        }
    }
}

fn write_png(file_name: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), ImageError> {
    let output = File::create(file_name)?;
    let encoder = PngEncoder::new(output);
    encoder.write_image(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ExtendedColorType::L8
    )?;

    Ok(())
}

pub fn run() {
    let image_dimension = (3840, 2160);
    let mut pixels = vec![0; image_dimension.0 * image_dimension.1];
    
    let upper_left = Complex {
        re: -1.20,
        im: 0.35
    };

    let right_bottom = Complex {
        re: -1.0,
        im: 0.20
    };

    render(&mut pixels, image_dimension, upper_left, right_bottom);

    write_png("mandelbrot.png", &pixels, image_dimension)
    .expect("error writing png");
}