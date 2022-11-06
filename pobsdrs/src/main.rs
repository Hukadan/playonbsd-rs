#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate pobsdlib;
extern crate serde_json;

pub mod routes;
pub mod wrappers;

use self::routes::api::game_end_points::{game_all, game_id, game_search};
use self::routes::api::home::api_home;
use pobsdlib::collections::DataBase;
use rocket::fairing::AdHoc;
use rocket::fs::{relative, FileServer};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct Config {
    domain: String,
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(DataBase::new("../db/openbsd-games.db"))
        .mount("/", FileServer::from("front-end/dist/"))
        //.mount("/", FileServer::from(relative!("front-end/dist/")))
        .mount("/api/", routes![api_home, game_all, game_id, game_search])
        .attach(AdHoc::config::<Config>())
}
