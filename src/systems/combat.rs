#![warn(clippy::pedantic)]

use legion::{systems::CommandBuffer, world::SubWorld};

use crate::prelude::*;

#[system]
#[read_component(WantsToAttact)]
#[read_component(Player)]
#[write_component(Health)]
pub fn combat(ecs: &mut SubWorld, command: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttact)>::query();

    let victims = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.victim))
        .collect::<Vec<_>>();

    victims.iter().for_each(|(message, victim)| {
        let is_player = ecs
            .entry_ref(*victim)
            .unwrap()
            .get_component::<Player>()
            .is_ok();

        if let Ok(health) = ecs
            .entry_mut(*victim)
            .unwrap()
            .get_component_mut::<Health>()
        {
            health.current -= 1;
            if health.current < 1 && !is_player {
                command.remove(*victim);
            }
        }
        command.remove(*message);
    });
}
