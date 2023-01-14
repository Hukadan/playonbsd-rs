use crate::field::Field;
use crate::game::Game;

pub trait State {}

enum ParserState {
    Game,
    Cover,
    Engine,
    Setup,
    Runtime,
    Store,
    Hints,
    Genre,
    Tags,
    Year,
    Dev,
    Pub,
    Version,
    Status,
    Added,
    Updated,
    Error,
}

pub enum ParsingMode {
    Strict,
    Relaxed,
}

pub enum ParserResult {
    WithError(Vec<Game>, Vec<usize>),
    WithoutError(Vec<Game>),
}

impl Into<Vec<Game>> for ParserResult {
    fn into(self) -> Vec<Game> {
        match self {
            ParserResult::WithError(games, _) => games,
            ParserResult::WithoutError(games) => games,
        }
    }
}

pub struct Parser {
    state: ParserState,
    games: Vec<Game>,
    mode: ParsingMode,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            state: ParserState::Game,
            games: Vec::new(),
            mode: ParsingMode::Relaxed,
        }
    }
}
impl Parser {
    pub fn new(mode: ParsingMode) -> Self {
        Self {
            state: ParserState::Game,
            games: Vec::new(),
            mode,
        }
    }
    pub fn load_from_string(mut self, data: &str) -> ParserResult {
        let mut has_error = false;
        let mut lines: Vec<usize> = Vec::new();
        let mut counter = 0;
        for line in data.lines() {
            counter += 1;
            self.parse(line);
            // Check for parsing error. Only stop if in Strict mode
            // Otherwise continues but keeps track of the ignored lines
            if let ParserState::Error = self.state {
                lines.push(counter);
                match self.mode {
                    ParsingMode::Strict => break,
                    ParsingMode::Relaxed => {
                        has_error = true;
                        eprintln!("Error while parsing... Trying my best to recover")
                    }
                }
            }
        }
        match has_error {
            true => ParserResult::WithError(self.games, lines),
            false => ParserResult::WithoutError(self.games),
        }
    }
    impl_parse![ParserState::Game, Field::Game, name, ParserState::Cover;
         (ParserState::Cover, Field::Cover, cover, ParserState::Engine);
         (ParserState::Engine, Field::Engine, engine, ParserState::Setup);
         (ParserState::Setup, Field::Setup, setup, ParserState::Runtime);
         (ParserState::Runtime, Field::Runtime, runtime, ParserState::Store);
         (ParserState::Store, Field::Store, stores, ParserState::Hints);
         (ParserState::Hints, Field::Hints, hints, ParserState::Genre);
         (ParserState::Genre, Field::Genres, genres, ParserState::Tags);
         (ParserState::Tags, Field::Tags, tags, ParserState::Year);
         (ParserState::Year, Field::Year, year, ParserState::Dev);
         (ParserState::Dev, Field::Dev, dev, ParserState::Pub);
         (ParserState::Pub, Field::Publi, publi, ParserState::Version);
         (ParserState::Version, Field::Version, version, ParserState::Status);
         (ParserState::Status, Field::Status, status, ParserState::Added);
         (ParserState::Added, Field::Added, added, ParserState::Updated);
         (ParserState::Updated, Field::Updated, updated, ParserState::Game)
    ];
}
