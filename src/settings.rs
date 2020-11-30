use crate::math::Complex;

pub const DEFAULT_WIDTH: u32 = 256;
pub const DEFAULT_HEIGHT: u32 = 256;
pub const DEFAULT_MAX_TRIES: u32 = 1000;
pub const DEFAULT_SCALE: f64 = 0.000000001;
pub const DEFAULT_SAMPLE_SIZE: u32 = 50;

pub struct FractalSettings {
    pub width: u32,
    pub height: u32,
    pub center: Complex,
    pub max_tries: u32,
    pub scale: f64,
    pub sample_size: u32,
    pub image_name: String,
}
