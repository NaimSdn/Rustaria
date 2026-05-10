use crate::plugins::player::player::{Grounded, Player, PlayerStats};
use bevy::prelude::{App, ButtonInput, Entity, IntoScheduleConfigs, KeyCode, Plugin, Query, Res, Transform, Update, Vec2, With};
use bevy_rapier2d::prelude::{QueryFilter, RapierContext, Real, Velocity};

pub struct InputPlugin;

fn moving_player(
    button_input: Res<ButtonInput<KeyCode>>,
    mut input_query: Query<(&mut Velocity, &PlayerStats, &Grounded), With<Player>>,
) {
    let Ok((mut velocity, stats, grounded)) = input_query.single_mut() else {
        return;
    };

    if button_input.pressed(KeyCode::KeyA) {
        velocity.linvel.x = -stats.speed;
    } else if button_input.pressed(KeyCode::KeyD) {
        velocity.linvel.x = stats.speed;
    } else {
        velocity.linvel.x = 0.0;
    }

    if button_input.just_pressed(KeyCode::Space) && grounded.0{
        velocity.linvel.y = stats.speed * 2.0;
    }
}

fn update_grounded(
    mut player_query: Query<(&Velocity, &mut Grounded), With<Player>>,
) {
    let Ok((velocity, mut grounded)) = player_query.single_mut() else { return };
    grounded.0 = velocity.linvel.y.abs() < 1.0;
}


impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_grounded, moving_player).chain());
    }
}