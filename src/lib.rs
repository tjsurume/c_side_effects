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
mod systems;
mod gametime;
mod status_view;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::components::PlayerPlugin;
use crate::gametime::GameTimePlugin;
use crate::status_view::StatusViewPlugin;

use bevy::app::App;

mod prelude {
    pub use bevy::prelude::*;
    
    pub const SCREEN_WIDTH: i32 = 30;
    pub const SCREEN_HEIGHT: i32 = 30;
    pub const ITEM_NUM : i32 = 30;
    pub const WINDOW_WIDTH: f32 = 800.;
    pub const WINDOW_HEIGHT: f32 = 600.;
    
    pub const STATUS_START_X:f32 = (WINDOW_WIDTH / 2.) - 150.;
    pub const STATUS_START_Y:f32 = (WINDOW_HEIGHT / 2.) - 150.;


    pub const PLAYING_TIME : f32 = 20.;

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
    pub use crate::systems::*;
    pub use crate::gametime::*;
    pub use crate::status_view::*;

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
            .add_plugin(SystemsPlugin)
            .add_plugin(GameTimePlugin)
            .add_plugin(StatusViewPlugin)
            .add_system(
                position_translation.in_set(OnUpdate(MyState::Playing))
            )
            
            ;
    }
}
