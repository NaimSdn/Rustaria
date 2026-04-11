use crate::plugins::player::player::{Player, Velocity};
use bevy::prelude::{Query, Res, Time, With};
use crate::constants::GRAVITY;

fn apply_gravity(time: Res<Time>, mut gravity_query: Query<&mut Velocity, With<Player>>) {
    let Ok(mut velocity) = gravity_query.single_mut() else {
        return;
    };

    velocity.value.y -= GRAVITY * time.delta_secs();
}
