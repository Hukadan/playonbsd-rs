#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
mod parser_macros;
pub mod field;
pub mod game;
pub mod parser;
mod split_line;

pub use crate::parser::Parser;
pub use crate::game::Game;
pub use crate::field::Field;