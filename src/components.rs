// use bracket_lib::terminal::*;
use rand::Rng;

use crate::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;
#[derive(Resource)]
pub struct PlayerStatus{
    pub is_ghost : bool,
}

#[derive(Component)]
pub struct Item;


#[derive(Component)]
pub struct MapTile;


#[derive(Component)]
pub struct TileSize {
    pub width: f32,
    pub height: f32,
}
impl TileSize {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination: Position
}


#[derive(Debug, Resource)]
pub struct MyStatus {
    pub score : u32,
    pub time : u32,
}


#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct MyGameCamera;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource::<MyStatus>( MyStatus{ score : 0, time : 0})
            .add_system(spawn_player.in_schedule(OnExit(MyState::Loading)))
            .add_system(move_player.in_set(OnUpdate(MyState::Playing)))
            .add_system(spawn_item.in_schedule(OnExit(MyState::Next)))
            .add_system(spawn_item.in_schedule(OnExit(MyState::Loading)))
            .add_system(get_item.in_set(OnUpdate(MyState::Playing)))
            .add_system(view_update_player.in_set(OnUpdate(MyState::Playing)))
            ;
    }
}


fn view_update_player
(
    player_status : ResMut<PlayerStatus>,
    mut player_query: Query<(&mut TextureAtlasSprite, &Player)>
)   
{   
    for(mut sprite, _) in player_query.iter_mut(){
        if player_status.is_ghost  == true{
            sprite.color.set_a(0.3);
        }else{
            sprite.color.set_a(1.0);
        }
    }

}

const TILESIZE: i32 = 16;
const POS_SPRITE: Point = Point{x:432/TILESIZE as i32, y:288/TILESIZE as i32};
fn spawn_player(
    mut commands: Commands, 
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mb: ResMut<MapBuilder>,
) {
    let texture_handle: Handle<Image> = textures.tilemap.clone();
    let texture_atlas = 
        TextureAtlas::from_grid(
            texture_handle, 
            Vec2::new(
                TILESIZE as f32,
                TILESIZE as f32
            ),
            POS_SPRITE.x as usize,
            POS_SPRITE.y as usize,
            None, None
        );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    let player_start = mb.player_start;
    let mut rng = rand::thread_rng();
    let noise = rng.gen_range(0..SCREEN_HEIGHT * SCREEN_WIDTH);

    commands.spawn((

        // },
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(3*POS_SPRITE.x as usize + 24),
            transform: Transform::from_translation(Vec3::new(100., 0., 1.)),
            ..default()
        },
        TileSize::square(1.0),
        Position { x: noise / SCREEN_WIDTH, y: noise % SCREEN_WIDTH, z: 2 },
        ),
        )
        .insert(Player);

}


fn spawn_item(
    mut commands: Commands, 
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
)
{
    let texture_handle: Handle<Image> = textures.tilemap.clone();
    let texture_atlas = 
        TextureAtlas::from_grid(
            texture_handle, 
            Vec2::new(
                TILESIZE as f32,
                TILESIZE as f32
            ),
            POS_SPRITE.x as usize,
            POS_SPRITE.y as usize,
            None, None
        );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    
    let mut rng = rand::thread_rng();
    for _ in 0..ITEM_NUM as i32 {
        let noise = rng.gen_range(0..SCREEN_HEIGHT * SCREEN_WIDTH);
        commands.spawn((
            SpriteSheetBundle 
            {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(9*POS_SPRITE.x as usize +9 ),
                transform: Transform::from_translation(Vec3::new(100., 0., 1.)),
                ..default()
            },
            TileSize::square(1.0),
            Position { x: noise / SCREEN_WIDTH, y : noise % SCREEN_WIDTH, z: 2 },
            ),
            )
            .insert(Item);
    
        
    }

}


fn get_item(
    mut commands: Commands, 
    mut mystatus: ResMut<MyStatus>,
    query : Query<(Entity, &Position), With<Item>>,
    player_position: Query<(Entity, &Position), With<Player>>,
){

    let (_, player_pos) = player_position.single();
    for(ent, position) in query.iter(){

        if position == player_pos {
            commands.entity(ent).despawn();
            mystatus.score += 1;
        }
    }

}


fn move_player(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut use_status : ResMut<PlayerStatus>,
    mut player_query : Query<(Entity, &Position, &mut Player)>
) {
    let (player_ent, pos, mut player) = player_query.single();
    let key = keyboard_input.get_pressed().next().cloned();

    let mut new_position = pos.clone();

    if let Some(key) = key {
        match key {
            KeyCode::Left => new_position.x -= 1,
            KeyCode::Right => new_position.x += 1,
            KeyCode::Down => new_position.y -= 1,
            KeyCode::Up => new_position.y += 1,
            KeyCode::G => { use_status.is_ghost = !use_status.is_ghost; },
            _ => {}
        }

        if new_position != *pos {
            commands.spawn(WantsToMove{entity: player_ent, destination: new_position});
        }

    }
}
