mod asset;
mod perlin;
mod render;
mod tile;

use crate::plugins::world::asset::load_image;
use crate::plugins::world::perlin::generate_world;
use crate::plugins::world::render::render_world;
use crate::plugins::world::tile::WorldMap;
use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Startup};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldMap>()
            .add_systems(Startup, (load_image, generate_world, render_world).chain());
    }
}
