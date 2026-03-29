use bevy::prelude::{Camera2d, Commands, Res, Sprite, TextureAtlas, Transform};
use crate::plugins::world::asset::WorldTileset;
use crate::plugins::world::tile::{TileType, WorldMap};

pub(crate) fn render_world(mut commands: Commands, world_map: Res<WorldMap>, world_tileset: Res<WorldTileset>){
    commands.spawn(Camera2d);
    for col in 0..world_map.world_width as usize {
        for row in 0.. world_map.world_height as usize {
            if world_map.world_tiles[col][row].tile_type == TileType::Air {
                continue;
            }

            let tile_index = match world_map.world_tiles[col][row].tile_type {
                TileType::Dirt => 0,
                TileType::Grass  => 1,
                TileType::Stone => 16,
                TileType::Air   => continue,
            };

            let x = col as f32 * 16.0 - (world_map.world_width as f32 * 16.0 / 2.0);
            let y = row as f32 * 16.0 - (world_map.world_height as f32 * 16.0 / 2.0);

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
}