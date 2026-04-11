use bevy::image::TextureAtlas;
use bevy::prelude::{
    AssetServer, Assets, Commands, Component, Handle, Image, Res, ResMut, TextureAtlasLayout,
    Transform, UVec2, Vec2,
};
use bevy::sprite::Sprite;

#[derive(Component)]
pub struct Player;

// Unused PlayerStats health/max_health/speed for now.
#[derive(Component)]
#[allow(dead_code)]
pub struct PlayerStats {
    pub health: f32,
    pub max_health: f32,
    pub speed: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub value: Vec2,
}

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
        Velocity {
            value: Vec2::new(0.0, 0.0),
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Sprite::from_atlas_image(
            handle_image.clone(),
            TextureAtlas {
                layout: handle_texture_atlas.clone(),
                index: 0,
            },
        ),
    ));
}
