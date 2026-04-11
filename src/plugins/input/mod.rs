use crate::plugins::player::player::{Player, PlayerStats, Velocity};
use bevy::app::{App};
use bevy::prelude::{ButtonInput, IntoScheduleConfigs, KeyCode, Plugin, Query, Res, Time, Transform, Update, With};

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
    } else {
        velocity.value.x = 0.0;
    }
}

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, moving_player);
    }
}
