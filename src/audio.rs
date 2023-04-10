use crate::prelude::*;


use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system(
                control_scene_sound
                    .in_set(OnUpdate(MyState::Playing))
            )
            ;
    }
}

#[derive(Resource)]
struct FlyingAudio(Handle<AudioInstance>);

#[derive(Resource)]
struct ItemGotAudio(Handle<AudioInstance>);

#[derive(Resource)]
struct SceneResultAudio(Handle<AudioInstance>);


fn control_scene_sound(
    audio: Res<Audio>,
    audio_assets: Res<AudioAssets>,
    mut commands: Commands, 
    mut audio_instances: ResMut<Assets<AudioInstance>>,
    our_clock: ResMut<OurClock>,
    mut actions: ResMut<Actions>,
)
{
    if actions.scene_changed != None {
        

        match actions.scene_changed
                {
            Some(MyTimeState::Result)  => {
                
                audio.play(audio_assets.scene_result.clone())
                .with_volume(0.3);
            },
            Some(MyTimeState::Playing)  => {
                audio.play(audio_assets.go.clone())
                .with_volume(0.3);
            },
            Some(MyTimeState::Ready) => {
                audio.play(audio_assets.ready.clone())
                .with_volume(0.3);
            }
            None => {},
            _ => {}
        }

    }
    actions.scene_changed = None;
}
