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

pub struct CaveTheme {}

impl CaveTheme {
    pub fn new() -> Box<dyn MapThemes> {
        Box::new(Self {})
    }
}

impl MapThemes for CaveTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(','),
            TileType::Wall => to_cp437('%'),
            TileType::Exit => to_cp437('>')
        }
    }
}

pub struct DesertTheme {}

impl DesertTheme {
    pub fn new() -> Box<dyn MapThemes> {
        Box::new(Self {})
    }
}

impl MapThemes for DesertTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('~'),
            TileType::Wall => to_cp437('='),
            TileType::Exit => to_cp437('>')
        }
    }
}

pub struct IceTheme {}

impl IceTheme {
    pub fn new() -> Box<dyn MapThemes> {
        Box::new(Self {})
    }
}

impl MapThemes for IceTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('·'),
            TileType::Wall => to_cp437('█'),
            TileType::Exit => to_cp437('>')
        }
    }
}

pub struct CryptTheme {}

impl CryptTheme {
    pub fn new() -> Box<dyn MapThemes> {
        Box::new(Self {})
    }
}

impl MapThemes for CryptTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437(','),
            TileType::Wall => to_cp437('X'),
            TileType::Exit => to_cp437('>')
        }
    }
}

pub struct SwampTheme {}

impl SwampTheme {
    pub fn new() -> Box<dyn MapThemes> {
        Box::new(Self {})
    }
}

impl MapThemes for SwampTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('~'),
            TileType::Wall => to_cp437('"'),
            TileType::Exit => to_cp437('>')
        }
    }
}

pub struct LavaTheme {}

impl LavaTheme {
    pub fn new() -> Box<dyn MapThemes> {
        Box::new(Self {})
    }
}

impl MapThemes for LavaTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::Wall => to_cp437('█'),
            TileType::Exit => to_cp437('>')
        }
    }
}