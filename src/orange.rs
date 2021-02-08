use raster::Color;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Response data
#[derive(Serialize, Deserialize)]
pub struct ColorResult {
    pub color: String,
    pub is_orange: bool,
}

/// Request data
#[derive(Deserialize)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl fmt::Display for ColorResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "color {} is_orange {}", self.color, self.is_orange)
    }
}

/// Determine if hex code is orange
pub fn orange_hex(color_string: String) -> Result<ColorResult, &'static str> {
    let mut hex = "#".to_string();

    hex.push_str(&color_string);

    match Color::hex(&hex) {
        Ok(v) => Ok(ColorResult {
            color: hex,
            is_orange: determine_orange(v),
        }),
        Err(_e) => Err("invalid input hex"),
    }
}

/// Determine if rgb value is orange
pub fn orange_rgb(r: u8, g: u8, b: u8) -> ColorResult {
    ColorResult {
        color: format!("r{} g{} b{}", r, g, b),
        is_orange: determine_orange(Color::rgb(r, g, b)),
    }
}

/// Check if orange
pub fn determine_orange(color: raster::Color) -> bool {
    if color.r > 150 && color.g < 170 && color.b < 100 {
        return true;
    }
    return false;
}
