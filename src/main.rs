#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod lib;
use crate::lib::languages::get_languages_percentage;
use crate::lib::models::ErrorMessage;
use crate::lib::models::LanguagePercentage;
use rocket::http::Method;
use rocket::http::RawStr;
use rocket::request::Form;
use rocket::Request;
use rocket::{get, routes};
use rocket_contrib::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[catch(500)]
fn internal_error() -> Json<ErrorMessage> {
    let error = ErrorMessage {
        message: "Whoops! Looks like we messed up.".to_string(),
    };
    Json(error)
}

#[catch(404)]
fn not_found(req: &Request) -> Json<ErrorMessage> {
    let error = ErrorMessage {
        message: format!("I couldn't find '{}'. Try something else?", req.uri()),
    };
    Json(error)
}

#[derive(FromForm)]
pub struct Params {
    pub hide: Option<String>,
    pub limit: Option<usize>,
}

#[get("/user/<username>?<params..>")]
pub fn languages(username: &RawStr, params: Option<Form<Params>>) -> Json<Vec<LanguagePercentage>> {
    Json(get_languages_percentage(username.as_str(), params))
}

fn main() {
    let allowed_origins = AllowedOrigins::All;
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .unwrap();

    rocket::ignite()
        .mount("/", routes![languages])
        .attach(cors)
        .register(catchers![internal_error, not_found])
        .launch();
}
