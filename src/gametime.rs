use std::time::{Instant, Duration};
use bevy::{input::keyboard::KeyboardInput, text::{BreakLineOn, Text2dBounds}};
use crate::prelude::*;
use bevy::time::Stopwatch;

#[derive(Component)]
struct GameTime(Instant);

pub struct GameTimePlugin;


#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum MyTimeState {
    #[default]
    Stop,
    Ready,
    Playing,
    Result,
}

#[derive(Default, Resource)]
pub struct OurClock {
    pub stock_seconds: f32,
    pub state : MyTimeState,
    pub stop_watch : Stopwatch
}

impl Plugin for GameTimePlugin{
    fn build(&self, app: &mut App)
    {
        app
            .insert_resource::<OurClock>(OurClock{ stock_seconds : 0., state: MyTimeState::Stop, stop_watch: Stopwatch::new()})
            .add_system(print_game_time)
            .add_startup_system(setup_game_time)
            .add_system(update_delta_time.in_set(OnUpdate(MyTimeState::Playing)))
            .add_system(popup_ready.in_schedule(OnEnter(MyState::Playing)))
            .add_system(state_timer_management)
            ;
    }   
}


fn popup_ready(
    mut our_clock: ResMut<OurClock>
)
{
    println!("Ready");
    our_clock.stop_watch.reset();
    our_clock.state = MyTimeState::Ready;
}



fn state_timer_management(
    mut our_clock: ResMut<OurClock>
)
{

    match our_clock.state {
        MyTimeState::Ready => {
            
    println!("{:?}, {:?}", our_clock.stop_watch, our_clock.state);
            if our_clock.stop_watch.elapsed_secs() >= 5. {
                our_clock.state = MyTimeState::Playing;
                our_clock.stop_watch.reset();
            }          
        },
        MyTimeState::Playing => {
            if our_clock.stop_watch.elapsed_secs() >= 10. {
                our_clock.state = MyTimeState::Stop;   
                our_clock.stop_watch.reset();
                our_clock.stop_watch.pause();
            }

        },
        _ => ()
    }
}






fn update_delta_time(
    time: Res<Time>,
    mut our_clock: ResMut<OurClock>
)
{
    our_clock.stop_watch.tick(time.delta());       
    
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