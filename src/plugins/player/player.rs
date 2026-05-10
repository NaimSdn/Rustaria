use bevy::image::TextureAtlas;
use bevy::prelude::{
    AssetServer, Assets, Commands, Component, Handle, Image, Res, ResMut, TextureAtlasLayout,
    Transform, UVec2, Vec2,
};
use bevy::sprite::Sprite;
use bevy_rapier2d::prelude::{CoefficientCombineRule, Collider, Damping, Friction, GravityScale, LockedAxes, Restitution, RigidBody, Velocity};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
#[allow(dead_code)]
pub struct PlayerStats {
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
}

#[derive(Component)]
pub struct Grounded(pub bool);

pub(crate) fn init_player(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    let handle_image: Handle<Image> = server.load("textures/player_tileset.png");
    let layout: TextureAtlasLayout =
        TextureAtlasLayout::from_grid(UVec2::new(16, 16), 16, 16, None, None);
    let handle_texture_atlas: Handle<TextureAtlasLayout> = texture_atlas_layout.add(layout);

    commands.spawn((
        Player,
        PlayerStats {
            health: 100.0,
            max_health: 100.0,
            speed: 200.0,
        },
        RigidBody::Dynamic,
        Collider::cuboid(8.0, 8.0),
        Velocity::zero(),
        GravityScale(1.0),
        LockedAxes::ROTATION_LOCKED,
        Damping {
            linear_damping: 0.0,
            angular_damping: 0.0,
        },
        Friction {
            coefficient: 0.0,
            combine_rule: CoefficientCombineRule::Min,
        },
        Restitution::coefficient(0.0),
        Grounded(false),
        Transform::from_xyz(0.0, 300.0, 1.0),
        Sprite::from_atlas_image(
            handle_image.clone(),
            TextureAtlas {
                layout: handle_texture_atlas.clone(),
                index: 0,
            },
        ),
    ));
}
