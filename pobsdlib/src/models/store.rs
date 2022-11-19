use std::fmt;

#[derive(Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub enum Store {
    Steam(String),
    HumbleBundle(String),
    Gog(String),
    Hitchio(String),
    Other(String),
    #[default]
    NoStore,
}

impl fmt::Display for Store {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Store::Steam(url) => write!(f, "{}", url),
            Store::HumbleBundle(url) => write!(f, "{}", url),
            Store::Gog(url) => write!(f, "{}", url),
            Store::Hitchio(url) => write!(f, "{}", url),
            Store::Other(url) => write!(f, "{}", url),
            Store::NoStore => write!(f, "No store"),
        }
    }
}

impl Store {
    pub fn new(url: &str) -> Self {
        if url.contains("steampowered") {
            Self::Steam(url.to_string())
        } else if url.contains("humblebundle") {
            Self::HumbleBundle(url.to_string())
        } else {
            Self::Other(url.to_string())
        }
    }
}