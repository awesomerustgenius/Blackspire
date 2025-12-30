#![allow(dead_code, unused_variables)]
use crate::{
    map_builder::{
        automata::CellularAutomataArchitect,
        drunkard::DrunkArchitect,
        prefab::apply_prefab,
        rooms::RoomsArchitect,
        themes::{DungeonTheme, ForestTheme, CaveTheme, DesertTheme, IceTheme, CryptTheme, SwampTheme, LavaTheme},
    },
    prelude::*,
};
mod automata;
mod drunkard;
mod empty;
mod prefab;
mod rooms;
mod themes;
use empty::EmptyArchitect;

trait MapArchitect {
    fn new(&mut self, rand: &mut RandomNumberGenerator) -> MapBuilder;
}

pub trait MapThemes: Send + Sync {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
}

pub const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
    pub amulet_start: Point,
    pub monster_spawns: Vec<Point>,
    pub themes: Box<dyn MapThemes>,
}

impl MapBuilder {
    pub fn new(rand: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rand.range(0, 3) {
            0 => Box::new(DrunkArchitect {}),
            1 => Box::new(RoomsArchitect {}),
            _ => Box::new(CellularAutomataArchitect {}),
        };

        let mut mb = architect.new(rand);
        apply_prefab(&mut mb, rand);

        mb.themes = match rand.range(0, 8) {
            0 => DungeonTheme::new(),
            1 => ForestTheme::new(),
            2 => CaveTheme::new(),
            3 => DesertTheme::new(),
            4 => IceTheme::new(),
            5 => CryptTheme::new(),
            6 => SwampTheme::new(),
            _ => LavaTheme::new(),
        };

        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distance(&self) -> Point {
        let dijkistra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );
        const UNREACHABLE: &f32 = &f32::MAX;

        self.map.index_to_point2d(
            dijkistra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap_or((self.map.point2d_to_index(self.player_start), &0.0))
                .0,
        )
    }

    fn build_random_rooms(&mut self, rand: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            // Generate random rooms.
            let rooms = Rect::with_size(
                rand.range(1, SCREEN_WIDTH - 10),
                rand.range(1, SCREEN_HEIGHT - 10),
                rand.range(2, 10),
                rand.range(2, 10),
            );

            let mut overlap = false;
            for room in self.rooms.iter() {
                // Check whether the rooms intersects.
                if room.intersect(&rooms) {
                    overlap = true;
                }
            }
            if !overlap {
                rooms.for_each(|room| {
                    // check whether the rooms are withing the map boundaries.
                    if room.x > 0 && room.x < SCREEN_WIDTH && room.y > 0 && room.y < SCREEN_HEIGHT {
                        let idx = map_idx(room.x, room.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
            }
            self.rooms.push(rooms);
        }
    }

    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor
            }
        }
    }

    fn build_corridors(&mut self, rand: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        // Sorting rooms by their center points before allocating corridors. This helps to avoid long corridors that almost certainly overlap with other rooms.
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (idx, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[idx - 1].center();
            let new = room.center();

            if rand.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }

    fn spawn_monster(&self, start: &Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_MONSTERS: usize = 50;
        let mut spawnable_tiles = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, tile)| {
                **tile == TileType::Floor
                    && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect::<Vec<Point>>();

        let mut spawns = Vec::new();
        for _ in 0..NUM_MONSTERS {
            let target_idx = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_idx].clone());
            spawnable_tiles.remove(target_idx);
        }

        spawns
    }
}
