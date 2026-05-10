use crate::plugins::player::player::Player;
use crate::plugins::world::tile::WorldMap;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::camera::Camera2d;
use bevy::prelude::{Commands, Query, Res, Transform, With, Without};
use crate::constants::TILE_SIZE;

pub(crate) fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
pub(crate) fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
    world_map: Res<WorldMap>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let world_x_min = -(world_map.world_width as f32 * TILE_SIZE) / 2.0;
    let world_x_max = (world_map.world_width as f32 * TILE_SIZE) / 2.0;
    let world_y_min = -(world_map.world_height as f32 * TILE_SIZE) / 2.0;
    let world_y_max = (world_map.world_height as f32 * TILE_SIZE) / 2.0;

    camera_transform.translation.x = player_transform.translation.x.clamp(world_x_min, world_x_max);
    camera_transform.translation.y = player_transform.translation.y.clamp(world_y_min, world_y_max);
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow);
    }
}
