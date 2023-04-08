use crate::prelude::*;

mod movements;

struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(movements::movement.in_set(OnUpdate(MyState::Playing)))
            // .add_system(movements::camera_movement.in_set(OnUpdate(MyState::Playing)))
            
            ;
    }
}

pub struct SystemsPlugin;
impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App)
    {
        app
            .add_plugin(PlayerPlugin)
            ;
    }
}