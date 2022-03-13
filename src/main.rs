#[macro_use]
extern crate lazy_static;

mod templates;

fn main() {
    let context = tera::Context::new();

    println!("{:?}", templates::render("base.html", &context));
}
