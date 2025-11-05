#![allow(dead_code, unused_variables)]

use legion::{systems::CommandBuffer, world::SubWorld};

use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Players)]
#[read_component(Enemy)]
pub fn collision(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());
    players.iter(ecs).for_each(|pos| player_pos = *pos);

    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    enemies
        .iter(ecs)
        .filter(|(_, pos)| **pos == player_pos)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        });
}
