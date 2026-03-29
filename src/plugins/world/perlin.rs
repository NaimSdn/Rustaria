use bevy::prelude::ResMut;
use noise::{NoiseFn, Perlin};
use crate::plugins::world::tile::{Tile, TileType, WorldMap};

pub(crate) fn generate_world(mut world_map: ResMut<WorldMap>) {

    let perlin = Perlin::new(123456789);

    world_map.world_width = 1920;
    world_map.world_height = 1080;

    let width: usize = world_map.world_width as usize;
    let height: usize = world_map.world_height as usize;

    world_map.world_tiles = vec![vec![Tile { tile_type: TileType::Air }; height]; width];

    for row  in 0..width {
        let noise_value: f64 = perlin.get([row as f64 * 0.05, 0.0]);
        let surface_height: usize = ((noise_value + 1.0) / 2.0 * height as f64) as usize;
        for col in 0..height {
            match col {
                _ if col > surface_height => {
                    world_map.world_tiles[row][col] = Tile { tile_type: TileType::Air}
                }
                _ if col == surface_height => {
                    world_map.world_tiles[row][col] = Tile { tile_type: TileType::Grass}
                }
                _ if col < surface_height && col > surface_height - 5 => {
                    world_map.world_tiles[row][col] = Tile { tile_type: TileType::Dirt}
                }
                _ => {
                    world_map.world_tiles[row][col] = Tile { tile_type: TileType::Stone}
                },
            }
        }
    }
}