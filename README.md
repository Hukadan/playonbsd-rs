[![Rust](https://github.com/Hukadan/pobsdjs/actions/workflows/rust.yml/badge.svg)](https://github.com/Hukadan/pobsdjs/actions/workflows/rust.yml)
[![rust-clippy analyze](https://github.com/Hukadan/pobsdjs/actions/workflows/rust-clippy.yml/badge.svg)](https://github.com/Hukadan/pobsdjs/actions/workflows/rust-clippy.yml)

# pobsd-rs
pobsd-rs is a toy project I use to learn Rust. It aims to provide
a set of tools to interact with the PlayOnBSD database which lists
games that can be played on OpenBSD.

As the moment, the project provides:
* posbsdrs-parser which is a really simple parser to
read the PlayOnBSD database and converts it into
a vector of Games;
* posbsdlib which is a parser/db(kind of) to read
the PlayOnBSD database and converts it into a 
queryable database (should be deprecated soon in 
favor of a furture project named pobsdrs-db);
 * pobsdrs-sever which provides a minimal web server
to browse the PlayOnBSD database.

Please refer to their respective README for more
information.