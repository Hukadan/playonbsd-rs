extern crate pobsdlib;
use pobsdlib::utils::database_builder::DataBaseBuilder;
use std::{env, path, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments");
        process::exit(1);
    }
    if args.len() > 2 {
        eprintln!("Too many arguments");
        process::exit(1);
    }
    let path = path::Path::new(&args[1]);
    if path.is_file() {
        let game_db = DataBaseBuilder::new(true, true).build_from_file(&args[1]);
        let games = game_db.get_all_games();
        for mut game in games.items {
            if let Some(date) = game.added {
                game.added = Some(date);
            }
            println!("{}", game);
        }
    } else {
        eprintln!("This is not a file");
    }
}
