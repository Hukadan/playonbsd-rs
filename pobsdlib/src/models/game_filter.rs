#[derive(PartialEq, Eq, Debug, Default, Clone)]
pub struct GameFilter<'a> {
    pub(crate) name: Option<&'a str>,
    pub(crate) engine: Option<&'a str>,
    pub(crate) runtime: Option<&'a str>,
    pub(crate) genre: Option<&'a str>,
    pub(crate) tag: Option<&'a str>,
    pub(crate) year: Option<&'a str>,
    pub(crate) dev: Option<&'a str>,
    pub(crate) publi: Option<&'a str>,
}

impl<'a> GameFilter<'a> {
    pub fn new() -> Self {
        GameFilter::default()
    }
    pub fn name_contains(&mut self, pattern: &'a str) {
        self.name = Some(pattern);
    }
    pub fn engine_contains(&mut self, pattern: &'a str) {
        self.engine = Some(pattern);
    }
    pub fn runtime_contains(&mut self, pattern: &'a str) {
        self.runtime = Some(pattern);
    }
    pub fn genre_contains(&mut self, pattern: &'a str) {
        self.genre = Some(pattern);
    }
    pub fn tag_contains(&mut self, pattern: &'a str) {
        self.tag = Some(pattern);
    }
    pub fn year_contains(&mut self, pattern: &'a str) {
        self.year = Some(pattern);
    }
    pub fn dev_contains(&mut self, pattern: &'a str) {
        self.dev = Some(pattern);
    }
    pub fn publi_contains(&mut self, pattern: &'a str) {
        self.publi = Some(pattern);
    }
}
