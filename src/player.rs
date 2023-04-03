use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(GameState::Playing)))
            .add_system(move_player.in_set(OnUpdate(GameState::Playing)));
    }
}

struct Point {
    x: usize,
    y: usize,
}
const TILESIZE: usize = 16;
const POS_SPRITE: Point = Point{x:432/TILESIZE, y:288/TILESIZE};
fn spawn_player(mut commands: Commands, 
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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
    
    commands.spawn((
        SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(24),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..default()
        },
        ))
        .insert(Player);

}


fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let speed = 150.;
    let movement = Vec3::new(
        actions.player_movement.unwrap().x * speed * time.delta_seconds(),
        actions.player_movement.unwrap().y * speed * time.delta_seconds(),
        0.,
    );
    for mut player_transform in &mut player_query {
        player_transform.translation += movement;
    }
}
