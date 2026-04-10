mod plugins;

use crate::plugins::camera::CameraPlugin;
use crate::plugins::player::PlayerPlugin;
use crate::plugins::world::WorldPlugin;
use bevy::DefaultPlugins;
use bevy::prelude::App;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}
