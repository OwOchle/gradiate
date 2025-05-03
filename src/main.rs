use crate::args::{GradiateArgs, MixSpace};
use clap::Parser;
use crossterm::queue;
use crossterm::style::{Color, Print, ResetColor, SetForegroundColor};
use hex_color::HexColor;
use palette::{named, FromColor, Hsv, IntoColor, Srgb};
use std::io::{stdin, stdout, Write};
use std::ops::Deref;
use crate::spaces::hsv::hsv_space;
use crate::spaces::rgb::rgb_space;

mod args;
mod helpers;
mod spaces;

fn main() {
    let args = GradiateArgs::parse();
    
    let mut colors: Vec<_> = vec![];
    let mut space: MixSpace = args.space;
    
    if let Some(col) = args.colors {
        match col.deref() {
            "trans" => {
                colors = color_vector!(
                    Srgb::new(91u8, 206, 250);
                    Srgb::new(245u8, 169, 184);
                    Srgb::new(255u8, 255, 255);
                    Srgb::new(245u8, 169, 184);
                    Srgb::new(91u8, 206, 250)
                );
                space = MixSpace::RGB
            },
            _ => {
                for c in col.split(',') {
                    if let Ok(c) = HexColor::parse(c) {
                        colors.push(Hsv::from_color(Srgb::new(c.r, c.g, c.b).into_format()))
                    } else if let Some(n) = named::from_str(c) {
                        colors.push(Hsv::from_color(n.into_format()))
                    }
                }
            }
        }
    } else {
        colors = color_vector!(
            Srgb::new(255u8, 0, 0);
            Srgb::new(0u8, 255, 0);
            Srgb::new(0u8, 0, 255)
        );
        space = MixSpace::HSV
    }

    let mut stdout = stdout();

    if args.text.len() == 0 {
        for (i, rline) in stdin().lines().enumerate() {
            if rline.is_err() {
                continue
            }
            
            let line = rline.unwrap();
            let quantum = (colors.len() - 1) as f32 / line.len() as f32;
            let mut factor = i as f32 * args.line_offset;

            for cha in line.chars() {
                let mix = match space {
                    MixSpace::RGB => rgb_space(&colors, factor),
                    MixSpace::HSV => hsv_space(&colors, factor)
                };

                let _ = queue!(stdout, SetForegroundColor(Color::Rgb {r: mix.red, g: mix.green, b: mix.blue}), Print(cha));

                factor += quantum;
            }

            println!();
        }
    } else {
        let out = args.text.join(" ");
        
        for (i, line) in out.lines().enumerate() {
            let quantum = (colors.len() - 1) as f32 / line.len() as f32;
            let mut factor = i as f32 * args.line_offset;
            
            for cha in line.chars() {
                let mix = match space {
                    MixSpace::RGB => rgb_space(&colors, factor),
                    MixSpace::HSV => hsv_space(&colors, factor)
                };

                let _ = queue!(stdout, SetForegroundColor(Color::Rgb {r: mix.red, g: mix.green, b: mix.blue}), Print(cha));

                factor += quantum;
            }

            println!();
        }
    }
    
    let _ = queue!(stdout, ResetColor);

    stdout.flush().unwrap()
}
