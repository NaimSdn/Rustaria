use crate::plugins::player::player::{Player, Velocity};
use bevy::prelude::{App, Plugin, Query, Res, Time, Update, With};
use crate::constants::GRAVITY;

pub struct PhysicsPlugin;

fn apply_gravity(time: Res<Time>, mut gravity_query: Query<&mut Velocity, With<Player>>) {
    let Ok(mut velocity) = gravity_query.single_mut() else {
        return;
    };

    velocity.value.y -= GRAVITY * time.delta_secs();
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_gravity);
    }
}
