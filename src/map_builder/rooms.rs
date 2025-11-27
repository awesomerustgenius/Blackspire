use super::MapArchitect;
use crate::prelude::*;

pub struct RoomsArchitect {}

impl MapArchitect for RoomsArchitect {
    fn new(&mut self, rand: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            amulet_start: Point::zero(),
            player_start: Point::zero(),
            monster_spawns: Vec::new(),
        };

        mb.fill(TileType::Wall);
        mb.build_random_rooms(rand);
        mb.build_corridors(rand);

        mb.player_start = mb.rooms[0].center();
        mb.amulet_start = mb.find_most_distance();

        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }

        mb
    }
}
