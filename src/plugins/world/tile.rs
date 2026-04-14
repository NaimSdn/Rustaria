use bevy::prelude::Resource;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum TileType {
    Air,
    Grass,
    Dirt,
    Stone,
}

impl TileType {
    pub fn is_solid(&self) -> bool {
        matches!(self, TileType::Stone | TileType::Dirt | TileType::Grass)
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Tile {
    pub tile_type: TileType,
}

#[derive(Resource, Default)]
pub struct WorldMap {
    pub world_width: u32,
    pub world_height: u32,
    pub world_tiles: Vec<Vec<Tile>>,
}
