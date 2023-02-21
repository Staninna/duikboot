use colorsys::{Hsl, Rgb};

fn shade_hex(hex: &str, shade: i32) -> String {
    let rgb = {
        let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
        let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
        let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
        (r, g, b)
    };

    let hsl = Hsl::from(Rgb::from(rgb));

    let adjusted_lightness = hsl.lightness() as i32 + shade;
    let new_lightness = if adjusted_lightness < 0 {
        0
    } else if adjusted_lightness > 100 {
        100
    } else {
        adjusted_lightness as u8
    };

    let new_color = Hsl::new(hsl.hue(), hsl.saturation(), new_lightness as f64, None);
    let new_rgb = Rgb::from(new_color);
    let new_hex = format!(
        "#{:02X}{:02X}{:02X}",
        new_rgb.red() as u8,
        new_rgb.green() as u8,
        new_rgb.blue() as u8
    );

    new_hex
}

fn color_sheme(base_hex: &str) -> [String; 6] {
    [
        shade_hex(base_hex, 0),
        shade_hex(base_hex, 10),
        shade_hex(base_hex, 20),
        shade_hex(base_hex, -10),
        shade_hex(base_hex, -20),
        shade_hex(base_hex, -30),
    ]
}
