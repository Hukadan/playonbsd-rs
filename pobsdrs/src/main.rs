#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate pobsdlib;
extern crate serde_json;

pub mod game_end_points;

use self::game_end_points::{game_all, game_id, game_search_by_name};
use pobsdlib::collections::DataBase;
use rocket::fs::{relative, FileServer};
//use std::{env, io, path, process};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(DataBase::new("../db/openbsd-games.db"))
        .mount("/", FileServer::from(relative!("front-end/dist/")))
        .mount("/api/", routes![game_all, game_id, game_search_by_name])
}
