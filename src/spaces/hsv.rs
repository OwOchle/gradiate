use palette::{Hsv, IntoColor, Mix, Srgb};

pub fn hsv_space(colors: &Vec<Hsv>, factor: f32) -> Srgb<u8> {
    let index = factor.floor() as usize;

    let a: Hsv = colors[index % colors.len()];
    let b: Hsv = colors[(index + 1) % colors.len()];

    <Hsv as IntoColor<Srgb>>::into_color(a.mix(b, factor % 1.0)).into_format()
}