use crate::collections::DataBase;
use crate::models::Field;
use crate::utils::{game_dispatch, read_lines};

pub struct DataBaseBuilder {
    expand_cover: bool,
    steam_cover: bool,
}

impl DataBaseBuilder {
    pub fn new(expand_cover: bool, steam_cover: bool) -> Self {
        Self {
            expand_cover,
            steam_cover,
        }
    }
    pub fn set_expand_cover(&mut self, setting: bool) {
        self.expand_cover = setting;
    }
    pub fn set_steam_cover(&mut self, setting: bool) {
        self.steam_cover = setting;
    }
    pub fn build_from_string(self, data: String) -> DataBase {
        let mut counter: usize = 0;
        let mut database = DataBase::default();
        for line in data.lines() {
            counter = game_dispatch(
                Field::from(&line),
                &mut database,
                self.expand_cover,
                self.steam_cover,
                counter,
            );
        }
        database
    }
    pub fn build_from_file(self, filename: &str) -> DataBase {
        let mut counter: usize = 0;
        let mut database = DataBase::default();
        if let Ok(lines) = read_lines(filename) {
            for line in lines.flatten() {
                counter = game_dispatch(
                    Field::from(&line),
                    &mut database,
                    self.expand_cover,
                    self.steam_cover,
                    counter,
                );
            }
        }
        database
    }
}
