mod libs;

mod loading;
mod playing;

use bevy::prelude::*;

use self::{loading::LoadingPlugin, playing::PlayingPlugin};

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Start,
    // Playing,
    // Menu,
}

#[derive(Default)]
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(PlayingPlugin);
    }
}
