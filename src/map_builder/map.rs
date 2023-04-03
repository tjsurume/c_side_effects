use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
    Exit,
    Void,
}


pub struct Map {
    pub tiles: Vec<TileType>,
    pub occupation: Vec<Option<Entity>>
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Void; NUM_TILES],
            occupation : vec![None; NUM_TILES],
        }
    }
}