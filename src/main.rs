mod plugins;

use bevy::DefaultPlugins;
use bevy::prelude::{App};
use plugins::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldPlugin)
        .run();
}
