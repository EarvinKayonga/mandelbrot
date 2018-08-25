extern crate raster;
#[macro_use]
extern crate failure;

use failure::Error;
use raster::{Color, Image};

fn main() -> Result<(), Error> {
    let output = "out.png";
    let width = 1024;
    let height = 1024;

    let file = create_file(width, height);
    let image = paint_pixels(file);

    save_image(&image, output)
}

fn create_file(width: i32, height: i32) -> Image {
    Image::blank(width, height)
}

fn save_image(image: &Image, path: &str) -> Result<(), Error> {
    raster::save(image, path)
        .map_err(|err| format_err!("an error occured while saving image: {:?}", err))
}

fn paint_pixels(mut image: Image) -> Image {
    let Image { width, height, .. } = image;

    for col in 0..width {
        for line in 0..height {
            let color = calculate_color(col, line, width, height);
            let _ = image
                .set_pixel(col, line, color)
                .map_err(|err| format_err!("an error occured while setting pixels: {:?}", err));
        }
    }

    image
}

fn calculate_color(col: i32, line: i32, width: i32, height: i32) -> Color {
    let complexity = 8192.0;
    let max_i = 20000;

    let (mut x, mut y) = (0.0, 0.0);

    let xi = norm(col, width, -1.0, 2.0);
    let yi = norm(line, height, -1.0, 1.0);

    for _i in 0..max_i {
        if x * x + y * y < complexity {
            x = x * x - y * y + xi;
            y = 2.0 * x * y + yi;
        }
    }

    let color = gray(x as u8);
    color
}

fn gray(scale: u8) -> Color {
    Color::rgba(0, 0, 0, scale)
}

fn norm(x: i32, total: i32, min: f64, max: f64) -> f64 {
    (max - min) * (x as f64) / (total as f64) - max
}
