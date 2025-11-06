#![allow(unused_imports, dead_code, unused_variables)]
use std::collections::btree_map::Keys;

use legion::{systems::CommandBuffer, world::SubWorld};

use crate::{player, prelude::*};

// The system macro transforms my function into player_input_system, and wrap it with all of the extra code Legion requires to construct a system.

#[system]
#[read_component(Point)]
#[read_component(Players)]
pub fn player_input(
    ecs: &mut SubWorld,
    command: &mut CommandBuffer,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());

    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        players.iter(ecs).for_each(|(entity, pos)| {
          let destination = *pos + delta;
          command.push((
            (),
            WantsToMove {
              entity: *entity,
              destination,
            },
          ));
        });
        *turn_state = TurnState::PlayerTurn;
      }
}
