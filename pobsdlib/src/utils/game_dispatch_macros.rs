macro_rules! field_dispatch {
    (newgame $n: expr, $db: expr, $cr: expr) => {
        if let Some(name) = $n {
            $cr.counter += 1;
            let mut game = Game::default();
            let mut hasher = DefaultHasher::new();
            name.hash(&mut hasher);
            $cr.uuid = hasher.finish();
            game.name = name.to_string();
            game.id = $cr.counter;
            game.uuid = $cr.uuid;
            $db.games.insert($cr.uuid, game);
        } else {
            eprintln!(
                "Error trying to insert game with id: {}.",
                $db.games.len() + 1
            );
        };
    };
    ($field: ident, $n: expr, $db: expr, $cr: expr) => {
        if let Some(name) = $n {
            if let Some(game) = $db.games.get_mut(&$cr.uuid) {
                game.$field = Some(name.to_string());
            };
        };
    };
    ($field: ident in $sdb: ident, $n: expr, $db: expr, $cr: expr) => {
        if let Some(name) = $n {
            if let Some(game) = $db.games.get_mut(&$cr.uuid) {
                game.$field = Some(name.to_string());
            };
            $db.$sdb
                .entry(name.to_string())
                .and_modify(|e| e.games.push($cr.uuid))
                .or_insert(Item {
                    name: name.to_string(),
                    games: vec![$cr.uuid],
                });
        };
    };
    ($field: ident, with_items $n: expr, $db: expr, $cr: expr) => {
        if let Some(items) = $n {
            if let Some(game) = $db.games.get_mut(&$cr.uuid) {
                for item in items {
                    match &mut game.$field {
                        Some(stores) => stores.push(item.to_string()),
                        None => {
                            game.$field = Some(vec![item.to_string()]);
                        }
                    }
                }
            };
        };
    };
    ($field: ident in $sdb: ident, with_items $n: expr, $db: expr, $cr: expr) => {
        if let Some(items) = $n {
            if let Some(game) = $db.games.get_mut(&$cr.uuid) {
                for item in &items {
                    match &mut game.$field {
                        Some(genres) => genres.push(item.to_string()),
                        None => {
                            game.$field = Some(vec![item.to_string()]);
                        }
                    }
                }
            };
            for item in &items {
                $db.$sdb
                    .entry(item.to_string())
                    .and_modify(|e| e.games.push($cr.uuid))
                    .or_insert(Item {
                        name: item.to_string(),
                        games: vec![$cr.uuid],
                    });
            }
        };
    };
    ($field: ident, is_date $n: expr, $db: expr, $cr: expr) => {
        if let Some(date) = $n {
            if let Some(game) = $db.games.get_mut(&$cr.uuid) {
                game.$field =
                    Some(NaiveDate::parse_from_str(date, "%F").expect("fail to convert to date"));
            };
        }
    };
    (unknown $left: expr, $right: expr) => {
        if let Some(left) = $left {
            if let Some(right) = $right {
                eprintln!("Skipping unknown field: {}: {}", left, right);
            } else {
                eprintln!("Skipping unknown field: {}", left);
            };
        } else {
            eprintln!("Skipping unknown field");
        }
    };
}
