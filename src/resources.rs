use crate::prelude::*;

use bevy::time::Stopwatch;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum MyState {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    Next,
}


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


pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
}
