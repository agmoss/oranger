use raster::Color;

pub fn orange_hex(color_string: String) -> Result<bool, &'static str> {
    let mut hex = "#".to_string();

    hex.push_str(&color_string);

    let color = Color::hex(&hex);

    match color {
        Ok(v) => Ok(determine_orange(v)),
        Err(_e) => Err("invalid input hex"),
    }
}

pub fn orange_rgb(r: u8, g: u8, b: u8) -> bool {
    let color = Color::rgb(r, g, b);
    determine_orange(color)
}

pub fn determine_orange(color: raster::Color) -> bool {
    if color.r > 150 && color.g < 170 && color.b < 100 {
        return true;
    }
    return false;
}
