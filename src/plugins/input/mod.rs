use crate::plugins::player::player::{Player, PlayerStats, Velocity};
use bevy::app::{App};
use bevy::prelude::{ButtonInput, IntoScheduleConfigs, KeyCode, Plugin, Query, Res, Transform, Update, With};
use bevy::time::Time;

pub struct InputPlugin;

fn moving_player(
    button_input: Res<ButtonInput<KeyCode>>,
    mut input_query: Query<(&mut Velocity, &PlayerStats), With<Player>>,
) {
    let Ok((mut velocity, stats)) = input_query.single_mut() else {
        return;
    };

    if button_input.pressed(KeyCode::KeyA) {
        velocity.value.x = -stats.speed;
    } else if button_input.pressed(KeyCode::KeyD) {
        velocity.value.x = stats.speed;
    } else if button_input.pressed(KeyCode::Space) {
        velocity.value.y = stats.speed * 2.0;
    } else {
        velocity.value.x = 0.0;
        velocity.value.y = 0.0;
    }
}

fn apply_velocity(
    mut query: Query<(&Velocity, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let Ok((velocity, mut transform)) = query.single_mut() else {
        return;
    };
    transform.translation.x += velocity.value.x * time.delta_secs();
    transform.translation.y += velocity.value.y * time.delta_secs();
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (moving_player, apply_velocity).chain());
    }
}
