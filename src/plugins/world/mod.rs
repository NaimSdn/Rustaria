mod perlin;
mod tile;
mod asset;
mod render;

use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Startup};
use crate::plugins::world::tile::WorldMap;
use asset::load_image;
use perlin::generate_world;
use render::render_world;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldMap>().add_systems(Startup, (load_image, generate_world, render_world).chain());
    }
}