use palette::{Hsv, IntoColor, Mix, Srgb};
use palette::rgb::Rgb;

pub fn rgb_space(colors: &Vec<Hsv>, factor: f32) -> Srgb<u8> {
    let index = factor.floor() as usize;

    let a: Rgb = colors[index % colors.len()].into_color();
    let b: Rgb = colors[index + 1 % colors.len()].into_color();

    a.mix(b, factor % 1.0).into_format()
}