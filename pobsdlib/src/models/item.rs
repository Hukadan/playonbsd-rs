use std::cmp::{Ordering, PartialOrd};
use std::fmt;

/// Items are attributes to which several games can
/// be associated with such as genres, tags or years.
///
/// An Item contains the name of the item and a
/// vector containing the ids of the games associated
/// to said item.
///
#[derive(Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct Item {
    /// name of the item.
    pub name: String,
    /// vector of ids of the games associated to the items.
    pub games: Vec<u64>,
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
