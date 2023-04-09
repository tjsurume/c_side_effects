use crate::prelude::*;

mod prefab;
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

pub fn renew_map(
    mut commands: Commands,
    mb: ResMut<MapBuilder>,
    state: ResMut<NextState<MyState>>,
    mut query: Query<Entity, With<MapTile>>,
    mut item_query : Query<(Entity, With<Item>), Without<MapTile>>

)
{

    for (ent) in query.iter() {
        commands.entity(ent).despawn();
    }

    for (ent, _) in item_query.iter() {
       println!("despawn item!!");
        commands.entity(ent).despawn();
        
    }
    commands.remove_resource::<MapBuilder>();
    build_map(commands);

}

pub fn build_map(
    mut commands: Commands,
)
{
    let mut mb = MapBuilder::new();

    let farer_position = mb.amulet_start;
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
        .add_system(
            renew_map.in_schedule(OnEnter(MyState::Next))
        )
        .add_system(
            spawn_map_tiles.in_schedule(OnExit(MyState::Next))
        )
        ;
    }
}
