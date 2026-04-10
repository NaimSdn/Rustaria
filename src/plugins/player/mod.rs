use crate::plugins::player::player::init_player;
use bevy::app::{App, Plugin, Startup};

pub(crate) mod player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_player);
    }
}
