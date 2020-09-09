#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod lib;
use lib::get_languages_percentage;
use lib::get_languages_percentage_hide_option;
use lib::models::Error;
use lib::models::LanguagePercentage;
use rocket::http::RawStr;
use rocket::Request;
use rocket_contrib::json::Json;

#[catch(500)]
fn internal_error() -> Json<Error> {
    let error = Error {
        message: "Whoops! Looks like we messed up.".to_string(),
    };
    Json(error)
}

#[catch(404)]
fn not_found(req: &Request) -> Json<Error> {
    let error = Error {
        message: format!("I couldn't find '{}'. Try something else?", req.uri()),
    };
    Json(error)
}

#[get("/user/<username>")]
pub fn languages(username: &RawStr) -> Json<Vec<LanguagePercentage>> {
    Json(get_languages_percentage(username.as_str()))
}

#[get("/user/<username>?<hide>")]
pub fn languages_hide(username: &RawStr, hide: &RawStr) -> Json<Vec<LanguagePercentage>> {
    Json(get_languages_percentage_hide_option(
        username.as_str(),
        hide.as_str(),
    ))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![languages, languages_hide])
        .register(catchers![not_found])
        .launch();
}
