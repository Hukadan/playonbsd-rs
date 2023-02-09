//! pobsdrs-parser provides a simplistic [`Parser`] that converts
//! the [PlayOnBSD Database](https://github.com/playonbsd/OpenBSD-Games-Database)
//! (either provided as a string or as a file) into a vector of [`Game`] objects.
//!
//! # Parser
//! A new parser can be create using the [`Parser::new`] method and proving
//! a [`ParsingMode`] enum as only argument.
//! The parsing supports two modes representend by the two variants of the
//! [`ParsingMode`] enum:
//! * **strict mode** ([`ParsingMode::Strict`]) in which the parsing
//!  will stop if a parsing error occurs returning the games processed
//! before the error as well as the line in the input (file or string)
//! where the error occured;
//! * **relaxed mode** ([`ParsingMode::Relaxed`]) where the parsing
//! will continue even after an error is encountered, the parsing
//! resuming when reaching the next game after the parsing error
//! ; it returns all the games that have been parsed as well as
//! the lines that were ignored due to parsing errors.
//!
//! The database can be provided as a string using the [`Parser::load_from_string`] method
//! or as a file using the [`Parser::load_from_file`] method.
//!
//! ### Returned value
//! The returned value depend on the method used to parse the PlayOnBSD database.
//!
//! The [`Parser::load_from_string`] method returns an [`ParserResult`] enum. It has to variants:
//! * [`ParserResult::WithoutError`] holding a vector of [`Game`] object;
//! * [`ParserResult::WithError`] holding a vector of [`Game`] objects as well as
//! a vector of [`usize`] where each element is the number of a line ignored during parsing
//! due to parsing errors.
//!
//! The [`Parser::load_from_file`] method returns [`Result`]<[`ParserResult`], [`std::io::Error`]>.
//!
//! ### Example
//!
//! ```no_run
//! use pobsdrs_parser::{Parser, ParserResult};
//!
//! // Print the database
//! fn main() -> Result<(), std::io::Error> {
//!     let result = Parser::default().load_from_file("tests/data/good-data.db")?;
//!     match result {
//!         ParserResult::WithoutError(games) => {
//!             for game in games {
//!                 println!("{}", game);
//!             }
//!         },
//!         ParserResult::WithError(_, _) => println!("Parsing errors"),
//!     }
//!     Ok(())
//! }
//! ```
//! # Game
//! pobsdrs-parser provides a [`Game`] struct representing a
//! game from the database with an additional [`Game::id`] field which
//!  represents the position in the database.
//!
//! The name of some fields differs from the one used
//! in the database itself: Genre and Store are plural
//! since there can be more than one item for each
//! and Pub translate to publi since pub is a reserved
//! keyword in Rust.
//!
//! All fields are optional strings or vectors of strings
//! except for the name of the game which is mandatory.
//! The parser does not try to be smart with dates and
//! just store them as string.
//!
//! ### Display
//! The [`Game`] struct implement the [`core::fmt::Display`] trait
//! and will be displayed as it would appear in the
//! PlayOnBSD database.
//!
//! ### PartialOrd
//! The [`Game`] struct implements the [`core::cmp::PartialOrd`] trait
//! and [`Game`] objects are ordered according to their
//! [`Game::id`] which correspond to the game position in
//! the PlayOnBSD database.

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
mod parser_macros;
pub mod field;
pub mod game;
pub mod parser;
mod split_line;

pub use crate::field::Field;
pub use crate::game::Game;
pub use crate::parser::Parser;
pub use crate::parser::ParserResult;
pub use crate::parser::ParsingMode;
