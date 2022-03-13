use std::collections::HashMap;

use tera::{from_value, to_value, Tera, Value};

use crate::create_shelf;

fn create_shelf_data_uri(args: &HashMap<String, Value>) -> tera::Result<Value> {
    let s: String = from_value(args.get("tint_colour").unwrap().to_owned()).unwrap();
    Ok(to_value(create_shelf::create_shelf_data_uri(&s).unwrap()).unwrap())
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
    Ok(tera)
}
