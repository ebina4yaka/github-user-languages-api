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

#[get("/languages")]
pub fn languages() -> Json<Vec<LanguagePercentage>> {
    Json(get_languages_percentage())
}

#[get("/languages?<hide>")]
pub fn languages_hide(hide: &RawStr) -> Json<Vec<LanguagePercentage>> {
    Json(get_languages_percentage_hide_option(hide.as_str()))
}

fn main() {
    rocket::ignite()
        .mount("/", routes![languages, languages_hide])
        .launch();
}
