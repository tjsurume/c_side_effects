// use bracket_lib::terminal::*;
use rand::Rng;

use crate::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

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

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource::<MyStatus>( MyStatus{ score : 0, time : 0})
            .add_system(spawn_player.in_schedule(OnEnter(MyState::Playing)))
            .add_system(move_player.in_set(OnUpdate(MyState::Playing)))
            .add_system(spawn_item.in_schedule(OnEnter(MyState::Playing)))
            .add_system(get_item.in_set(OnUpdate(MyState::Playing)))
            .add_system(setup_status.in_schedule(OnEnter(MyState::Playing)))
            .add_system(update_view.in_set(OnUpdate(MyState::Playing)))
            ;
    }
}


const TILESIZE: i32 = 16;
const POS_SPRITE: Point = Point{x:432/TILESIZE as i32, y:288/TILESIZE as i32};
fn spawn_player(mut commands: Commands, 
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
    commands.spawn((

        // },
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(24),
            transform: Transform::from_translation(Vec3::new(100., 0., 1.)),
            ..default()
        },
        TileSize::square(1.0),
        Position { x: player_start.x, y: player_start.y, z: 2 },
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
    for _ in 0..10 {
        let noise = rng.gen_range(0..100);
        commands.spawn((
            SpriteSheetBundle 
            {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(9*POS_SPRITE.x as usize +9 ),
                transform: Transform::from_translation(Vec3::new(100., 0., 1.)),
                ..default()
            },
            TileSize::square(1.0),
            Position { x: noise / 10, y : noise % 10, z: 2 },
            ),
            )
            .insert(Item);
    
        
    }

}

fn setup_status(
    mut commands: Commands, 
    font_assets: Res<FontAssets>,
)
{
    let text_style = bevy::prelude::TextStyle {
        font: font_assets.fira_sans.clone(),
        font_size: 36.0,
        color: Color::WHITE,
    };


    commands
    .spawn(
        Text2dBundle {
            transform: Transform {
                translation: Vec3::from((0f32, 50f32, 2f32)),
                rotation: Quat::from_rotation_z(0.0f32),
                scale: Vec3::new(1f32, 1f32, 1f32),
            },
            text:  Text::from_section("Now Loading", text_style),
            ..default()
        }
    )
    ;

}

fn update_view(
    mut query: Query<(Entity,  &mut Text)>,
    my_status: ResMut<MyStatus>
)
{
    for (_, mut text) in query.iter_mut()
    {
        text.sections[0].value = my_status.score.to_string();
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
    keyboard_input: Res<bevy::prelude::Input<KeyCode>>,
    player_position: Query<(Entity, &Position), With<Player>>,
) {
    let (player_ent, pos) = player_position.single();
    let key = keyboard_input.get_pressed().next().cloned();

    let mut new_position = pos.clone();

    if let Some(key) = key {

        match key {
            KeyCode::Left => new_position.x -= 1,
            KeyCode::Right => new_position.x += 1,
            KeyCode::Down => new_position.y -= 1,
            KeyCode::Up => new_position.y += 1,
            _ => {}
        }

        if new_position != *pos {
            commands.spawn(WantsToMove{entity: player_ent, destination: new_position});
        }

    }
}
