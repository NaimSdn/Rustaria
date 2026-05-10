use crate::plugins::world::asset::WorldTileset;
use crate::plugins::world::tile::{TileType, WorldMap};
use bevy::prelude::{Commands, Res, Sprite, TextureAtlas, Transform, Vec2};
use bevy_rapier2d::prelude::{Collider, RigidBody};
use crate::constants::TILE_SIZE;

pub(crate) fn render_world(
    mut commands: Commands,
    world_map: Res<WorldMap>,
    world_tileset: Res<WorldTileset>,
) {
    let width = world_map.world_width as usize;
    let height = world_map.world_height as usize;

    for col in 0..width {
        for row in 0..height {
            let tile_type = world_map.world_tiles[col][row].tile_type;
            let tile_index = match tile_type {
                TileType::Dirt  => 0,
                TileType::Grass => 1,
                TileType::Stone => 16,
                TileType::Air   => continue,
            };

            let x = col as f32 * TILE_SIZE - (width as f32 * TILE_SIZE / 2.0);
            let y = -(row as f32 * TILE_SIZE) + (height as f32 * TILE_SIZE / 2.0);

            commands.spawn((
                Transform::from_xyz(x, y, 0.0),
                Sprite::from_atlas_image(
                    world_tileset.0.clone(),
                    TextureAtlas {
                        layout: world_tileset.1.clone(),
                        index: tile_index,
                    },
                ),
            ));
        }
    }

    // --- Colliders: one merged collider per contiguous vertical run per column ---
    for col in 0..width {
        let mut row = 0;
        while row < height {
            if !world_map.world_tiles[col][row].tile_type.is_solid() {
                row += 1;
                continue;
            }

            let run_start = row;
            while row < height && world_map.world_tiles[col][row].tile_type.is_solid() {
                row += 1;
            }
            let run_end = row;

            let run_len = (run_end - run_start) as f32;

            let x = col as f32 * TILE_SIZE - (width as f32 * TILE_SIZE / 2.0);
            let y_top    = -(run_start as f32 * TILE_SIZE) + (height as f32 * TILE_SIZE / 2.0);
            let y_bottom = -(( run_end - 1) as f32 * TILE_SIZE) + (height as f32 * TILE_SIZE / 2.0);
            let y_center = (y_top + y_bottom) / 2.0;

            let half_height = run_len * TILE_SIZE / 2.0;

            commands.spawn((
                Transform::from_xyz(x, y_center, 0.0),
                RigidBody::Fixed,
                Collider::cuboid(TILE_SIZE / 2.0, half_height),
            ));
        }
    }
}