#![warn(clippy::pedantic)]

use legion::{systems::CommandBuffer, world::SubWorld};

use crate::prelude::*;

#[system]
#[read_component(WantsToAttact)]
#[read_component(Player)]
#[write_component(Health)]
#[read_component(Carried)]
#[read_component(Damage)]
pub fn combat(ecs: &mut SubWorld, command: &mut CommandBuffer) {
    let mut attackers = <(Entity, &WantsToAttact)>::query();

    let victims = attackers
        .iter(ecs)
        .map(|(entity, attack)| (*entity, attack.attacker, attack.victim))
        .collect::<Vec<_>>();

    victims.iter().for_each(|(message, attacker, victim)| {
        let base_damage = if let Ok(v) = ecs.entry_ref(*attacker) {
            if let Ok(damage) = v.get_component::<Damage>() {
                damage.0
            } else {
                0
            }
        } else {
            0
        };

        let weapon_damage: i32 = <(&Carried, &Damage)>::query()
            .iter(ecs)
            .filter(|(carried, _)| carried.0 == *attacker)
            .map(|(_, damage)| damage.0)
            .sum();

        let final_damage = base_damage + weapon_damage;

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
            health.current -= final_damage;
            if health.current < 1 && !is_player {
                command.remove(*victim);
            }
        }
        command.remove(*message);
    });
}
