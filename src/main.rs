use clap::Parser;
use hex_color::HexColor;
use palette::{named, FromColor, Hsv, IntoColor, Srgb};
use palette::chromatic_adaptation::AdaptInto;
use palette::rgb::Rgb;
use crate::args::GradiateArgs;

mod args;

fn main() {
    let args = GradiateArgs::parse();
    
    let mut colors: Vec<Hsv<_, f32>> = vec![];
    
    if let Some(col) = args.colors {
        for c in col.split(',') {
            if let Ok(c) = HexColor::parse(c) {
                colors.push(Hsv::from_color(Srgb::new(c.r, c.g, c.b).into_format()))
            } else if let Some(n) = named::from_str(c) {
                colors.push(Hsv::from_color(n.into_format()))
            }
        }
    }
    
    if args.text.len() == 0 {
        // TODO: Stdin
    } else {
        let out = args.text.join(" ");

        println!("{}", out);
    }
}
