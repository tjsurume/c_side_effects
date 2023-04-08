use crate::prelude::*;

pub fn position_translation(mut q: Query<(&Position, &mut Transform)>) {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                pos.x as f32 * 16. - 100f32,
                pos.y as f32* 16. - 100f32,
                pos.z as f32,
            );
        }
}