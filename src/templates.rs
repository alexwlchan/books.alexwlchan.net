use std::collections::HashMap;

use tera::{from_value, to_value, try_get_value, Tera, Value};

use crate::{create_shelf, colours, models, text_helpers};

fn create_shelf_data_uri(args: &HashMap<String, Value>) -> tera::Result<Value> {
    let s: String = from_value(args.get("tint_colour").unwrap().to_owned()).unwrap();
    Ok(to_value(create_shelf::create_shelf_data_uri(&s).unwrap()).unwrap())
}

pub fn markdown(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = try_get_value!("markdown", "value", String, value);
    Ok(to_value(text_helpers::markdown(&s)).unwrap())
}

pub fn smartypants(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = try_get_value!("smartypants", "value", String, value);
    Ok(to_value(text_helpers::smartypants(&s)).unwrap())
}

pub fn render_date(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let s = try_get_value!("render_date", "value", String, value);
    Ok(to_value(text_helpers::render_date(&s)).unwrap())
}

pub fn spread_star_rating(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let rating = try_get_value!("spread_star_rating", "value", usize, value);
    Ok(to_value(text_helpers::spread_star_rating(rating)).unwrap())
}

pub fn star_rating(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let rating = try_get_value!("star_rating", "value", usize, value);
    Ok(to_value(text_helpers::star_rating(rating)).unwrap())
}

fn sorted_reviews(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let reviews = try_get_value!("sorted_reviews", "value", Vec<models::Review>, value);
    println!("{:?}", reviews);
    Ok(to_value("hello").unwrap())
}

pub fn as_rgba(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    let hex_string = try_get_value!("as_rgba", "value", String, value);
    let opacity: f32 = from_value(args.get("opacity").unwrap().to_owned()).unwrap();

    let rgb = colours::parse_hex_string(&hex_string);
    let output = format!("rgba({}, {}, {}, {})", rgb.red, rgb.green, rgb.blue, opacity);

    Ok(to_value(output).unwrap())
}

pub fn boost(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
    let hex_string = try_get_value!("boost", "value", String, value);
    let multiplier: f32 = from_value(args.get("multiplier").unwrap().to_owned()).unwrap();
    let boost: f32 = from_value(args.get("boost").unwrap().to_owned()).unwrap();

    let rgb = colours::parse_hex_string(&hex_string);

    /* See https://stackoverflow.com/a/11615135/1558022 */
    let red: f32   = rgb.red as f32 * multiplier + boost * 255.0;
    let green: f32 = rgb.green as f32 * multiplier + boost * 255.0;
    let blue: f32  = rgb.blue as f32 * multiplier + boost * 255.0;

    let output = format!(
        "rgb({}, {}, {})",
        if red.round() as i32 > 255   { 255 } else { red.round()   as i32 },
        if green.round() as i32 > 255 { 255 } else { green.round() as i32 },
        if blue.round() as i32 > 255  { 255 } else { blue.round()  as i32 }
    );

    Ok(to_value(output).unwrap())
}

pub fn get_templates() -> Result<Tera, tera::Error> {
    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            return Err(e);
        }
    };
    tera.autoescape_on(vec![".html"]);

    tera.register_function("create_shelf_data_uri", create_shelf_data_uri);

    tera.register_filter("as_rgba", as_rgba);
    tera.register_filter("boost", boost);
    tera.register_filter("render_date", render_date);
    tera.register_filter("markdown", markdown);
    tera.register_filter("smartypants", smartypants);
    tera.register_filter("sorted_reviews", sorted_reviews);
    tera.register_filter("spread_star_rating", spread_star_rating);
    tera.register_filter("star_rating", star_rating);

    Ok(tera)
}
