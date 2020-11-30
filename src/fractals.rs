use image;
use rand::{thread_rng, Rng};

use crate::color;
use crate::settings::FractalSettings;

pub struct Mandelbrot {
    settings: FractalSettings
}

impl Mandelbrot {
    pub fn new(settings: FractalSettings) -> Self {
        Self {
            settings: settings
        }
    }

    pub fn save_img(&self) {
        let settings = &self.settings;
        let buf_size = (settings.width * settings.height * 3) as usize;
        let mut buf: Vec<u8> = Vec::with_capacity(buf_size);
        buf.resize(buf_size, 0);
        self.alg1(&mut buf);
        image::save_buffer(&settings.image_name, &buf, settings.width, settings.height, image::ColorType::Rgb8).unwrap();
    }

    fn alg1(&self, buf: &mut Vec<u8>) {
        let settings = &self.settings;
        let center = &settings.center;
        let mut rng = thread_rng();

        for y in 0..settings.height {

            for x in 0..settings.width {

                // Choose at random 50 points in the open box of 1*scale around
                //      scale*(x,y) + center
                // This ensures that we are only sampling points around just the pixel
                // in question -- no sample overlap with pixels adjacent to (x,y)
                let (sum_r, sum_g, sum_b) = (0..settings.sample_size)
                    .fold((0,0,0), |(c_r, c_g, c_b), _| {
                        let nx = settings.scale *
                            (((x as f64) + rng.gen_range(-0.5, 0.5)) /
                            (settings.width as f64)) + center.re;

                        let ny = settings.scale *
                            (((y as f64) + rng.gen_range(-0.5, 0.5)) /
                            (settings.height as f64)) + center.im;

                        // Compute membership in mandelbrot set
                        let (z, tries) = self.mandelbrot_iter(nx, ny);
                        let (r, g, b) = Self::paint(z, tries);
                        (c_r + r as u32, c_g + g as u32, c_b + b as u32)
                    });

                let r = ((sum_r as f64)  / (settings.sample_size as f64)) as u8;
                let g = ((sum_g as f64) / (settings.sample_size as f64)) as u8;
                let b = ((sum_b as f64) / (settings.sample_size as f64)) as u8;

                let offset = (settings.width * 3 * y) + (x * 3);
                buf[offset as usize] = r;
                buf[(offset + 1) as usize] = g;
                buf[(offset + 2) as usize] = b;
            }
        }
    }

    fn paint(r: f64, n: u32) -> (u8, u8, u8) {
        if  r > 4. {
            return color::hsl_to_rgb(n as f64 / 800. * r, 1., 0.5);
        } else {
            return (255, 255, 255);
        }
    }

    fn mandelbrot_iter(&self, re: f64, im: f64) -> (f64, u32) {
        let (mut x, mut y, mut xx, mut yy) = (0., 0., 0., 0.);
        let mut xy;
        let settings = &self.settings;

        // z_n
        for i in 0..settings.max_tries {
            xx = x * x;
            yy = y * y;
            xy = x * y;
            if xx + yy > 4. {
                return (xx + yy, i);
            }
            x = xx - yy + re;
            y = 2. * xy + im;
        }

        return (xx + yy, settings.max_tries);
    }
}
