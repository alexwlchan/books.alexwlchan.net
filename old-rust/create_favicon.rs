use std::fs;
use std::path::PathBuf;

use palette::Srgb;

use crate::colours;

// Creates a favicon.
//
// This function assumes the hex string is correctly formatted.
pub fn create_favicon(hex_string: &str) -> () {
    let rgb: Srgb<u8> = colours::parse_hex_string(hex_string);

    let prefix = format!("{:02x}{:02x}{:02x}", rgb.red, rgb.green, rgb.blue);

    let ico_path = PathBuf::from(format!("_html/favicons/{}.ico", prefix));
    if ico_path.exists() {
        return;
    }

    let mut img16 = image::open("static/favicon_16.png").unwrap().into_rgba8();

    for (_, _, pixel) in img16.enumerate_pixels_mut() {
        let alpha = pixel[3];
        *pixel = image::Rgba([rgb.red, rgb.green, rgb.blue, alpha]);
    }

    let mut img32 = image::open("static/favicon_32.png").unwrap().into_rgba8();

    for (_, _, pixel) in img32.enumerate_pixels_mut() {
        let alpha = pixel[3];
        *pixel = image::Rgba([rgb.red, rgb.green, rgb.blue, alpha]);
    }

    // The use of .unwrap() here is very naughty, I know, I know...
    //
    // But since this is only ever going to run on a machine I control
    // and it'll be pretty obvious if any of this fails, I'm fine with that.
    fs::create_dir_all("_html/favicons").unwrap();

    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);

    let ico16 = ico::IconImage::from_rgba_data(16, 16, img16.to_vec());
    icon_dir.add_entry(ico::IconDirEntry::encode(&ico16).unwrap());

    let ico32 = ico::IconImage::from_rgba_data(32, 32, img32.to_vec());
    icon_dir.add_entry(ico::IconDirEntry::encode(&ico32).unwrap());

    img32
        .save(format!("_html/favicons/{}.png", prefix))
        .unwrap();

    // We save the ico file last, so we can use its presence to skip
    // recreating it next time.
    let file = std::fs::File::create(ico_path).unwrap();
    icon_dir.write(file).unwrap();
}
