use bevy::prelude::{AssetServer, Assets, Commands, Handle, Image, Res, ResMut, Resource, TextureAtlasLayout, UVec2};

#[derive(Resource)]
pub struct WorldTileset(Handle<Image>, Handle<TextureAtlasLayout>);

pub (crate) fn load_image(mut commands: Commands, server: Res<AssetServer>, mut texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>) {
    let handle_image: Handle<Image> = server.load("textures/map_tileset.png");
    let layout: TextureAtlasLayout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 16, 16, None, None);
    let handle_texture_atlas: Handle<TextureAtlasLayout> = texture_atlas_layout.add(layout);

    commands.insert_resource(WorldTileset(handle_image, handle_texture_atlas));
}