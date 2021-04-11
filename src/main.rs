use std::fmt;

mod vec3;
use crate::vec3::Color;

struct PPMImage {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl PPMImage {
    fn new(width: usize, height: usize, pixels: Vec<Color>) -> Self {
        Self {
            width,
            height,
            pixels,
        }
    }
}

impl fmt::Display for PPMImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P3\n{} {}\n255\n", self.width, self.height)?;

        let mut count = 0;
        for pixel in &self.pixels {
            let rgb = pixel.to_rgb();
            write!(f, "{} {} {}", rgb.0, rgb.1, rgb.2).unwrap();
            if count >= 6 {
                write!(f, "\n")?;
                count = 0;
            } else {
                write!(f, " ")?;
                count += 1;
            }
        }

        write!(f, "\n")
    }
}

const IMAGE_WIDTH: usize = 256;
const IMAGE_HEIGHT: usize = 256;

fn main() {
    let mut colors: Vec<Color> = Vec::with_capacity(IMAGE_WIDTH * IMAGE_HEIGHT);

    for j in (0..=IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );

            colors.push(color);
        }
    }

    let image = PPMImage::new(IMAGE_WIDTH, IMAGE_HEIGHT, colors);
    println!("{}", image);
}
