use legion::world::SubWorld;

use crate::prelude::*;

#[system]
#[read_component(Health)]
#[read_component(Player)]
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
pub fn hud(ecs: &SubWorld) {
    let mut health_query = <&Health>::query().filter(component::<Player>());

    <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, _)| Some(*entity))
        .unwrap();

    let mut item_query = <(&Item, &Name, &Carried)>::query();

    let (player, map_level) = <(Entity, &Player)>::query()
        .iter(ecs)
        .find_map(|(entity, player)| Some((*entity, player.map_level)))
        .unwrap();

    if let Some(player_health) = health_query.iter(ecs).nth(0) {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(2);

        // Top bar: Health bar at y=0
        draw_batch.bar_horizontal(
            Point::new(0, 0),
            SCREEN_WIDTH,
            player_health.current,
            player_health.max,
            ColorPair::new(RED, BLACK),
        );
        
        // Health text overlay on the bar (centered)
        draw_batch.print_color_centered(
            0,
            format!(
                " Health: {} / {} ",
                player_health.current, player_health.max
            ),
            ColorPair::new(WHITE, RED),
        );

        // Dungeon level in top right corner
        draw_batch.print_color_right(
            Point::new(SCREEN_WIDTH - 1, 0),
            format!("Level {}", map_level + 1),
            ColorPair::new(YELLOW, BLACK),
        );

        // Instructions/controls at y=1 (below health bar)
        draw_batch.print_color(
            Point::new(1, 1),
            "Arrow keys: Move | G: Pick up | 0-9: Use item",
            ColorPair::new(CYAN, BLACK),
        );

        // Inventory section on the left side starting at y=3
        let mut item_count = 0;
        let inventory_start_y = 3;
        let max_items_display = 8; // Limit items to prevent overlap with game area
        
        item_query
            .iter(ecs)
            .filter(|(_, _, carried)| carried.0 == player)
            .take(max_items_display)
            .for_each(|(_, name, _)| {
                let y_pos = inventory_start_y + item_count;
                draw_batch.print_color(
                    Point::new(1, y_pos),
                    format!("[{}] {}", item_count, name.0),
                    ColorPair::new(GREEN, BLACK),
                );
                item_count += 1;
            });

        // Inventory header (only show if there are items)
        if item_count > 0 {
            draw_batch.print_color(
                Point::new(1, inventory_start_y - 1),
                format!("Inventory ({}):", item_count),
                ColorPair::new(YELLOW, BLACK),
            );
        }

        draw_batch.submit(10000).expect("Batch error");
    }
}
