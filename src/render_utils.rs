use crate::prelude::*;

pub fn size_scaling(mut q: Query<(&TileSize, &mut Transform)>) {

        for (sprite_size, mut transform) in q.iter_mut() {
            let scale = Vec3::new(
                sprite_size.width / SCREEN_WIDTH as f32 ,
                sprite_size.height / SCREEN_HEIGHT as f32 ,
                1.0,
            );
            transform.scale = scale;
        }
}

pub fn position_translation(mut q: Query<(&Position, &mut Transform)>) {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                pos.x as f32 * 2. - 100f32,
                pos.y as f32* 2. - 100f32,
                pos.z as f32,
            );
        }
}