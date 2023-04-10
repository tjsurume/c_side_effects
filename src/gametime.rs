use std::time::{Instant, Duration};
use bevy::{input::keyboard::KeyboardInput, text::{BreakLineOn, Text2dBounds}};
use crate::{prelude::*, map_builder};

use bevy::time::Stopwatch;

#[derive(Component)]
struct GameTime(Instant);

pub struct GameTimePlugin;


impl Plugin for GameTimePlugin{
    fn build(&self, app: &mut App)
    {
        app
            .insert_resource::<OurClock>(OurClock{ stock_seconds : 0., state: MyTimeState::Stop, stop_watch: Stopwatch::new()})
            .insert_resource::<PlayerStatus>(PlayerStatus{ is_ghost : false, direction: GameControl::Down})
            .add_system(print_game_time)
            .add_startup_system(setup_game_time)
            .add_system(update_delta_time.in_set(OnUpdate(MyTimeState::Playing)))
            .add_system(popup_ready.in_schedule(OnEnter(MyState::Playing)))
            .add_system(state_timer_management)
            .add_system(update_key_game)
            .add_system(next_to_replay.in_schedule(OnEnter(MyState::Next)))
            ;
    }   
}


fn popup_ready(
    mut our_clock: ResMut<OurClock>,
        mut actions: ResMut<Actions>,
)
{
    println!("Ready");
    our_clock.stop_watch.reset();
    our_clock.state = MyTimeState::Ready;
    actions.scene_changed = Some(MyTimeState::Ready);
}



fn state_timer_management(
    mut our_clock: ResMut<OurClock>,
    mut actions: ResMut<Actions>, 
)
{

    match our_clock.state {
        MyTimeState::Ready => {
            if our_clock.stop_watch.elapsed_secs() >= 5. {
                our_clock.state = MyTimeState::Playing;
                our_clock.stop_watch.reset();
                actions.scene_changed =Some(MyTimeState::Playing);
            }          
        },
        MyTimeState::Playing => {
            if our_clock.stop_watch.elapsed_secs() >= PLAYING_TIME {
                our_clock.state = MyTimeState::Result;   
                our_clock.stop_watch.reset();
                our_clock.stop_watch.pause();
                actions.scene_changed =Some(MyTimeState::Result);
            }

        },
        _ => ()
    }
}






fn update_delta_time(
    time: Res<Time>,
    mut our_clock: ResMut<OurClock>,
    mut player_query: Query<(Entity, &mut Player)>,
    mut player_status : ResMut<PlayerStatus>
)
{
    let mut speed_up = false;

    speed_up  = player_status.is_ghost;

    let mut delta_time = time.delta();
    if speed_up {
        delta_time *= 2;
    }
    our_clock.stop_watch.tick(delta_time);       
    
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
    // commands.spawn(())
    //     .insert(
    //         GameTime(time.startup() + time.elapsed())
    //     )
        
        // ;
}

fn update_key_game(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mut our_clock: ResMut<OurClock>,
    mut mb: ResMut<MapBuilder>,
    mut player_status : ResMut<PlayerStatus>,
    mut mystatus: ResMut<MyStatus>,
    mut state: ResMut<NextState<MyState>>,
    mut actions: ResMut<Actions>,
) {
    let key = keyboard_input.get_just_pressed().next().cloned();

    if let Some(key) = key {
        match key {
            KeyCode::R => {
                if our_clock.state == MyTimeState::Result {
                    our_clock.state = MyTimeState::Ready;   
                    actions.scene_changed = Some(MyTimeState::Ready);
                    our_clock.stop_watch.reset();
                    mystatus.score = 0;
                    player_status.is_ghost = false;
                    our_clock.stop_watch.unpause();
                    state.set(MyState::Next);
                }
            }
            _ => {}
        }
    }
}


fn next_to_replay
(
    mut state: ResMut<NextState<MyState>>,
)
{
    state.set(MyState::Playing);
}