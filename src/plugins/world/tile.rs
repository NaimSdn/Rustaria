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
pub struct WorldMap {
    world_width : u32,
    world_height : u32,
    world_tiles : Vec<Vec<Tile>>,
}