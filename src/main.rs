mod settings;
mod fractals;
mod cli;
mod math;
mod color;

use fractals::Mandelbrot;

fn main() {
    Mandelbrot::new(cli::generate_settings())
        .save_img();
}
