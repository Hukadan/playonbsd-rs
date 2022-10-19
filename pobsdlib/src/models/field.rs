use crate::utils::split_line;

/* ------------------------ FIELD ENUM -----------------------*/
/// # Represent a field generated form a line of the game database
///
#[derive(PartialEq, Debug, Clone)]
pub enum Field<'a> {
    Game(Option<&'a str>),
    Cover(Option<&'a str>),
    Engine(Option<&'a str>),
    Setup(Option<&'a str>),
    Runtime(Option<&'a str>),
    Hints(Option<&'a str>),
    Dev(Option<&'a str>),
    Publi(Option<&'a str>),
    Version(Option<&'a str>),
    Status(Option<&'a str>),
    Store(Option<Vec<&'a str>>),
    Genres(Option<Vec<&'a str>>),
    Tags(Option<Vec<&'a str>>),
    Year(Option<&'a str>),
    Unknown(Option<&'a str>),
}

impl<'a> Field<'a> {
    /// Convert a line of the database in a Field enum (see exemple above).
    /// Return Field::Unknown if the field is not recognized.
    /// ```
    /// use pobsdlib::models::Field;
    ///
    /// let line_str = "Tags\ttag1, tag2";
    /// let field = Field::from(line_str);
    ///
    /// assert_eq!(field,Field::Tags(Some(vec![&"tag1",&"tag2"])));
    /// ```
    pub fn from(line: &'a str) -> Self {
        // Split the line in a left and right hand sides
        let (left, right) = split_line(line);
        // Use the left hand side to discriminate between single and multiple item lines
        if let Some(left) = left {
            match left {
                "Game" => match right {
                    Some(right) => Field::Game(Some(right)),
                    None => Field::Game(None),
                },
                "Cover" => match right {
                    Some(right) => Field::Cover(Some(right)),
                    None => Field::Cover(None),
                },
                "Engine" => match right {
                    Some(right) => Field::Engine(Some(right)),
                    None => Field::Engine(None),
                },
                "Setup" => match right {
                    Some(right) => Field::Setup(Some(right)),
                    None => Field::Setup(None),
                },
                "Runtime" => match right {
                    Some(right) => Field::Runtime(Some(right)),
                    None => Field::Runtime(None),
                },
                "Hints" => match right {
                    Some(right) => Field::Hints(Some(right)),
                    None => Field::Hints(None),
                },
                "Dev" => match right {
                    Some(right) => Field::Dev(Some(right)),
                    None => Field::Dev(None),
                },
                "Pub" => match right {
                    Some(right) => Field::Publi(Some(right)),
                    None => Field::Publi(None),
                },
                "Version" => match right {
                    Some(right) => Field::Version(Some(right)),
                    None => Field::Version(None),
                },
                "Status" => match right {
                    Some(right) => Field::Status(Some(right)),
                    None => Field::Status(None),
                },
                // Store does not use the same separator than Genre and Tags
                "Store" => match right {
                    Some(right) => {
                        let mut items: Vec<&str> = Vec::new();
                        for item in right.split(' ') {
                            items.push(item.trim());
                        }
                        Field::Store(Some(items))
                    }
                    None => Field::Store(None),
                },
                "Genre" => match right {
                    Some(right) => {
                        let mut items: Vec<&str> = Vec::new();
                        for item in right.split(',') {
                            items.push(item.trim());
                        }
                        Field::Genres(Some(items))
                    }
                    None => Field::Genres(None),
                },
                "Tags" => match right {
                    Some(right) => {
                        let mut items: Vec<&str> = Vec::new();
                        for item in right.split(',') {
                            items.push(item.trim());
                        }
                        Field::Tags(Some(items))
                    }
                    None => Field::Tags(None),
                },
                "Year" => match right {
                    Some(right) => Field::Year(Some(right)),
                    None => Field::Year(None),
                },
                _ => match right {
                    Some(right) => Field::Unknown(Some(right)),
                    None => Field::Unknown(None),
                },
            }
        } else {
            Field::Unknown(None)
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
        assert_eq!(Field::Game(Some(&"Toto")), field);
        let input = "Game";
        let field = Field::from(&input);
        assert_eq!(Field::Game(None), field);
    }
    #[test]
    fn from_cover_line() {
        let input = "Cover\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Cover(Some(&"Toto")), field);
        let input = "Cover";
        let field = Field::from(&input);
        assert_eq!(Field::Cover(None), field);
        let field = Field::from(&input);
        assert_eq!(Field::Cover(None), field);
    }
    #[test]
    fn from_engine_line() {
        let input = "Engine\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Engine(Some(&"Toto")), field);
        let input = "Engine";
        let field = Field::from(&input);
        assert_eq!(Field::Engine(None), field);
    }
    #[test]
    fn from_setup_line() {
        let input = "Setup\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Setup(Some(&"Toto")), field);
        let input = "Setup";
        let field = Field::from(&input);
        assert_eq!(Field::Setup(None), field);
    }
    #[test]
    fn from_runtime_line() {
        let input = "Runtime\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Runtime(Some(&"Toto")), field);
        let input = "Runtime";
        let field = Field::from(&input);
        assert_eq!(Field::Runtime(None), field);
    }
    #[test]
    fn from_hints_line() {
        let input = "Hints\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Hints(Some(&"Toto")), field);
        let input = "Hints";
        let field = Field::from(&input);
        assert_eq!(Field::Hints(None), field);
    }
    #[test]
    fn from_dev_line() {
        let input = "Dev\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Dev(Some(&"Toto")), field);
        let input = "Dev";
        let field = Field::from(&input);
        assert_eq!(Field::Dev(None), field);
    }
    #[test]
    fn from_publi_line() {
        let input = "Pub\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Publi(Some(&"Toto")), field);
        let input = "Pub";
        let field = Field::from(&input);
        assert_eq!(Field::Publi(None), field);
    }
    #[test]
    fn from_version_line() {
        let input = "Version\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Version(Some(&"Toto")), field);
        let input = "Version";
        let field = Field::from(&input);
        assert_eq!(Field::Version(None), field);
    }
    #[test]
    fn from_status_line() {
        let input = "Status\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Status(Some(&"Toto")), field);
        let input = "Status";
        let field = Field::from(&input);
        assert_eq!(Field::Status(None), field);
    }
    #[test]
    fn from_store_line() {
        let input = "Store\tfirst second";
        let field = Field::from(&input);
        assert_eq!(Field::Store(Some(vec![&"first", &"second"])), field);
        let input = "Store";
        let field = Field::from(&input);
        assert_eq!(Field::Store(None), field);
    }
    #[test]
    fn from_genre_line() {
        let input = "Genre\tfirst, second";
        let field = Field::from(&input);
        assert_eq!(Field::Genres(Some(vec![&"first", &"second"])), field);
        let input = "Genre";
        let field = Field::from(&input);
        assert_eq!(Field::Genres(None), field);
    }
    #[test]
    fn from_tag_line() {
        let input = "Tags\tfirst, second";
        let field = Field::from(&input);
        assert_eq!(Field::Tags(Some(vec![&"first", &"second"])), field);
        let input = "Tags";
        let field = Field::from(&input);
        assert_eq!(Field::Tags(None), field);
    }
    #[test]
    fn from_year_line() {
        let input = "Year\t1980";
        let field = Field::from(&input);
        assert_eq!(Field::Year(Some(&"1980")), field);
        let input = "Year";
        let field = Field::from(&input);
        assert_eq!(Field::Year(None), field);
    }
    #[test]
    fn from_malformed_line() {
        let input = "Let's not\tpanic";
        let field = Field::from(&input);
        assert_eq!(Field::Unknown(Some(&"panic")), field);
    }
    #[test]
    fn from_malformed_line_notab() {
        let input = "Let's not";
        let field = Field::from(&input);
        assert_eq!(Field::Unknown(None), field);
    }
}
