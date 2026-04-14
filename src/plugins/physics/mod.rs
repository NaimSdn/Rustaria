use crate::constants::{GRAVITY, TILE_SIZE};
use crate::plugins::player::player::{Player, Velocity};
use crate::plugins::world::tile::WorldMap;
use bevy::prelude::{App, IntoScheduleConfigs, Plugin, Query, Res, Time, Transform, Update, With};

pub struct PhysicsPlugin;

fn apply_gravity(time: Res<Time>, mut gravity_query: Query<&mut Velocity, With<Player>>) {
    let Ok(mut velocity) = gravity_query.single_mut() else {
        return;
    };

    velocity.value.y -= GRAVITY * time.delta_secs();
}

fn resolve_collision(
    world_map: Res<WorldMap>,
    mut collision_query: Query<(&mut Transform, &mut Velocity), With<Player>>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut velocity)) = collision_query.single_mut() else {
        return;
    };

    let half = TILE_SIZE / 2.0;
    let dt = time.delta_secs();

    // 1. Predict next position
    let next_x = transform.translation.x + velocity.value.x * dt;
    let next_y = transform.translation.y + velocity.value.y * dt;

    // 2. Bounding box edges at predicted position
    let left   = next_x - half;
    let right  = next_x + half;
    let bottom = next_y - half;


    // 3. Convert to tile coordinates
    let next_row_bottom = ((-bottom + world_map.world_height as f32 * TILE_SIZE / 2.0) / TILE_SIZE).round() as usize;
    let left_col  = ((left  + world_map.world_width as f32 * TILE_SIZE / 2.0) / TILE_SIZE) as usize;
    let right_col = ((right + world_map.world_width as f32 * TILE_SIZE / 2.0) / TILE_SIZE) as usize;

    // 4. Bounds check — player outside world, just move freely
    if next_row_bottom >= world_map.world_height as usize
        || left_col >= world_map.world_width as usize
        || right_col >= world_map.world_width as usize
    {
        transform.translation.x = next_x;
        transform.translation.y = next_y;
        return;
    }

    // 5. Always apply horizontal movement
    transform.translation.x = next_x;

    // 6. Vertical collision check
    let left_tile  = world_map.world_tiles[left_col][next_row_bottom];
    let right_tile = world_map.world_tiles[right_col][next_row_bottom];

    if left_tile.tile_type.is_solid() || right_tile.tile_type.is_solid() {
        let tile_center_y = -(next_row_bottom as f32 * TILE_SIZE)
            + (world_map.world_height as f32 * TILE_SIZE / 2.0);
        let tile_top = tile_center_y + half;
        transform.translation.y = tile_top + half;
        velocity.value.y = 0.0;
    } else {
        transform.translation.y = next_y;
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (apply_gravity, resolve_collision).chain());
    }
}
