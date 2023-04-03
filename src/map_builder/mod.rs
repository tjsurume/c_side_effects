use crate::prelude::*;

mod map;
pub use map::*;


trait MapArchitect {
    fn new(&mut self) -> MapBuilder;

}

#[derive(Resource)]
pub struct MapBuilder {
    pub map: Map,
    walls: Vec<Rect>,
    rooms: Vec<Rect>,
    pub player_Start: Position
}

impl MapBuilder {
    pub fn new() -> Self 
    {
        Box::new(SampleArchitect{}).new()
    }

}


struct SampleArchitect {}


impl MapArchitect for SampleArchitect {

    fn new(&mut self) -> MapBuilder 
    {
        let mut mb = MapBuilder {
            map : Map::new(),
            rooms: Vec::new(),
            walls: Vec::new(),
            player_Start: Position::new(0, 0, 0),
        };
        mb
    }

}



pub fn build_map(
    mut commands: Commands,
    player_q: Query<&Player>
)
{
    let mut mb = MapBuilder::new();
    commands.insert_resource(mb);
}

pub struct MapPlugin;
impl Plugin for MapPlugin
{

    fn build(&self, app: &mut App){

    }
}
