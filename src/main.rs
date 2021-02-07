#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response::status::BadRequest;
use rocket::Request;

mod orange;

#[get("/orange_hex/<color>")]
fn orange_hex(color: String) -> Result<String, BadRequest<String>> {
    let ret = orange::orange_hex(color);
    match ret {
        Ok(v) => Ok(format!("{}", v)),
        Err(e) => Err(BadRequest(Some(format!("Error: {}", e.to_string())))),
    }
}

#[get("/orange_rgb/<r>/<g>/<b>")]
fn orange_rgb(r: u8, g: u8, b: u8) -> String {
    let v = orange::orange_rgb(r, g, b);
    format!("color: {} is_orange: {}", v.color, v.is_orange)
}

#[get("/")]
fn index() -> &'static str {
    "is it orange?"
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

fn main() {
    let e = rocket::ignite()
        .mount("/", routes![orange_hex, orange_rgb, index])
        .register(catchers![not_found])
        .launch();
    println!("Whoops! Rocket didn't launch!");
    println!("This went wrong: {}", e);
}
