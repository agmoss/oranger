#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use serde::Deserialize;

use rocket::http::{Cookie, Cookies};
use rocket::response::status;
use rocket::response::status::BadRequest;
use rocket::Request;
use rocket_contrib::json::{Json, JsonValue};

mod orange;

/// Determine if hex code is orange
/// # Example
/// ```bash
/// curl -v http://localhost:8000/orange_hex/FFA500
/// ```
#[get("/orange_hex/<color>", format = "json")]
fn orange_hex(color: String) -> Result<Json<orange::ColorResult>, BadRequest<JsonValue>> {
    return orange::orange_hex(color)
        .map(|v| Json(v))
        .map_err(|e| BadRequest(Some(json!({"error": e.to_string()}))));
}

/// Determine if rgb is orange
/// # Example
/// ```bash
/// curl -v http://localhost:8000/orange_rgb/255/165/0
/// ```
#[get("/orange_rgb/<r>/<g>/<b>", format = "json")]
fn orange_rgb(r: u8, g: u8, b: u8) -> Json<orange::ColorResult> {
    return Json(orange::orange_rgb(r, g, b));
}

/// Determine if rgb is orange
/// # Example
/// ```bash
/// curl --header "Content-Type: application/json" \
///  --request POST \
///  --data '{"r":255,"g":165,"b":0}' \
///  http://localhost:8000/orange_rgb_post
/// ```
#[post("/orange_rgb_post", data = "<rgb>")]
fn orange_rgb_post(rgb: Json<orange::RgbColor>) -> Json<orange::ColorResult> {
    return Json(orange::orange_rgb(rgb.r, rgb.g, rgb.b));
}

#[derive(Deserialize)]
pub struct AuthInfo {
    pub user_id: i32,
    pub password: String,
}

/// Cookie based login strategy
/// # Example
/// ```bash
/// curl --header "Content-Type: application/json" \
///  --request POST \
///  --data '{"user_id":1,"password":"orange"}' \
///  -c cookies.txt \
///  http://localhost:8000/login
/// ```
#[post("/login", data = "<user>", format = "json")]
fn login(
    user: Json<AuthInfo>,
    mut cookies: Cookies,
) -> Result<status::Accepted<JsonValue>, BadRequest<JsonValue>> {
    cookies.add(Cookie::new("pass", user.password.to_string()));
    return cookies
        .get("pass")
        .map(|_v| status::Accepted(Some(json!({"login":"success"}))))
        .ok_or(BadRequest(Some(json!({"error": "you are not logged in"}))));
}

/// Protected route
/// # Example
/// ```bash
/// curl -L -b cookies.txt http://localhost:8000/orange
/// ```
#[get("/orange", format = "json")]
fn obtain_orange(cookies: Cookies) -> Result<JsonValue, BadRequest<JsonValue>> {
    match cookies.get("pass") {
        Some(v) => {
            if v.value() == "orange".to_string() {
                // User logged in
                Ok(json!({"orange":"#FFA500".to_string() }))
            } else {
                Err(BadRequest(Some(json!({"error": "invalid password"}))))
            }
        }
        None => Err(BadRequest(Some(json!({"error": "you are not logged in"})))),
    }
}

/// Testing
#[get("/examine-cookies")]
fn handler(cookies: Cookies) -> String {
    match cookies.get("pass") {
        Some(v) => v.to_string(),
        None => "No Cookies".to_owned(),
    }
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
        .mount(
            "/",
            routes![
                orange_hex,
                orange_rgb,
                orange_rgb_post,
                login,
                obtain_orange,
                handler,
                index
            ],
        )
        .register(catchers![not_found])
        .launch();
    println!("Whoops! Rocket didn't launch!");
    println!("This went wrong: {}", e);
}
