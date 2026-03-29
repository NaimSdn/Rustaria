mod perlin;
mod tile;
mod asset;

use bevy::prelude::{App, Plugin, Startup};
use crate::plugins::world::tile::WorldMap;
use asset::load_image;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldMap>().add_systems(Startup, load_image);
    }
}