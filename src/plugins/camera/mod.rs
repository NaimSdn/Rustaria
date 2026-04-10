use crate::plugins::player::player::Player;
use bevy::app::{App, Plugin, Startup, Update};
use bevy::camera::Camera2d;
use bevy::prelude::{Commands, Query, Transform, With, Without};

pub(crate) fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
pub(crate) fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let Ok(mut camera_transform) = camera_query.single_mut() else {
        return;
    };

    let Ok(player_transform) = player_query.single() else {
        return;
    };

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, camera_follow);
    }
}
