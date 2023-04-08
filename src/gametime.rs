use bevy::{prelude::*};
use std::time::Instant;
use bevy::{input::keyboard::KeyboardInput};

#[derive(Component)]
struct GameTime(Instant);

pub struct GameTimePlugin;


#[derive(Default, Resource)]
struct OurClock {
    stock_seconds: f32,
}

impl Plugin for GameTimePlugin{
    fn build(&self, app: &mut App)
    {
        app
            .insert_resource::<OurClock>(OurClock{ stock_seconds : 0.})
            .add_system(print_game_time)
            .add_startup_system(setup_game_time)
            .add_startup_system(setup_view)
            .add_system(update_delta_time)
            .add_system(update_view)
            ;
    }   
}

pub fn setup_view(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
)
{
    let font = asset_server.load("my_bevy_assets/fonts/JF-Dot-Shinonome16.ttf");
    let text_style = bevy::prelude::TextStyle {
        font: font.clone(),
        font_size: 36.0,
        color: Color::WHITE,
    };

    commands
    .spawn(
        Text2dBundle {
            transform: Transform {
                translation: Vec3::from((0f32, 50f32, 20f32)),
                rotation: Quat::from_rotation_z(0.0f32),
                scale: Vec3::new(1f32, 1f32, 1f32),
            },
            text:  Text::from_section("Now Loading", text_style),
            ..default()
        }
    )
    ;
}


fn update_view(
    mut query: Query<(Entity,  &mut Text)>,
    our_clock: ResMut<OurClock>
)
{
    for (_, mut text) in query.iter_mut()
    {
        text.sections[0].value = our_clock.stock_seconds.to_string();
    } 
}

fn update_delta_time(
    time: Res<Time>,
    mut our_clock: ResMut<OurClock>
)
{
    our_clock.stock_seconds +=  time.delta_seconds();
}



fn print_game_time(
    mut query: Query<(
        Entity,
        &mut GameTime,
    )>
)
{
    for (entity, mut game_timer) in query.iter_mut() {
        let mytime = game_timer.0.elapsed();
        // println!("{}", mytime.as_millis());
        // todo something.     
        
    }
}



fn setup_game_time(
    mut commands: Commands,
    time: Res<Time>

)
{
    commands.spawn(())
        .insert(
            GameTime(time.startup() + time.elapsed())
        )
        
        ;
}