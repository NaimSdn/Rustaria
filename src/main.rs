mod plugins;
mod constants;

use crate::plugins::camera::CameraPlugin;
use crate::plugins::input::InputPlugin;
use crate::plugins::player::PlayerPlugin;
use crate::plugins::world::WorldPlugin;
use bevy::DefaultPlugins;
use bevy::prelude::App;
use crate::plugins::physics::PhysicsPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(PhysicsPlugin)
        .run();
}
