use crate::prelude::*;


pub struct StatusViewPlugin;

#[derive(Component)]
pub struct TimerStatusView;

#[derive(Component)]
pub struct ScoreStatusView;

#[derive(Component)]
pub struct MyTimeStateView;


#[derive(Component)]
pub struct MyGuide;

impl Plugin for StatusViewPlugin{
    fn build(&self, app: &mut App)
    {
        app
            .add_system(show_status_view.in_schedule(OnExit(MyState::Loading)))
            .add_system(update_timer_status_view.in_set(OnUpdate(MyState::Playing)))
            .add_system(update_score_status_view.in_set(OnUpdate(MyState::Playing)))
            .add_system(update_time_state_view.in_set(OnUpdate(MyState::Playing)))
            .add_system(spawn_cover_pic.in_schedule(OnEnter(MyState::Menu)))
            .add_system(despawn_cover_pic.in_schedule(OnExit(MyState::Menu)))
            .add_system(update_guide.in_set(OnUpdate(MyState::Playing)))
            .add_system(update_guide_next.in_set(OnUpdate(MyState::Next)))
            
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
        font_size: 16.0,
        color: Color::WHITE,
    };

    let score_text = Text2dBundle {
        text: Text::from_section(
            "",
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
            "",
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
            "",
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


    let my_guide = Text2dBundle {
        text: Text::from_section(
            "",
            text_style.clone(),
        ),
        transform: Transform {
            translation: Vec3::from((-100f32, 130f32, 2f32)),
            rotation: Quat::from_rotation_z(0.0f32),
            scale: Vec3::new(1f32, 1f32, 1f32),
        },
        ..default()
    };
    commands.spawn((my_guide, MyGuide));

}


fn setup_status_view(
    mut commands: Commands,
)
{

}
    

fn update_score_status_view(
    mut commands: Commands, 
    mut mystatus: ResMut<MyStatus>,
    our_clock: ResMut<OurClock>,
    mut query: Query<(Entity, &mut Text, &mut Transform), With<TimerStatusView>>,
    mut camera_query: Query<&mut Transform, (With<MyGameCamera>, Without<TimerStatusView>)>,

)
{
    let mut camera_transform = camera_query.single_mut();
    
    for(_, mut text, mut transform) in query.iter_mut(){
        
        if our_clock.state != MyTimeState::Ready { 
            text.sections[0].value = "Score : ".to_string() +  &mystatus.score.to_string();
            
        }else{
            text.sections[0].value = "".to_string();
            
        }
        transform.translation.x = camera_transform.translation.x - STATUS_START_X + 100.;
        transform.translation.y = camera_transform.translation.y - STATUS_START_Y;


    }   
}
fn update_timer_status_view(
    mut commands: Commands, 
    our_clock: ResMut<OurClock>,
    mut query: Query<(Entity, &mut Text, &mut Transform), With<ScoreStatusView>>,
    mut camera_query: Query<&mut Transform, (With<MyGameCamera>, Without<ScoreStatusView>)>,

)
{
    let mut camera_transform = camera_query.single_mut();
    
    for(_, mut text, mut transform) in query.iter_mut(){
        let mut mytime = our_clock.stop_watch.elapsed_secs();
    
        let mut view_time = 0.;
        let mut enable_view = true;
        match our_clock.state {
            MyTimeState::Playing => view_time = PLAYING_TIME - mytime,
            MyTimeState::Ready => view_time = 5. - mytime,
            _ => enable_view = false,
        }
        if enable_view == true {
            text.sections[0].value = "Time : ".to_string()  + &format!("{view_time:.2}");//&view_time.to_string();
            
        }else {
            text.sections[0].value = "".to_string();
            
        }

        transform.translation.x = camera_transform.translation.x - STATUS_START_X + 200.;
        transform.translation.y = camera_transform.translation.y - STATUS_START_Y;

    
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
            MyTimeState::Playing => "Go!!",
            MyTimeState::Result => "Result",
        };
        text.sections[0].value  = state_label.to_string();
        transform.translation.x = camera_transform.translation.x - STATUS_START_X;
        transform.translation.y = camera_transform.translation.y - STATUS_START_Y;

    }   
}

fn update_guide(
    our_clock: ResMut<OurClock>,
    mut query: Query<(Entity, &mut Text, &mut Transform), With<MyGuide>>,
    mut camera_query: Query<&mut Transform, (With<MyGameCamera>, Without<MyGuide>)>,
)
{
    let mut camera_transform = camera_query.single_mut();
    for(_, mut text, mut transform) in query.iter_mut(){
        let state_label =  match our_clock.state {
            MyTimeState::Stop => "",
            MyTimeState::Ready => "",
            MyTimeState::Playing => "Press <G> Toggle Ghost, <Cursor> Move",
            MyTimeState::Result => "Press <R> Restart",
        };
        text.sections[0].value  = state_label.to_string();
        transform.translation.x = camera_transform.translation.x - STATUS_START_X + 350.;
        transform.translation.y = camera_transform.translation.y - STATUS_START_Y;
    }
}

fn update_guide_next(
    our_clock: ResMut<OurClock>,
    mut query: Query<(Entity, &mut Text, &mut Transform), With<MyGuide>>,
    mut camera_query: Query<&mut Transform, (With<MyGameCamera>, Without<MyGuide>)>,
)
{
    let mut camera_transform = camera_query.single_mut();
    for(_, mut text, mut transform) in query.iter_mut(){
        let state_label = "Press <X> to restart.".to_string();
        text.sections[0].value  = state_label.to_string();
        transform.translation.x = camera_transform.translation.x - STATUS_START_X + 350.;
        transform.translation.y = camera_transform.translation.y - STATUS_START_Y;
    }
}


#[derive(Component)]
pub struct CoverPic;

fn spawn_cover_pic(
    mut commands: Commands, 
    textures: Res<TextureAssets>
    
)
{

    commands
        .spawn((SpriteBundle {
            texture: textures.cover_pic.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        }, CoverPic
        ),
        )
        ;
}

fn despawn_cover_pic(
    mut commands: Commands, 
    textures: Res<TextureAssets>,
    query: Query<Entity, With<CoverPic>>
)
{
    for ent in query.iter(){
        commands.entity(ent).despawn();
    }
}