
use crate::prelude::*;
use super::MapArchitect;

// use bracket_lib::;
// use rltk::*;
// use rltk::*;

use rand::Rng;


pub struct PrefabArchitect {}

// give to it the functionality from DrunkardWalk
// impl DrunkardWalk for PrefabArchitect {}

impl MapArchitect for PrefabArchitect 
{
    fn new(&mut self) -> MapBuilder 
    {
        let mut mb = MapBuilder{
            map : Map::new(),
            player_start : Position::new(0, 0, 0),
            amulet_start : Position::new(0, 0, 0),
        };

        mb.fill(TileType::Wall);
        mb.player_start = Position{x:2, y:2, z: 0};
        self.apply_prefab(&mut mb);
        mb
    }
}

impl PrefabArchitect 
{
    fn apply_prefab(&self, mb: &mut MapBuilder) 
    {


        let tile_array = [TileType::Floor, TileType::Wall];
        let mut rng = rand::thread_rng();
    
        for tx in 0 .. SCREEN_WIDTH  {
            for ty in 0 .. SCREEN_HEIGHT {
                let tile_idx = map_idx(tx, ty);
                let type_idx  = rng.gen_range(0..tile_array.len());
                // println!("{}", idx);
                mb.map.tiles[tile_idx] = tile_array[type_idx];
            }
        }
    }
}

