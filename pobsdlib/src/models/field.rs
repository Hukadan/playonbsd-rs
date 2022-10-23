use crate::utils::split_line;
use std::fmt;

/* ------------------------ FIELD ENUM -----------------------*/
/// Represent a field generated form a line of the game database
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
    Unknown(Option<&'a str>, Option<&'a str>),
}

impl fmt::Display for Field<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::Game(name) => {
                match name {
                    Some(name) => write!(f, "Game\t{}", name),
                    None => write!(f, "Game"),
                }
            }
            Field::Cover(name) => {
                match name {
                    Some(name) => write!(f, "Cover\t{}", name),
                    None => write!(f, "Cover"),
                }
            }
            Field::Engine(name) => {
                match name {
                    Some(name) => write!(f, "Engine\t{}", name),
                    None => write!(f, "Engine"),
                }
            }
            Field::Setup(name) => {
                match name {
                    Some(name) => write!(f, "Setup\t{}", name),
                    None => write!(f, "Setup"),
                }
            }
            Field::Runtime(name) => {
                match name {
                    Some(name) => write!(f, "Runtime\t{}", name),
                    None => write!(f, "Runtime"),
                }
            }
            Field::Hints(name) => {
                match name {
                    Some(name) => write!(f, "Hints\t{}", name),
                    None => write!(f, "Hints"),
                }
            }
            Field::Dev(name) => {
                match name {
                    Some(name) => write!(f, "Dev\t{}", name),
                    None => write!(f, "Dev"),
                }
            }
            Field::Publi(name) => {
                match name {
                    Some(name) => write!(f, "Pub\t{}", name),
                    None => write!(f, "Pub"),
                }
            }
            Field::Version(name) => {
                match name {
                    Some(name) => write!(f, "Version\t{}", name),
                    None => write!(f, "Version"),
                }
            }
            Field::Status(name) => {
                match name {
                    Some(name) => write!(f, "Status\t{}", name),
                    None => write!(f, "Status"),
                }
            }
            Field::Store(name) => {
                match name {
                    Some(name) => write!(f, "Store\t{}", name.join(" ")),
                    None => write!(f, "Store"),
                }
            }
            Field::Genres(name) => {
                match name {
                    Some(name) => write!(f, "Genre\t{}", name.join(", ")),
                    None => write!(f, "Genre"),
                }
            }
            Field::Tags(name) => {
                match name {
                    Some(name) => write!(f, "Tags\t{}", name.join(", ")),
                    None => write!(f, "Tags"),
                }
            }
            Field::Year(name) => {
                match name {
                    Some(name) => write!(f, "Year\t{}", name),
                    None => write!(f, "Year"),
                }
            }
            Field::Unknown(left, right) => {
                match right {
                    Some(right) => write!(f, "Unknown\t{}\t{}", left.unwrap(), right),
                    None => write!(f, "Unknown\t{}", left.unwrap()),
                }
            }
        }
    }
}

impl<'a> Field<'a> {
    /// Convert a line of the database in a Field enum (see exemple above).
    /// Return Field::Unknown if the field is not recognized.
    /// ```
    /// use pobsdlib::models::Field;
    ///
    /// let runtime_str = "Runtime\truntime name";
    /// let runtime_field = Field::from(runtime_str);
    /// assert_eq!(runtime_field,Field::Runtime(Some(&"runtime name")));
    ///
    /// let tag_str = "Tags\ttag1, tag2";
    /// let tag_field = Field::from(tag_str);
    /// assert_eq!(tag_field,Field::Tags(Some(vec![&"tag1",&"tag2"])));
    ///
    /// let uk_str = "not a field\tnot an entry";
    /// let uk_field = Field::from(uk_str);
    /// assert_eq!(uk_field,Field::Unknown(Some("not a field"), Some("not an entry")));
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
                    Some(right) => Field::Unknown(Some(left), Some(right)),
                    None => Field::Unknown(Some(left), None),
                },
            }
        } else {
            Field::Unknown(Some("Unknown"),None)
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
        assert_eq!(format!("{}", field), input);
        let input = "Game";
        let field = Field::from(&input);
        assert_eq!(Field::Game(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn from_cover_line() {
        let input = "Cover\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Cover(Some(&"Toto")), field);
        assert_eq!(format!("{}", field), input);
        let input = "Cover";
        let field = Field::from(&input);
        assert_eq!(Field::Cover(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn from_engine_line() {
        let input = "Engine\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Engine(Some(&"Toto")), field);
        assert_eq!(format!("{}", field), input);
        let input = "Engine";
        let field = Field::from(&input);
        assert_eq!(Field::Engine(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn from_setup_line() {
        let input = "Setup\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Setup(Some(&"Toto")), field);
        assert_eq!(format!("{}", field), input);
        let input = "Setup";
        let field = Field::from(&input);
        assert_eq!(Field::Setup(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn from_runtime_line() {
        let input = "Runtime\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Runtime(Some(&"Toto")), field);
        assert_eq!(format!("{}", field), input);
        let input = "Runtime";
        let field = Field::from(&input);
        assert_eq!(Field::Runtime(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn from_hints_line() {
        let input = "Hints\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Hints(Some(&"Toto")), field);
        assert_eq!(format!("{}", field), input);
        let input = "Hints";
        let field = Field::from(&input);
        assert_eq!(Field::Hints(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn from_dev_line() {
        let input = "Dev\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Dev(Some(&"Toto")), field);
        assert_eq!(format!("{}", field), input);
        let input = "Dev";
        let field = Field::from(&input);
        assert_eq!(Field::Dev(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn from_publi_line() {
        let input = "Pub\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Publi(Some(&"Toto")), field);
        assert_eq!(format!("{}", field), input);
        let input = "Pub";
        let field = Field::from(&input);
        assert_eq!(Field::Publi(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn from_version_line() {
        let input = "Version\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Version(Some(&"Toto")), field);
        assert_eq!(format!("{}", field), input);
        let input = "Version";
        let field = Field::from(&input);
        assert_eq!(Field::Version(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn from_status_line() {
        let input = "Status\tToto";
        let field = Field::from(&input);
        assert_eq!(Field::Status(Some(&"Toto")), field);
        assert_eq!(format!("{}", field), input);
        let input = "Status";
        let field = Field::from(&input);
        assert_eq!(Field::Status(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn from_store_line() {
        let input = "Store\tfirst second";
        let field = Field::from(&input);
        assert_eq!(Field::Store(Some(vec![&"first", &"second"])), field);
        assert_eq!(format!("{}", field), input);
        let input = "Store";
        let field = Field::from(&input);
        assert_eq!(Field::Store(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn from_genre_line() {
        let input = "Genre\tfirst, second";
        let field = Field::from(&input);
        assert_eq!(Field::Genres(Some(vec![&"first", &"second"])), field);
        assert_eq!(format!("{}", field), input);
        let input = "Genre";
        let field = Field::from(&input);
        assert_eq!(Field::Genres(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn from_tag_line() {
        let input = "Tags\tfirst, second";
        let field = Field::from(&input);
        assert_eq!(Field::Tags(Some(vec![&"first", &"second"])), field);
        assert_eq!(format!("{}", field), input);
        let input = "Tags";
        let field = Field::from(&input);
        assert_eq!(Field::Tags(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn from_year_line() {
        let input = "Year\t1980";
        let field = Field::from(&input);
        assert_eq!(Field::Year(Some(&"1980")), field);
        assert_eq!(format!("{}", field), input);
        let input = "Year";
        let field = Field::from(&input);
        assert_eq!(Field::Year(None), field);
        assert_eq!(format!("{}", field), input);
    }
    #[test]
    fn from_malformed_line() {
        let input = "Let's not\tpanic";
        let field = Field::from(&input);
        assert_eq!(Field::Unknown(Some(&"Let's not"),Some(&"panic")), field);
        assert_eq!(format!("{}", field), format!("Unknown\t{}",input));
    }
    #[test]
    fn from_malformed_line_notab() {
        let input = "Let's not";
        let field = Field::from(&input);
        assert_eq!(Field::Unknown(Some(&"Let's not"),None), field);
        assert_eq!(format!("{}", field), format!("Unknown\t{}", input));
    }
}
