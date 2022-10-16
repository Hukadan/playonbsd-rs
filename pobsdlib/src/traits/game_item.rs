use crate::traits::BasicItem;

pub trait GameItem: BasicItem {
    fn get_tags(&self) -> &Vec<String>;
    fn get_genres(&self) -> &Vec<String>;
}
