//! # pobsdlib
//! pobsdlib is a library developped to interact with
//! the playonbsd database listing games that should
//! run on OpenBSD. The database can be found here:
//! `https://github.com/playonbsd/OpenBSD-Games-Database`
//!
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate reqwest;
extern crate serde_json;
// public api
pub mod collections;
pub mod models;
pub mod utils;

pub use crate::collections::database::DataBase;
pub use crate::collections::query_result::QueryResult;
pub use crate::models::field::Field;
pub use crate::models::game::Game;
pub use crate::models::game_filter::GameFilter;
pub use crate::models::item::Item;
