#![allow(clippy::pedantic, unused_imports)]
mod camera;
mod components;
mod map;
mod map_builder;
mod player;
mod spawner;
mod systems;
mod turn_state;
mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::player::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use crate::turn_state::*;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
}

use std::{collections::HashSet, process};

use legion::systems::{CommandBuffer, Resource};
use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    input_system: Schedule,
    player_system: Schedule,
    monster_system: Schedule,
}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rand = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rand);
        spawn_player(&mut ecs, map_builder.player_start);
        // spawn_amulet_of_yala(&mut ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;

        map_builder
            .monster_spawns
            .iter()
            .for_each(|pos| spawn_entity(&mut ecs, &mut rand, *pos));
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_entity(&mut ecs, &mut rand, pos));

        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        resources.insert(TurnState::AwaitingInput);
        resources.insert(map_builder.themes);

        Self {
            ecs,
            resources,
            input_system: build_input_scheduler(),
            player_system: build_player_scheduler(),
            monster_system: build_monster_scheduler(),
        }
    }

    fn game_over(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(3);
        ctx.print_color_centered(18, RED, BLACK, "Your quest has ended");
        ctx.print_color_centered(
            20,
            WHITE,
            BLACK,
            "Slain by a monster, your hero's journey has come to a premature end",
        );
        ctx.print_color_centered(
            21,
            WHITE,
            BLACK,
            "The Amulet of Yala remains unclaimed, and your home town is not saved.",
        );
        ctx.print_color_centered(
            25,
            YELLOW,
            BLACK,
            "Don't worry, you can always try again with a new hero.",
        );

        ctx.print_color_centered(27, GREEN, BLACK, "Press 1 to play again.");
        ctx.print_color_centered(29, GREEN, BLACK, "Press 2 to exit.");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }

        if let Some(VirtualKeyCode::Key2) = ctx.key {
            std::process::exit(0);
        }
    }

    fn victory(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(2);
        ctx.print_color_centered(18, GREEN, BLACK, "You have won!");
        ctx.print_color_centered(
            20,
            WHITE,
            BLACK,
            "You put on the Amulet of Yala and feel its power course through your veins...",
        );
        ctx.print_color_centered(
            21,
            WHITE,
            BLACK,
            "Your town is saved, and you can return to your normal life.",
        );
        ctx.print_color_centered(27, GREEN, BLACK, "Press 1 to play again..");
        ctx.print_color_centered(28, GREEN, BLACK, "Press 2 to exit..");

        if let Some(VirtualKeyCode::Key1) = ctx.key {
            self.reset_game_state();
        }
        if let Some(VirtualKeyCode::Key2) = ctx.key {
            process::exit(0);
        }
    }

    fn reset_game_state(&mut self) {
        self.ecs = World::default();
        self.resources = Resources::default();
        let mut rand = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rand);
        spawn_player(&mut self.ecs, map_builder.player_start);
        // spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
        map_builder.map.tiles[exit_idx] = TileType::Exit;
        map_builder
            .monster_spawns
            .iter()
            .for_each(|pos| spawn_entity(&mut self.ecs, &mut rand, *pos));

        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_entity(&mut self.ecs, &mut rand, pos));
        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.themes);
    }

    fn advanced_level(&mut self) {
        // 1. Remove all entities from the ECS World that aren’t either the player or items carried by the player
        let player = *<(Entity)>::query()
            .filter(component::<Player>())
            .iter(&mut self.ecs)
            .nth(0)
            .unwrap();
        // 2. Set the is_dirty flag on the player’s FieldOfView to ensure that the map renders correctly on the next turn
        let mut entities_to_keep = HashSet::new();
        entities_to_keep.insert(player);

        <(Entity, &Carried)>::query()
            .iter(&self.ecs)
            .filter(|(_, carry)| carry.0 == player)
            .map(|(entity, _)| *entity)
            .for_each(|entity| {
                entities_to_keep.insert(entity);
            });

        let mut cb = CommandBuffer::new(&mut self.ecs);
        for e in Entity::query().iter(&self.ecs) {
            if !entities_to_keep.contains(e) {
                cb.remove(*e);
            }
        }
        cb.flush(&mut self.ecs, &mut self.resources);
        <(&mut FieldOfView)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|fov| fov.is_dirty = true);

        // 3. Generate a new level as you did before.
        let mut rng = RandomNumberGenerator::new();
        let mut map_builder = MapBuilder::new(&mut rng);
        let mut map_level = 0;
        <(&mut Player, &mut Point)>::query()
            .iter_mut(&mut self.ecs)
            .for_each(|(player, pos)| {
                player.map_level += 1;
                map_level = player.map_level;
                pos.x = map_builder.player_start.x;
                pos.y = map_builder.player_start.y;
            });

        // 4. Check the current level number: if it’s 0 or 1, spawn an exit staircase; if it’s 2, spawn the Amulet of Yala.
        if map_level == 2 {
            spawn_amulet_of_yala(&mut self.ecs, map_builder.amulet_start);
        } else {
            let exit_idx = map_builder.map.point2d_to_index(map_builder.amulet_start);
            map_builder.map.tiles[exit_idx] = TileType::Exit;
        }
        
        // 5. Finish setting up spawned monsters and resources as you did before
        map_builder
            .monster_spawns
            .iter()
            .for_each(|pos| spawn_entity(&mut self.ecs, &mut rng, *pos));

        self.resources.insert(map_builder.map);
        self.resources.insert(Camera::new(map_builder.player_start));
        self.resources.insert(TurnState::AwaitingInput);
        self.resources.insert(map_builder.themes);
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        ctx.set_active_console(2);
        ctx.cls();
        ctx.set_active_console(3);
        ctx.cls();
        self.resources.insert(ctx.key);
        ctx.set_active_console(0);
        self.resources.insert(Point::from_tuple(ctx.mouse_pos()));

        let current_state = self.resources.get::<TurnState>().unwrap().clone();

        match current_state {
            TurnState::AwaitingInput => self
                .input_system
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::PlayerTurn => self
                .player_system
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::MonsterTurn => self
                .monster_system
                .execute(&mut self.ecs, &mut self.resources),
            TurnState::GameOver => {
                self.game_over(ctx);
            }
            TurnState::Victory => {
                self.victory(ctx);
            }
            TurnState::NextLevel => {
                self.advanced_level();
            }
        }

        render_draw_buffer(ctx).expect("Render error");
    }
}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .with_simple_console_no_bg(DISPLAY_WIDTH * 2, DISPLAY_HEIGHT * 2, "terminal8x8.png")
        .with_sparse_console_no_bg(DISPLAY_WIDTH * 2, DISPLAY_HEIGHT * 2, "terminal8x8.png")
        .build()?;

    main_loop(context, State::new())
}
