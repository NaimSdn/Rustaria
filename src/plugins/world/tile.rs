use bevy::prelude::Resource;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TileType {
    Air,
    Grass,
    Dirt,
    Stone,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Tile {
    pub tile_type: TileType,
}

#[derive(Resource)]
#[derive(Default)]
pub struct WorldMap {
    pub world_width : u32,
    pub world_height : u32,
    pub world_tiles : Vec<Vec<Tile>>,
}