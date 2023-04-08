
use crate::prelude::*;
use super::MapArchitect;

// use bracket_lib::;
// use rltk::*;
// use rltk::*;

const FORTRESS : (&str, i32, i32) = ("
------------
---######---
---#----#---
---#-M--#---
-###----###-
--M------M--
-###----###-
---#----#---
---#----#---
---######---
------------
", 12, 11);

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


        let string_vec : Vec<char> = FORTRESS.0
            .chars().filter(|a| *a != '\r' && *a !='\n')
            .collect();
        let mut i = 0;
        for ty in 0 .. FORTRESS.2 {
            for tx in  0 .. FORTRESS.1 {
                let idx = map_idx(tx, ty);
                let c = string_vec[i];
                match c {
                    'M' => {
                        mb.map.tiles[idx] = TileType::Floor;
                    }
                    '-' => mb.map.tiles[idx] = TileType::Floor,
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    _ => println!("No idea what to do with [{}]", c)
                }
                i += 1;
            }
        }
    }
}

