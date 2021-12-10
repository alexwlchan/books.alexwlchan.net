use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use clap::{Arg, App, SubCommand};
use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use oxipng;
use regex::Regex;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use palette::IntoColor;
use palette::{Srgb, Hsl};
use palette::rgb::Rgb;

pub fn subcommand() -> App<'static, 'static> {
    SubCommand::with_name("create_shelf")
        .about("Creates the shelf image used as a header on review pages")
        .arg(
            Arg::with_name("COLOUR")
                .help("The hex colour to use as the base of this shelf")
                .required(true)
                .validator(is_hex_string)
        )
}

// Creates a shelf file.
//
// This function assumes the hex string is correctly formatted.
pub fn create_shelf(hex_string: &str) -> () {
    let mut rng = SmallRng::seed_from_u64(0);

    let width: i32 = 2000;
    let height: i32 = 90;

    let mut img: RgbaImage = ImageBuffer::new(width as u32, height as u32);

    let mut x_coord: i32 = 0;

    let rgb: Srgb = parse_hex_string(hex_string);
    let hsl: Hsl = rgb.into_hsl();

    while x_coord < width {
        let shelf_width: i32 = rng.gen_range(4..28);

        // Shelves go from 30px to 45px height, then 2x for retina displays.
        let shelf_height: u32 = rng.gen_range(60..90);

        draw_filled_rect_mut(
            &mut img,
            Rect::at(x_coord, 0).of_size(shelf_width as u32, shelf_height),
            create_random_colour_like(&mut rng, &hsl)
        );

        x_coord += shelf_width;
    }

    fs::create_dir_all("_shelves").unwrap();

    let out_path = format!("_shelves/{:02x}{:02x}{:02x}.png", (rgb.red * 255.0) as i32, (rgb.green * 255.0) as i32, (rgb.blue * 255.0) as i32);

    img.save(&out_path).unwrap();
    optimise_png(&out_path);
}

// Optimise a PNG image using oxipng.
//
// This can get significantly smaller files (~12KB down to 1KB).
fn optimise_png(p: &str) -> () {
    let mut options = oxipng::Options::from_preset(5);
    options.timeout = Some(Duration::from_secs(2));
    let infile = oxipng::InFile::Path(PathBuf::from(p));
    let outfile = oxipng::OutFile::Path(Some(PathBuf::from(p)));
    oxipng::optimize(&infile, &outfile, &options).unwrap();
}

fn min(f1: f32, f2: f32) -> f32 {
    if f1 > f2 { f2 } else { f1 }
}

fn max(f1: f32, f2: f32) -> f32 {
    if f1 > f2 { f1 } else { f2 }
}

fn create_random_colour_like(rng: &mut SmallRng, hsl: &Hsl) -> Rgba<u8> {
    let v = min(hsl.lightness, 0.45);
    let new_lightness = rng.gen_range(max(v * 3.0 / 4.0, 0.0)..min(v * 4.0 / 3.0, 1.0));

    let new_hsl = Hsl::new(hsl.hue, hsl.saturation, new_lightness);

    let rgb: Srgb = Rgb::from_linear(new_hsl.into_rgb());
    Rgba::from([(rgb.red * 255.0) as u8, (rgb.green * 255.0) as u8, (rgb.blue * 255.0) as u8, 255])
}

// Checks whether a given string is a valid hex string.
//
// See https://docs.rs/clap/latest/clap/struct.Arg.html#method.validator
fn is_hex_string(s: String) -> Result<(), String> {
    let hex_regex = Regex::new(r"^#[0-9a-fA-F]{6}$").unwrap();
    if hex_regex.is_match(&s) {
        Ok(())
    } else {
        Err(format!("Expected a hex string, e.g. #d01c11, got {}", s))
    }
}

// Parses a hex string as an RGB tuple, e.g. #d01c11 ~> (208, 28, 17)
//
// This function assumes the hex string is correctly formatted.
fn parse_hex_string(s: &str) -> Srgb {
    assert_eq!(s.len(), 7);
    let r = u8::from_str_radix(&s[1..3], 16).unwrap() as f32;
    let g = u8::from_str_radix(&s[3..5], 16).unwrap() as f32;
    let b = u8::from_str_radix(&s[5..7], 16).unwrap() as f32;
    Srgb::new(r / 255f32, g / 255f32, b / 255f32)
}