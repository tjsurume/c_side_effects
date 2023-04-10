use crate::MyState;
use crate::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(MyState::Loading).continue_to_state(MyState::Menu),
        )
        .add_collection_to_loading_state::<_, FontAssets>(MyState::Loading)
        .add_collection_to_loading_state::<_, AudioAssets>(MyState::Loading)
        .add_collection_to_loading_state::<_, TextureAssets>(MyState::Loading);
    }
}

// the following asset collections will be loaded during the State `MyState::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,

    #[asset(path = "audio/click2.ogg")]
    pub item_got : Handle<AudioSource>,

    #[asset(path = "audio/jingles_STEEL02.ogg")]
    pub scene_result : Handle<AudioSource>,


    #[asset(path = "audio/ready.ogg")]
    pub ready : Handle<AudioSource>,

    #[asset(path = "audio/go.ogg")]
    pub go : Handle<AudioSource>,

    #[asset(path = "audio/time_over.ogg")]
    pub time_over : Handle<AudioSource>,


}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,

    #[asset(path = "textures/tilemap_packed.png")]
    pub tilemap: Handle<Image>,

    #[asset(path = "pictures/girl_in_the_forest.png")]
    pub cover_pic: Handle<Image>,

}
