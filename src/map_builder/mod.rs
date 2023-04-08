use crate::prelude::*;

// use rltk::*;

mod prefab;
use bracket_lib::prelude::Algorithm2D;
use prefab::PrefabArchitect;

trait MapArchitect {
    fn new(&mut self) -> MapBuilder;

}

#[derive(Resource, Debug)]
pub struct MapBuilder {
    pub map: Map,
    pub amulet_start: Position,
    pub player_start: Position,
}

impl MapBuilder {
    pub fn new() -> Self 
    {
        Box::new(PrefabArchitect{}).new()
    }

    
    fn fill(&mut self, tile:TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

}



pub fn build_map(
    mut commands: Commands,
)
{
    let mut mb = MapBuilder::new();

    let farer_position = mb.amulet_start;
    let idx = mb.map.point2d_to_index(farer_position.into());
    mb.map.tiles[idx] = TileType::Exit;

    commands.insert_resource(mb);
}

pub struct MapPlugin;
impl Plugin for MapPlugin
{
    fn build(&self, app: &mut App){
        app
        .add_system(
            build_map.in_schedule(OnEnter(MyState::Loading))
        )
        .add_system(
            spawn_map_tiles.in_schedule(OnExit(MyState::Loading))
        )
        ;
    }
}
