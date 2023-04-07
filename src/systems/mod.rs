use crate::prelude::*;

mod movements;

struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(movements::movement)
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