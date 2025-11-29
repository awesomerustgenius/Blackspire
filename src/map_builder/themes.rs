use crate::prelude::*;

pub struct DungeonTheme {}

impl DungeonTheme {
    pub fn new() -> Box<dyn MapThemes> {
        Box::new(Self {})
    }
}

impl MapThemes for DungeonTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('#'),
            TileType::Exit => to_cp437('>')
        }
    }
}

pub struct ForestTheme {}

impl ForestTheme {
    pub fn new() -> Box<dyn MapThemes> {
        Box::new(Self {})
    }
}

impl MapThemes for ForestTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(';'),
            TileType::Wall => to_cp437('"'),
            TileType::Exit => to_cp437('>')
        }
    }
}
