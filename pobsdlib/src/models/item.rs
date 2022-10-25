use std::cmp::{Ordering, PartialOrd};
use std::fmt;

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub games: Vec<usize>,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name\t{}", self.name)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Item) -> Option<Ordering> {
        self.name.partial_cmp(&other.name)
    }
    fn lt(&self, other: &Item) -> bool {
        self.name.lt(&other.name)
    }
    fn le(&self, other: &Item) -> bool {
        self.name.le(&other.name)
    }
    fn gt(&self, other: &Item) -> bool {
        self.name.gt(&other.name)
    }
    fn ge(&self, other: &Item) -> bool {
        self.name.ge(&other.name)
    }
}
