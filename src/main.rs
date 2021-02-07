#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::response::status::BadRequest;
use rocket::Request;
use rocket_contrib::json::{Json, JsonValue};

mod orange;

/// Determine if hex code is orange
/// # Example
/// ```
/// curl -v http://localhost:8000/orange_hex/FFA500
/// ```
#[get("/orange_hex/<color>", format = "json")]
fn orange_hex(color: String) -> Result<Json<orange::ColorResult>, BadRequest<JsonValue>> {
    let ret = orange::orange_hex(color);

    return ret
        .map(|v| Json(v))
        .map_err(|e| BadRequest(Some(json!({"error": e.to_string()}))));
}

/// Determine if rgb is orange
/// # Example
/// ```
/// curl -v http://localhost:8000/orange_rgb/255/165/0
/// ```
#[get("/orange_rgb/<r>/<g>/<b>", format = "json")]
fn orange_rgb(r: u8, g: u8, b: u8) -> Json<orange::ColorResult> {
    let v = orange::orange_rgb(r, g, b);
    Json(v)
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
