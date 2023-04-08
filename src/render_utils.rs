use crate::prelude::*;

pub fn position_translation(
    mut q: Query<(&Position, &mut Transform)>
    
) {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                pos.x as f32 * 16. - 100f32,
                pos.y as f32* 16. - 100f32,
                pos.z as f32,
            );
        }
}

pub fn convert_pos(pos: f32, bound_game: f32) -> f32 {
    let tile_size = 16.;
    let bound_window = tile_size * bound_game;
    pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)  + 100f32
}
