use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::components::{Action, GoingToTakeAction, Position};
use crate::game::config::{MAP_ID, WALLS_LAYER_ID};
use crate::game::tilemap::is_in_bounds;

pub fn take_action(
    mut moving_query: Query<(&mut Position, &mut GoingToTakeAction)>,
    map_query: MapQuery,
) {
    for (mut entity_position, mut chosen_action) in moving_query.iter_mut() {
        let from = *entity_position;
        let delta = match &chosen_action.action {
            Some(Action::North) => IVec2::new(0, 1),
            Some(Action::South) => IVec2::new(0, -1),
            Some(Action::West) => IVec2::new(-1, 0),
            Some(Action::East) => IVec2::new(1, 0),
            _ => IVec2::new(0, 0),
        };
        let to = IVec2::new(
            (from.x as i32).wrapping_add(delta.x),
            (from.y as i32).wrapping_add(delta.y),
        );

        let is_to_position_in_bounds = is_in_bounds(IVec2::new(to.x, to.y));
        if is_to_position_in_bounds {
            let will_collide_with_wall = map_query
                .get_tile_entity(UVec2::new(to.x as u32, to.y as u32), MAP_ID, WALLS_LAYER_ID)
                .is_ok();
            if !will_collide_with_wall {
                // Move the entity
                entity_position.x = to.x as u32;
                entity_position.y = to.y as u32;
            }
        }

        // Reset desired action
        chosen_action.action = None;
    }
}
