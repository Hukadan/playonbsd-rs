use crate::collections::DataBase;
use crate::models::Field;
use crate::utils::{game_dispatch, read_lines};

pub fn load_database(database: &mut DataBase, filename: &str) {
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            game_dispatch(Field::from(&line), database, false, false);
        }
    }
}

pub fn load_database_from_string(database: &mut DataBase, data: String) {
    for line in data.lines() {
        game_dispatch(Field::from(&line), database, true, true);
    }
}
