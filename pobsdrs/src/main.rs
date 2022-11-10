#[macro_use]
extern crate rocket;
extern crate rocket_dyn_templates;
#[macro_use]
extern crate serde_derive;
extern crate pobsdlib;
extern crate serde_json;

pub mod routes;
pub mod wrappers;

use self::routes::api::game_end_points::{game_all, game_id, game_search};
use self::routes::api::home::api_home;
use self::routes::html::{gamedetails, gamelist};
use pobsdlib::collections::DataBase;
use rocket::fairing::AdHoc;
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::Template;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Config {
    domain: String,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(DataBase::new("../db/openbsd-games.db"))
        .mount("/static", FileServer::from("static/"))
        .mount("/api/", routes![api_home, game_all, game_id, game_search])
        .mount("/", routes![gamelist, gamedetails])
        .attach(AdHoc::config::<Config>())
        .attach(Template::fairing())
}
