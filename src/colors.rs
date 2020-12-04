/// Copyright (c) 2020, Shoyo Inokuchi
use crate::tiles::TileColor;
use std::collections::HashMap;

pub struct Colors;

impl Colors {
    pub fn all() -> HashMap<TileColor, bool> {
        let mut map = HashMap::new();
        map.insert(TileColor::Black, true);
        map.insert(TileColor::Red, true);
        map.insert(TileColor::Blue, true);
        map.insert(TileColor::Orange, true);
        map
    }

    pub fn none() -> HashMap<TileColor, bool> {
        let mut map = HashMap::new();
        map.insert(TileColor::Black, false);
        map.insert(TileColor::Red, false);
        map.insert(TileColor::Blue, false);
        map.insert(TileColor::Orange, false);
        map
    }

    pub fn only(color: TileColor) -> HashMap<TileColor, bool> {
        let mut map = HashMap::new();
        map.insert(TileColor::Black, false);
        map.insert(TileColor::Red, false);
        map.insert(TileColor::Blue, false);
        map.insert(TileColor::Orange, false);
        map.insert(color, true);
        map
    }

    pub fn except(color: TileColor) -> HashMap<TileColor, bool> {
        let mut map = HashMap::new();
        map.insert(TileColor::Black, true);
        map.insert(TileColor::Red, true);
        map.insert(TileColor::Blue, true);
        map.insert(TileColor::Orange, true);
        map.insert(color, false);
        map
    }
}
