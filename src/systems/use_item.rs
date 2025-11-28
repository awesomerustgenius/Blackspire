use crate::prelude::*;
use legion::{systems::CommandBuffer, world::SubWorld};

#[system]
#[read_component(ProvideHealing)]
#[read_component(ProvidedDungeonMap)]
#[write_component(Health)]
#[read_component(ActivateItem)]
pub fn use_item(ecs: &mut SubWorld, commands: &mut CommandBuffer, #[resource] map: &mut Map) {
    let mut healing_to_apply = Vec::<(Entity, i32)>::new();

    <(Entity, &ActivateItem)>::query()
        .iter(ecs)
        .for_each(|(entity, activate)| {
            let item = ecs.entry_ref(activate.item);

            if let Ok(item) = item {
                if let Ok(healing) = item.get_component::<ProvideHealing>() {
                    healing_to_apply.push((activate.used_by, healing.amount));
                }

                if let Ok(_) = item.get_component::<ProvidedDungeonMap>() {
                    map.revealed_tiles.iter_mut().for_each(|tile| *tile = true);
                }
            }

            commands.remove(activate.item);
            commands.remove(*entity);
        });

    for heal in healing_to_apply {
        if let Ok(mut target) = ecs.entry_mut(heal.0) {
            if let Ok(health) = target.get_component_mut::<Health>() {
                health.current = i32::min(health.max, health.current + heal.1)
            }
        }
    }
}
