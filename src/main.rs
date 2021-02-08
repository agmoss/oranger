#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use serde::Deserialize;

use rocket::http::{Cookie, Cookies};
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
    let ret = orange::orange_hex(color);

    return ret
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
    let v = orange::orange_rgb(r, g, b);
    Json(v)
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
    let v = orange::orange_rgb(rgb.r, rgb.g, rgb.b);
    Json(v)
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
///  http://localhost:8000/login \
/// ```
#[post("/login", data = "<user>")]
fn login(user: Json<AuthInfo>, mut cookies: Cookies) -> String {
    let pass = user.password.to_string();
    cookies.add(Cookie::new("pass", pass));
    cookies
        .get("pass")
        .map(|_c| "Login success".to_string())
        .unwrap_or("Login failure".to_string())
}

/// Protected route
/// # Example
/// ```bash
/// curl -L -b cookies.txt http://localhost:8000/orange
/// ```
#[get("/orange")]
fn obtain_orange(cookies: Cookies) -> String {
    let c = cookies.get("pass");
    match c {
        Some(v) => {
            if v.value() == "orange".to_string() {
                // User logged in 
                "#FFA500".to_string()
            } else {
                "Invalid password".to_string()
            }
        }
        None => "You are not logged in".to_owned(),
    }
}

/// Testing
#[get("/examine-cookies")]
fn handler(cookies: Cookies) -> String {
    let c = cookies.get("pass");
    match c {
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
