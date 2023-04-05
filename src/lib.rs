mod actions;
mod audio;
mod loading;
mod menu;
mod components;
mod map_builder;
mod map;
mod utils;
mod resources;
mod render_utils;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::components::PlayerPlugin;

use bevy::app::App;

mod prelude {
    pub use bevy::prelude::*;
    pub const SCREEN_WIDTH: i32 = 30;
    pub const SCREEN_HEIGHT: i32 = 30;

    pub use bevy::winit::WinitSettings;
    pub use crate::map_builder::*;
    pub use crate::utils::*;
    pub use crate::audio::*;
    pub use crate::components::*;
    pub use crate::resources::*;
    pub use crate::actions::*;
    pub use crate::loading::*;
    pub use crate::map::*;
    pub use crate::render_utils::*;

}

use prelude::*;



pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MyState>()
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(MapPlugin)
            .add_system(
                position_translation
            )
            
            ;
    }
}
