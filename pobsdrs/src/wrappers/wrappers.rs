use pobsdlib::{Game, QueryResult};
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct ItemWrapper<'a> {
    name: &'a str,
    // the url to the item
    #[serde(rename = "self")]
    self_url: String,
    // the url to the games associated to the item
    games: String,
}

impl<'a> ItemWrapper<'a> {
    pub(crate) fn new(name: &'a str, self_url: String, games: String) -> Self {
        ItemWrapper {
            name,
            self_url,
            games,
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct GameWrapper<'a> {
    id: usize,
    name: &'a String,
    cover: &'a Option<String>,
    setup: &'a Option<String>,
    hints: &'a Option<String>,
    version: &'a Option<String>,
    status: &'a Option<String>,
    stores: &'a Option<Vec<String>>,
    engine: Option<ItemWrapper<'a>>,
    runtime: Option<ItemWrapper<'a>>,
    year: Option<ItemWrapper<'a>>,
    dev: Option<ItemWrapper<'a>>,
    #[serde(rename = "pub")]
    publi: Option<ItemWrapper<'a>>,
    genres: Option<Vec<ItemWrapper<'a>>>,
    tags: Option<Vec<ItemWrapper<'a>>>,
    #[serde(rename = "self")]
    self_url: String,
}
impl<'a> GameWrapper<'a> {
    pub(crate) fn new(game: &'a Game, domain: &'a String) -> Self {
        let wrapped_engine: Option<ItemWrapper>;
        let wrapped_runtime: Option<ItemWrapper>;
        let wrapped_year: Option<ItemWrapper>;
        let wrapped_dev: Option<ItemWrapper>;
        let wrapped_publi: Option<ItemWrapper>;
        let wrapped_genres: Option<Vec<ItemWrapper>>;
        let wrapped_tags: Option<Vec<ItemWrapper>>;
        match &game.engine {
            Some(engine) => {
                wrapped_engine = Some(ItemWrapper::new(
                    engine,
                    format!("{}/api/engines/{}", domain, engine),
                    format!("{}/api/games/search?engine={}", domain, engine),
                ))
            }
            None => wrapped_engine = None,
        }
        match &game.runtime {
            Some(runtime) => {
                wrapped_runtime = Some(ItemWrapper::new(
                    runtime,
                    format!("{}/api/runtimes/{}", domain, runtime),
                    format!("{}/api/games/search?runtime={}", domain, runtime),
                ))
            }
            None => wrapped_runtime = None,
        }
        match &game.year {
            Some(year) => {
                wrapped_year = Some(ItemWrapper::new(
                    year,
                    format!("{}/api/years/{}", domain, year),
                    format!("{}/api/games/search?year={}", domain, year),
                ))
            }
            None => wrapped_year = None,
        }
        match &game.dev {
            Some(dev) => {
                wrapped_dev = Some(ItemWrapper::new(
                    dev,
                    format!("{}/api/devs/{}", domain, dev),
                    format!("{}/api/games/search?dev={}", domain, dev),
                ))
            }
            None => wrapped_dev = None,
        }
        match &game.publi {
            Some(publi) => {
                wrapped_publi = Some(ItemWrapper::new(
                    publi,
                    format!("{}/api/pubs/{}", domain, publi),
                    format!("{}/api/games/search?pub={}", domain, publi),
                ))
            }
            None => wrapped_publi = None,
        }
        match &game.genres {
            Some(genres) => {
                let mut items_to_wrap: Vec<ItemWrapper> = Vec::new();
                for genre in genres {
                    items_to_wrap.push(ItemWrapper::new(
                        genre,
                        format!("{}/api/genres/{}", domain, genre),
                        format!("{}/api/games/search?genre={}", domain, genre),
                    ))
                }
                wrapped_genres = Some(items_to_wrap);
            }
            None => wrapped_genres = None,
        }
        match &game.tags {
            Some(tags) => {
                let mut items_to_wrap: Vec<ItemWrapper> = Vec::new();
                for tag in tags {
                    items_to_wrap.push(ItemWrapper::new(
                        tag,
                        format!("{}/api/tags/{}", domain, tag),
                        format!("{}/api/games/search?tag={}", domain, tag),
                    ))
                }
                wrapped_tags = Some(items_to_wrap);
            }
            None => wrapped_tags = None,
        }
        Self {
            id: game.id,
            name: &game.name,
            cover: &game.cover,
            setup: &game.setup,
            hints: &game.hints,
            version: &game.version,
            status: &game.status,
            stores: &game.stores,
            engine: wrapped_engine,
            runtime: wrapped_runtime,
            year: wrapped_year,
            dev: wrapped_dev,
            publi: wrapped_publi,
            genres: wrapped_genres,
            tags: wrapped_tags,
            self_url: format!("{}/api/games/{}", domain, game.id),
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub(crate) struct QueryResultWrapper<'a> {
    count: usize,
    items: Vec<GameWrapper<'a>>,
}

impl<'a> QueryResultWrapper<'a> {
    pub(crate) fn new(qr: QueryResult<&'a Game>, domain: &'a String) -> Self {
        let mut items: Vec<GameWrapper> = Vec::new();
        for item in &qr.items {
            items.push(GameWrapper::new(&item, domain))
        }
        Self {
            count: qr.count,
            items,
        }
    }
}
