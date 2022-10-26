## pobsdlib
pobsdlib makes parsing the PlayOnBSD database easy.
Once loaded with pobsdlib, you can query the database
using based on most of the fields.

### Example
```rust
extern crate pobsdlib;
use pobsdlib::collections::DataBase;

let db = DataBase::new("tests/data/test-games.db");
// return a list of all games in the form of
// a Game struct.
let games = db.get_all_games();
// When loaded, each game is given an id
// representing it position in alphabetic
// order (discarding the starting The/A).
// The get_game_by_id return an option.
if let Some(game) = db_game.get_game_by_id(2) {
    assert_eq!(game.name, "The Adventures of Shuggy".to_string());
}
```
More complicated query are possible (see doc.rs).
