#![feature(proc_macro_hygiene)]
#![feature(decl_macro)]

#[macro_use]
extern crate rocket;

mod lib;
use lib::get_languages_percentage;
use lib::get_languages_percentage_hide_option;
use lib::models::LanguagePercentage;
use rocket::http::RawStr;
use rocket_contrib::json::Json;

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
        .launch();
}
