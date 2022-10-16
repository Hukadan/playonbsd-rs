use crate::utils::split_line;

/* ------------------------ FIELD ENUM -----------------------*/
/// # Represent a field generated form a line of the game database
///
#[derive(PartialEq, Debug, Clone)]
pub enum Field<'a> {
    Game(&'a str),
    Cover(&'a str),
    Engine(&'a str),
    Setup(&'a str),
    Runtime(&'a str),
    Hints(&'a str),
    Dev(&'a str),
    Publi(&'a str),
    Version(&'a str),
    Status(&'a str),
    Store(Vec<&'a str>),
    Genres(Vec<&'a str>),
    Tags(Vec<&'a str>),
    Year(&'a str),
    Unknown(&'a str),
}

impl<'a> Field<'a> {
    /// Try to convert a line of the database in a Field enum (see exemple above).
    /// Panic if it cannot.
    /// ```
    /// use pobsdlib::models::Field;
    ///
    /// let line_str = "Tags\ttag1, tag2";
    /// let field = Field::from(line_str);
    ///
    /// assert_eq!(field,Field::Tags(vec![&"tag1",&"tag2"]));
    /// ```
    pub fn from(line: &'a str) -> Self {
        // Split the line in a left and right hand sides
        let (left, right) = split_line(line);
        // Use the left hand side to discriminate between single and multiple item lines
        match left {
            "Game" => Field::Game(right),
            "Cover" => Field::Cover(right),
            "Engine" => Field::Engine(right),
            "Setup" => Field::Setup(right),
            "Runtime" => Field::Runtime(right),
            "Hints" => Field::Hints(right),
            "Dev" => Field::Dev(right),
            "Pub" => Field::Publi(right),
            "Version" => Field::Version(right),
            "Status" => Field::Status(right),
            // Store does not use the same separator than Genre and Tags
            "Store" => {
                let mut items: Vec<&str> = Vec::new();
                for item in right.split(' ') {
                    items.push(item.trim());
                }
                Field::Store(items)
            }
            "Genre" => {
                let mut items: Vec<&str> = Vec::new();
                for item in right.split(',') {
                    items.push(item.trim());
                }
                Field::Genres(items)
            }
            "Tags" => {
                let mut items: Vec<&str> = Vec::new();
                for item in right.split(',') {
                    items.push(item.trim());
                }
                Field::Tags(items)
            }
            "Year" => Field::Year(right),
            _ => Field::Unknown(left),
        }
    }
}

#[cfg(test)]
mod test_methods {
    use super::*;
    #[test]
    fn from_game_line() {
        let input = "Game\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Game(&"Toto"), field);
    }
    #[test]
    fn from_cover_line() {
        let input = "Cover\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Cover(&"Toto"), field);
    }
    #[test]
    fn from_engine_line() {
        let input = "Engine\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Engine(&"Toto"), field);
    }
    #[test]
    fn from_setup_line() {
        let input = "Setup\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Setup(&"Toto"), field);
    }
    #[test]
    fn from_runtime_line() {
        let input = "Runtime\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Runtime(&"Toto"), field);
    }
    #[test]
    fn from_hints_line() {
        let input = "Hints\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Hints(&"Toto"), field);
    }
    #[test]
    fn from_store_line() {
        let input = "Store\tfirst second";
        let field = Field::from(&input);
        assert_eq!(Field::Store(vec![&"first", &"second"]), field);
    }
    #[test]
    fn from_genre_line() {
        let input = "Genre\tfirst, second";
        let field = Field::from(&input);
        assert_eq!(Field::Genres(vec![&"first", &"second"]), field);
    }
    #[test]
    fn from_tag_line() {
        let input = "Tags\tfirst, second";
        let field = Field::from(&input);
        assert_eq!(Field::Tags(vec![&"first", &"second"]), field);
    }
    #[test]
    fn from_malformed_line() {
        let input = "Let's not\tpanic";
        let field = Field::from(&input);
        assert_eq!(Field::Unknown(&"Let's not"), field);
    }
}
