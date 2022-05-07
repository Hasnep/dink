use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::components::Position;
use crate::game::config::*;

pub fn destroy_tile(tile_position: Position, commands: &mut Commands, map_query: &mut MapQuery) {
    let tile_position = TilePos(tile_position.x, tile_position.y);
    let _ = map_query
        .despawn_tile(commands, tile_position, MAP_ID, TILES_LAYER_ID)
        .expect("Oh no something went wrong with de-spawning a tile!");
    map_query.notify_chunk_for_tile(tile_position, MAP_ID, TILES_LAYER_ID);
}

pub fn move_tile(
    from: &Position,
    to: &Position,
    commands: &mut Commands,
    map_query: &mut MapQuery,
    texture_index: u16,
) {
    let from = TilePos(from.x, from.y);
    let to = TilePos(to.x, to.y);
    let _ = map_query
        .get_tile_entity(from, MAP_ID, TILES_LAYER_ID)
        .expect("Tried to move a tile that doesn't exist!");
    let _ = map_query
        .despawn_tile(commands, from, MAP_ID, TILES_LAYER_ID)
        .expect("Oh no something went wrong with de-spawning a tile!");
    let _ = map_query
        .set_tile(
            commands,
            to,
            Tile {
                texture_index: texture_index,
                ..Default::default()
            },
            MAP_ID,
            TILES_LAYER_ID,
        )
        .expect(&format!("Couldn't set the new tile at {},{}!", to.0, to.1));
    map_query.notify_chunk_for_tile(from, MAP_ID, TILES_LAYER_ID);
    map_query.notify_chunk_for_tile(to, MAP_ID, TILES_LAYER_ID);
}

pub fn is_in_bounds(position: &IVec2) -> bool {
    return position.x >= 0
        && position.y >= 0
        && position.x <= (CHUNK_SIZE * N_CHUNKS_X) as i32
        && position.y <= (CHUNK_SIZE * N_CHUNKS_Y) as i32;
}
