//! # pobsdlib
//! pobsdlib is a library developped to interact with
//! the playonbsd database listing games that should
//! run on OpenBSD. The database can be found here:
//! `https://github.com/playonbsd/OpenBSD-Games-Database`
//!
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
mod utils;
// public api
pub mod collections;
pub mod models;

pub use crate::collections::database::DataBase;
