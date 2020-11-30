use crate::settings;
use settings::FractalSettings;
use crate::math::Complex;

use clap::{App,Arg};
use std::str::FromStr;

pub fn generate_settings() -> FractalSettings {
    let app = App::new("frctl")
        .version("0.1.0")
        .about("Generate mandelbrot fractals")
        .author("Tyler Ware")
        .args_from_usage("-s, --scale [scale] 'How much should the unit square be scaled?'
                            -w, --width [width] 'The width of the generated image.'
                            --max-tries [tries] 'The maximum amount of times we try to determine if a complex number is in the Mandelbrot set.'
                            -h, --height [height] 'The height of the generated image.'
                            --sample-size [sample-size] 'The number of samples for a pixel. The larger this number the longer the image takes, but the better the quality.'")
        .arg(Arg::with_name("re")
                .long("real")
                .short("r")
                .value_name("re")
                .allow_hyphen_values(true)
                .takes_value(true)
                .help("The real part (x) of the complex number c -- Re(c)."))
        .arg(Arg::with_name("im")
                .long("imaginary")
                .short("i")
                .value_name("im")
                .allow_hyphen_values(true)
                .takes_value(true)
                .help("The imaginary part (y) of the complex number c -- Im(c)"))
        .arg(Arg::with_name("image-name")
            .required(true));


    let matches = app.get_matches();
    FractalSettings {
        width: u32::from_str(matches.value_of("width").unwrap_or(""))
            .unwrap_or(settings::DEFAULT_WIDTH),
        height: u32::from_str(matches.value_of("height").unwrap_or(""))
            .unwrap_or(settings::DEFAULT_HEIGHT),
        center: Complex {
            re: f64::from_str(matches.value_of("re").unwrap_or(""))
                .unwrap_or(0.0),
            im: f64::from_str(matches.value_of("im").unwrap_or(""))
                .unwrap_or(0.0),
        },
        max_tries: u32::from_str(matches.value_of("max-tries").unwrap_or(""))
            .unwrap_or(settings::DEFAULT_MAX_TRIES),
        scale: f64::from_str(matches.value_of("scale").unwrap_or(""))
            .unwrap_or(settings::DEFAULT_SCALE),
        sample_size: u32::from_str(matches.value_of("sample_size").unwrap_or(""))
            .unwrap_or(settings::DEFAULT_SAMPLE_SIZE),
        image_name: matches.value_of("image-name").unwrap_or("").to_string()
    }
}
