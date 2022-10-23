use std::fmt;

#[derive(Serialize, Clone, Default, Debug, PartialEq)]
pub struct Item {
    pub id: usize,
    pub name: String,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
       write!(f, "Name\t{}", self.name)
    }
}
