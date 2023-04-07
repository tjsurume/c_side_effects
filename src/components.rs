use crate::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

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



/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `MyState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(MyState::Playing)))
            .add_system(move_player.in_set(OnUpdate(MyState::Playing)));
    }
}


const TILESIZE: i32 = 16;
const POS_SPRITE: rltk::Point = rltk::Point{x:432/TILESIZE as i32, y:288/TILESIZE as i32};
fn spawn_player(mut commands: Commands, 
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut mb: ResMut<MapBuilder>,
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


fn move_player(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    player_position: Query<(Entity, &Position), With<Player>>,
) {


    let (player_ent, mut pos) = player_position.single();
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
            // pos = &new_position;
            // println!("{:?}", new_position);
        }

    }
}
