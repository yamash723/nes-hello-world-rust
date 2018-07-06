use std::slice::Iter;
use super::tile::Tile;

pub struct Background {
    tiles: Vec<Tile>,
}

impl Background {
    pub fn new() -> Self {
        Background {
            tiles: Vec::new(),
        }
    }

    pub fn push(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }

    pub fn iter(&self) -> Iter<Tile> {
        self.tiles.iter()
    }
}