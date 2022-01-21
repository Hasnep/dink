use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::components::{Drawable, Position};
use crate::game::config::{CHUNK_SIZE, MAP_ID, N_CHUNKS_X, N_CHUNKS_Y, OBJECTS_LAYER_ID};

use crate::game::components::HaveUpdatedTilemap;

pub fn update_tilemap(
    drawable_entities_query: Query<(&Position, &Drawable)>,
    mut have_updated_tilemap_query: Query<&mut HaveUpdatedTilemap>,
    mut map_query: MapQuery,
    mut commands: Commands,
) {
    for x in 0..(CHUNK_SIZE * N_CHUNKS_X) {
        for y in 0..(CHUNK_SIZE * N_CHUNKS_Y) {
            let tile_position = UVec2::new(x, y);
            if map_query
                .get_tile_entity(tile_position, MAP_ID, OBJECTS_LAYER_ID)
                .is_ok()
            {
                let _ = map_query
                    .despawn_tile(&mut commands, tile_position, MAP_ID, OBJECTS_LAYER_ID)
                    .expect(&format!("Couldn't despawn tile at ({},{}).", x, y));
                map_query.notify_chunk_for_tile(tile_position, MAP_ID, OBJECTS_LAYER_ID);
            }
        }
    }

    for (entity_position, texture_index) in drawable_entities_query.iter() {
        let tile_position = UVec2::new(entity_position.x, entity_position.y);
        let _ = map_query
            .set_tile(
                &mut commands,
                tile_position,
                Tile {
                    texture_index: texture_index.texture_index,
                    ..Default::default()
                },
                MAP_ID,
                OBJECTS_LAYER_ID,
            )
            .expect(&format!(
                "Couldn't set the new tile at ({},{}).",
                entity_position.x, entity_position.y
            ));
        map_query.notify_chunk_for_tile(tile_position, MAP_ID, OBJECTS_LAYER_ID);
    }

    let mut have_updated_tilemap = have_updated_tilemap_query
        .single_mut()
        .expect("Cannot have more than one HaveUpdatedTilemap entity.");
    have_updated_tilemap.0 = true;
}

pub fn is_in_bounds(position: IVec2) -> bool {
    return position.x >= 0
        && position.y >= 0
        && position.x < (CHUNK_SIZE * N_CHUNKS_X) as i32
        && position.y < (CHUNK_SIZE * N_CHUNKS_Y) as i32;
}
