use crate::prelude::*;


pub struct StatusViewPlugin;

#[derive(Component)]
pub struct TimerStatusView;

#[derive(Component)]
pub struct ScoreStatusView;

#[derive(Component)]
pub struct MyTimeStateView;

impl Plugin for StatusViewPlugin{
    fn build(&self, app: &mut App)
    {
        app
            .add_system(show_status_view.in_schedule(OnExit(MyState::Loading)))
            .add_system(update_timer_status_view.in_set(OnUpdate(MyState::Playing)))
            .add_system(update_score_status_view.in_set(OnUpdate(MyState::Playing)))
            .add_system(update_time_state_view.in_set(OnUpdate(MyState::Playing)))
            ;
    }
}

fn show_status_view(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
)
{
    let text_style = bevy::prelude::TextStyle {
        font: font_assets.fira_sans.clone(),
        font_size: 36.0,
        color: Color::WHITE,
    };

    let score_text = Text2dBundle {
        text: Text::from_section(
            "hello bevy!",
            text_style.clone(),
        ),
        transform: Transform {
            translation: Vec3::from((-100f32, 150f32, 2f32)),
            rotation: Quat::from_rotation_z(0.0f32),
            scale: Vec3::new(1f32, 1f32, 1f32),
        },
        ..default()
    };
    commands.spawn((score_text, TimerStatusView));


    let score_text = Text2dBundle {
        text: Text::from_section(
            "hello bevy!",
            text_style.clone(),
        ),
        transform: Transform {
            translation: Vec3::from((-100f32, 100f32, 2f32)),
            rotation: Quat::from_rotation_z(0.0f32),
            scale: Vec3::new(1f32, 1f32, 1f32),
        },
        ..default()
    };
    commands.spawn((score_text, ScoreStatusView));


    let score_text = Text2dBundle {
        text: Text::from_section(
            "hello bevy!",
            text_style.clone(),
        ),
        transform: Transform {
            translation: Vec3::from((-100f32, 130f32, 2f32)),
            rotation: Quat::from_rotation_z(0.0f32),
            scale: Vec3::new(1f32, 1f32, 1f32),
        },
        ..default()
    };
    commands.spawn((score_text, MyTimeStateView));



}


fn update_timer_status_view(
    mut commands: Commands, 
    mut mystatus: ResMut<MyStatus>,
    mut query: Query<(Entity, &mut Text, &mut Transform), With<TimerStatusView>>,
    mut camera_query: Query<&mut Transform, (With<MyGameCamera>, Without<TimerStatusView>)>,

)
{
    let mut camera_transform = camera_query.single_mut();
    
    for(_, mut text, mut transform) in query.iter_mut(){
        text.sections[0].value = mystatus.score.to_string();
    
        transform.translation.x = camera_transform.translation.x - 100.;
        transform.translation.y = camera_transform.translation.y - 100.;


    }   
}

fn update_score_status_view(
    mut commands: Commands, 
    our_clock: ResMut<OurClock>,
    mut query: Query<(Entity, &mut Text, &mut Transform), With<ScoreStatusView>>,
    mut camera_query: Query<&mut Transform, (With<MyGameCamera>, Without<ScoreStatusView>)>,

)
{
    let mut camera_transform = camera_query.single_mut();
    
    for(_, mut text, mut transform) in query.iter_mut(){
        text.sections[0].value = our_clock.stop_watch.elapsed_secs().floor().to_string();
    
        transform.translation.x = camera_transform.translation.x - 100.;
        transform.translation.y = camera_transform.translation.y - 100.;

    
    }   
}


fn update_time_state_view(
    mut commands: Commands, 
    our_clock: ResMut<OurClock>,
    mut query: Query<(Entity, &mut Text, &mut Transform), With<MyTimeStateView>>,
    mut camera_query: Query<&mut Transform, (With<MyGameCamera>, Without<MyTimeStateView>)>,

)
{
    let mut camera_transform = camera_query.single_mut();
        
    for(_, mut text, mut transform) in query.iter_mut(){
        let state_label =  match our_clock.state {
            MyTimeState::Stop => "Stop",
            MyTimeState::Ready => "Ready",
            MyTimeState::Playing => "Playing",
            MyTimeState::Result => "Result",
        };
        text.sections[0].value  = state_label.to_string();
        transform.translation.x = camera_transform.translation.x - 100.;
        transform.translation.y = camera_transform.translation.y - 120.;

    }   
}
