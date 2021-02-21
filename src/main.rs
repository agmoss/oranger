#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use serde::Deserialize;

extern crate time;

use rocket::http::Status;
use rocket::http::{Cookie, Cookies};
use rocket::request::{self, FromRequest};
use rocket::response::status;
use rocket::response::status::BadRequest;
use rocket::Outcome;
use rocket::Request;
use rocket_contrib::json::{Json, JsonValue};
use time::Duration;

mod orange;

const PASSWORD: &str = "orange";
const API_KEY: &str = "not-orange";

/// Determine if hex code is orange
/// # Example
/// ```bash
/// curl -v http://localhost:8000/orange/FFA500
/// ```
#[get("/orange/<color>", format = "json")]
fn orange_hex(color: String) -> Result<Json<orange::ColorResult>, BadRequest<JsonValue>> {
    return orange::orange_hex(color)
        .map(|v| Json(v))
        .map_err(|e| BadRequest(Some(json!({"error": e.to_string()}))));
}

/// Determine if rgb is orange
/// # Example
/// ```bash
/// curl -v http://localhost:8000/orange/255/165/0
/// ```
#[get("/orange/<r>/<g>/<b>", format = "json")]
fn orange_rgb(r: u8, g: u8, b: u8) -> Json<orange::ColorResult> {
    return Json(orange::orange_rgb(r, g, b));
}

/// Determine if rgb is orange
/// # Example
/// ```bash
/// curl --header "Content-Type: application/json" \
///  --request POST \
///  --data '{"r":255,"g":165,"b":0}' \
///  http://localhost:8000/orange
/// ```
#[post("/orange", data = "<rgb>")]
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
    cookies.add_private(
        Cookie::build("password", user.password.to_string())
            .path("/")
            .max_age(Duration::days(5))
            .finish(),
    );
    return cookies
        .get_private("password")
        .map(|_v| status::Accepted(Some(json!({"login":"success"}))))
        .ok_or(BadRequest(Some(json!({"error": "you are not logged in"}))));
}

/// Protected route
/// # Example
/// ```bash
/// curl -L -b cookies.txt http://localhost:8000/orange
/// ```
#[get("/orange", format = "json")]
fn obtain_orange(
    mut cookies: Cookies,
) -> Result<status::Accepted<JsonValue>, BadRequest<JsonValue>> {
    match cookies.get_private("password") {
        Some(v) => {
            if v.value() == PASSWORD {
                // User logged in
                Ok(status::Accepted(Some(
                    json!({"orange":"#FFA500".to_string() }),
                )))
            } else {
                Err(BadRequest(Some(json!({"error": "invalid password"}))))
            }
        }
        None => Err(BadRequest(Some(json!({"error": "you are not logged in"})))),
    }
}

/// Testing
#[get("/examine-cookies")]
fn handler(mut cookies: Cookies) -> String {
    match cookies.get_private("pass") {
        Some(v) => v.to_string(),
        None => "No Cookies".to_owned(),
    }
}

/// Modified example of https://api.rocket.rs/v0.4/rocket/request/trait.FromRequest.html#example-1
/// type level proof

struct ApiKey(String);

fn is_valid(key: &str) -> bool {
    key == API_KEY
}

#[derive(Debug)]
enum ApiKeyError {
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    /// Can Activate?
    /// authenticate -> api_key -> Option<Authorized>
    /// or more specifically
    /// from_request -> request: &Request -> Outcome<S,E>
    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.headers().get_one("x-api-key") {
            Some(p) => {
                if is_valid(p) {
                    // ie. Some(Authorized)
                    Outcome::Success(ApiKey(p.to_string()))
                } else {
                    Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid))
                }
            }
            None => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
        }
    }
}

/// sensitive -> key:ApiKey -> JsonValue
/// The only way to get the ApiKey type is from `FromRequest` implementation
/// # Example
/// ```bash
/// curl --header "Content-Type: application/json" -H "x-api-key: not-orange" --request POST http://localhost:8000/sensitive
/// ``
#[post("/sensitive", format = "json")]
fn requires_api_key(_key: ApiKey) -> JsonValue {
    json!({"data":"sensitive data"})
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
                requires_api_key,
                handler,
                index,
            ],
        )
        .register(catchers![not_found])
        .launch();
    println!("Whoops! Rocket didn't launch!");
    println!("This went wrong: {}", e);
}
