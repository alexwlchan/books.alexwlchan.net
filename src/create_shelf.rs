// This file creates the "bookshelf" which appears as the header of
// every page.
//
// The bookshelves are tinted with the dominant colour of the cover of
// the book on that page (or black on the index pages).

use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use oxipng;
use regex::Regex;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use palette::FromColor;
use palette::{Srgb, Hsl};
use palette::rgb::Rgb;

use crate::colours;
use crate::errors::VfdError;
use crate::fs_helpers;

pub fn create_shelf_data_uri(hex_string: &str) -> Result<String, VfdError> {
    let path = create_shelf(hex_string);

    let bytes = fs_helpers::read_file(&path)?;

    Ok(format!("data:image/png;base64,{}", base64::encode(bytes)))
}

// Creates a shelf image.
//
// This function assumes the hex string is correctly formatted.
fn create_shelf(hex_string: &str) -> PathBuf {
    let rgb: Srgb<u8> = colours::parse_hex_string(hex_string);

    let out_path = PathBuf::from(format!("_shelves/{:02x}{:02x}{:02x}.png", rgb.red, rgb.green, rgb.blue));
    if out_path.exists() {
        return out_path;
    }

    let hsl: Hsl = if rgb.red <= 13 && rgb.green <= 13 && rgb.blue <= 13 {
        Hsl::from_color(Srgb::<f32>::new(13.0 / 255.0, 13.0 / 255.0, 13.0 / 255.0))
    } else {
        Hsl::from_color(Srgb::<f32>::new(rgb.red as f32 / 255.0, rgb.green as f32 / 255.0, rgb.blue as f32 / 255.0))
    };

    // We seed the random generator to ensure we always get the same shape.
    // i.e. the rectangles that make up the shelf.
    //
    // In particular, as somebody navigates around the site, they should
    // see the bookshelf changing colours, but it should never change
    // shape -- that would be too jarring.
    let mut rng = SmallRng::seed_from_u64(0);

    let mut img: RgbaImage = ImageBuffer::new(2000, 90);

    let mut x_coord: i32 = 0;

    while x_coord < img.width() as i32 {
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

    // If the shelves directory doesn't exist, we'll have to recreate all the
    // images from scratch.  Because the RNG is seeded, this should be fine --
    // the images are deterministic -- but let's provide some feedback to the
    // user about what we're doing.
    if !PathBuf::from("_shelves").exists() {
        println!("Creating shelf header images for the first time, this may take a while...");
    }

    // The use of .unwrap() here is very naughty, I know, I know...
    //
    // But since this is only ever going to run on a machine I control
    // and it'll be pretty obvious if any of this fails, I'm fine with that.

    fs::create_dir_all("_shelves").unwrap();

    img.save(&out_path).unwrap();
    optimise_png(&out_path);

    out_path
}

// Optimise a PNG image using oxipng.
//
// This can get significantly smaller files (~12KB down to 1KB).
fn optimise_png(p: &PathBuf) -> () {
    let mut options = oxipng::Options::from_preset(5);
    options.timeout = Some(Duration::from_secs(2));
    let infile = oxipng::InFile::Path(p.to_path_buf());
    let outfile = oxipng::OutFile::Path(Some(p.to_path_buf()));
    oxipng::optimize(&infile, &outfile, &options).unwrap();
}

fn min(f1: f32, f2: f32) -> f32 {
    if f1 > f2 { f2 } else { f1 }
}

fn max(f1: f32, f2: f32) -> f32 {
    if f1 > f2 { f1 } else { f2 }
}

// Create a random colour that's similar to the given colour.
//
// All this does is modify the "lightness" parameter in HSL space.
// There are probably better ways to create similar colours within a
// given hue (colour is neither linear nor simple), but this creates
// good enough results.
//
// I don't remember how I picked all these constants -- I might have
// chosen them arbitrarily until I got something that looked good.
fn create_random_colour_like(rng: &mut SmallRng, hsl: &Hsl) -> Rgba<u8> {
    let v = min(hsl.lightness, 0.45);
    let new_lightness = rng.gen_range(max(v * 3.0 / 4.0, 0.0)..min(v * 4.0 / 3.0, 1.0));

    let new_hsl = Hsl::new(hsl.hue, hsl.saturation, new_lightness);

    let rgb: Srgb = Rgb::from_color(new_hsl);
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
