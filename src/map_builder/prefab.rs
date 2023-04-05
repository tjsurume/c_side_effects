use crate::prelude::*;
use super::MapArchitect;

use crate::prelude::*;
use rand::Rng;
use rltk::*;

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
            rooms: Vec::new(),
            walls: Vec::new(),
            player_start : Position::new(0, 0, 0),
            amulet_start : Position::new(0, 0, 0),
        };

        mb.fill(TileType::Wall);
        let center = Position::new_from2d(SCREEN_WIDTH /2, SCREEN_HEIGHT/2);

        mb.player_start = Position{x:2, y:2, z: 0};
        self.apply_prefab(&mut mb);
        mb
    }
}

impl PrefabArchitect 
{
    fn apply_prefab(&self, mb: &mut MapBuilder) 
    {
        let mut placement = None;
    
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![mb.map.point2d_to_index(mb.player_start.into())],
            &mb.map,
            1024.0
        );
    
        let mut attempts = 0;
        while placement.is_none() && attempts < 10 {
            let dimensions = rltk::Rect::with_size(
                0,
                0,
                FORTRESS.1,
                FORTRESS.2
            );
    
            let mut can_place = true;
            dimensions.for_each(|pt| {
                let idx = mb.map.point2d_to_index(pt);
                let distance = dijkstra_map.map[idx];
                if distance < 2000.0 && distance > 20.0 && mb.amulet_start != pt.into() {
                    can_place = true;
                }
            });
    
            if can_place {
                placement = Some(Point::new(dimensions.x1, dimensions.y1));
                let points = dimensions.point_set();
            }
            attempts += 1;
        }
        println!("Pressxee");
        println!("{:?}",  placement);
        if let Some(placement) = placement {
            println!("Replaceee");
            let string_vec : Vec<char> = FORTRESS.0
                .chars().filter(|a| *a != '\r' && *a !='\n')
                .collect();
            let mut i = 0;
            for ty in placement.y .. placement.y + FORTRESS.2 {
                for tx in placement.x .. placement.x + FORTRESS.1 {
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
}

