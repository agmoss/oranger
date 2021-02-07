use raster::Color;
use std::fmt;

pub struct ColorResult {
    pub color: String,
    pub is_orange: bool,
}

impl fmt::Display for ColorResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "color {} is_orange {}", self.color, self.is_orange)
    }
}

pub fn orange_hex(color_string: String) -> Result<ColorResult, &'static str> {
    let mut hex = "#".to_string();

    hex.push_str(&color_string);

    let color = Color::hex(&hex);

    match color {
        Ok(v) => Ok(ColorResult {
            color: hex,
            is_orange: determine_orange(v),
        }),
        Err(_e) => Err("invalid input hex"),
    }
}

pub fn orange_rgb(r: u8, g: u8, b: u8) -> ColorResult {
    let color = Color::rgb(r, g, b);

    let res = ColorResult {
        color: format!("r{} g{} b{}", r, g, b),
        is_orange: determine_orange(color),
    };

    res
}

pub fn determine_orange(color: raster::Color) -> bool {
    if color.r > 150 && color.g < 170 && color.b < 100 {
        return true;
    }
    return false;
}
