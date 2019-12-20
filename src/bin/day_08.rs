use std::convert::From;

fn main() {
    let s = std::fs::read_to_string("src/inputs/day_08").expect("Failed to read input file");

    let image = Image::new(25, 6, &s.trim()).unwrap();

    let b = image
        .layers
        .iter()
        .map(|l| l.count_color(Color::Black))
        .enumerate()
        .map(|(a, b)| (b, a))
        .min();

    let idx = b.unwrap().1;

    let part_1 = image.layers[idx].count_color(Color::White)
        * image.layers[idx].count_color(Color::Transparent);

    println!("Part 1: {}\n", part_1);


    let decoded = image.decode().message();

    for i in 0..6 {
        for j in 0..25 {
            if decoded[i*25 + j] == 0 {
                print!(" ");
            } else {
                print!("X");
            }
        }
        println!();
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Color {
    Black,
    White,
    Transparent,
}

impl From<Color> for u8 {
    fn from(color: Color) -> Self {
        match color {
            Color::Black => 0,
            Color::White => 1,
            Color::Transparent => 2,
        }
    }
}

impl From<&Color> for u8 {
    fn from(color: &Color) -> Self {
        match color {
            Color::Black => 0,
            Color::White => 1,
            Color::Transparent => 2,
        }
    }
}

impl From<&Color> for char {
    fn from(color: &Color) -> Self {
        match color {
            Color::Black => '0',
            Color::White => '1',
            Color::Transparent => '2',
        }
    }
}

#[derive(Debug, Clone)]
struct ImageLayer {
    pixels: Vec<Color>,
}

struct Image {
    width: usize,
    height: usize,
    layers: Vec<ImageLayer>,
}

impl ImageLayer {
    fn message(&self) -> Vec<u8> {
        self.pixels.iter().map(u8::from).collect()
    }

    fn count_color(&self, color: Color) -> usize {
        self.pixels.iter().filter(|c| **c == color).count()
    }

    fn new(width: usize, height: usize) -> ImageLayer {
        ImageLayer {
            pixels: vec![Color::Transparent; width * height],
        }
    }

    fn stack(&mut self, other: &ImageLayer) {
        assert!(self.pixels.len() == other.pixels.len());

        for i in 0..self.pixels.len() {
            if other.pixels[i] != Color::Transparent {
                self.pixels[i] = other.pixels[i].clone();
            }
        }
    }
}

impl Image {
    fn decode(&self) -> ImageLayer {
        let mut decoded = ImageLayer::new(self.width, self.height);

        self.layers.iter().rev().for_each(|layer| decoded.stack(layer));

        decoded
    }

    fn new(width: usize, height: usize, pixel_data: &str) -> Result<Image, String> {
        let pixels = pixel_data
            .chars()
            .map(|c| match c {
                '0' => Ok(Color::Black),
                '1' => Ok(Color::White),
                '2' => Ok(Color::Transparent),
                _ => Err(format!("Could not map '{}' to a color", c).to_string()),
            })
            .collect::<Result<Vec<Color>, String>>()?;

        let n_pixels = width * height;
        let n_layers = pixels.len() / n_pixels;

        if n_pixels * n_layers != pixels.len() {
            return Err(format!(
                "Pixel data does not contain a whole number of layers (w:{} x h:{}, N: {})",
                width,
                height,
                pixel_data.len()
            )
            .to_string());
        }

        let mut layers = vec![ImageLayer::new(width, height); n_layers];
        for i in 0..n_layers {
            layers[i]
                .pixels
                .clone_from_slice(&pixels[(i * n_pixels)..(i * n_pixels + n_pixels)]);
        }

        Ok(Image {
            width,
            height,
            layers,
        })
    }
}

#[cfg(test)]
mod day_07 {
    use super::*;


}
