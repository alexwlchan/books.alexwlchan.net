#[macro_use]
extern crate lazy_static;

use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("new_templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html"]);
        tera
    };
}

fn main() {
    let context = tera::Context::new();

    println!("{}", (*TEMPLATES).render("base.html", &context).unwrap());
}
