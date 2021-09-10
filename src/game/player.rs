use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::components::{Drawable, PlayerJustMoved, Position};
use crate::game::config::*;
use crate::game::enemy::Enemy;
use crate::game::tilemap::*;

#[derive(Clone, Debug)]
pub struct Player {}

pub fn add_player(mut commands: Commands) {
    commands.spawn_bundle((
        Position { x: 2, y: 4 },
        Player {},
        Drawable {
            texture_index: PLAYER_TEXTURE_INDEX,
        },
    ));
}

fn try_to_move_player(
    delta_x: i32,
    delta_y: i32,
    mut player_query: Query<&mut Position, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(Entity, &Position), (With<Enemy>, Without<Player>)>,
    commands: &mut Commands,
    map_query: &mut MapQuery,
) {
    for mut player_position in player_query.iter_mut() {
        let from = *player_position;
        let to = Position {
            x: ((from.x as i32) + delta_x) as u32,
            y: ((from.y as i32) + delta_y) as u32,
        };

        let to_tile = map_query.get_tile_entity(UVec2::new(to.x, to.y), MAP_ID, TILES_LAYER_ID);

        if to_tile.is_ok() {
            for (enemy_id, enemy_position) in enemy_query.iter() {
                // If there is an enemy there
                if enemy_position.x == to.x && enemy_position.y == to.y {
                    // The player kills the enemy
                    commands.entity(enemy_id).despawn();
                    break;
                }
            }
            // If that space has a tile then the player digs that tile
            destroy_tile(to, commands, map_query);
        } else {
            // If that space is empty then move the player
            player_position.x = to.x;
            player_position.y = to.y;

            move_tile(from, to, commands, map_query, PLAYER_TEXTURE_INDEX);
        }
    }
}

pub fn player_movement(
    keys: Res<Input<KeyCode>>,
    player_query: Query<&mut Position, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(Entity, &Position), (With<Enemy>, Without<Player>)>,
    mut commands: Commands,
    mut map_query: MapQuery,
    player_just_moved: ResMut<PlayerJustMoved>,
) {
    // Only let the player move when the game loop is ready
    if !player_just_moved.0 {
        let mut key_was_pressed = false;
        let mut direction = IVec2::new(0, 0);
        // player_position_query
        if keys.just_released(KeyCode::Left) {
            direction = IVec2::new(-1, 0);
            key_was_pressed = true;
        } else if keys.just_released(KeyCode::Right) {
            direction = IVec2::new(1, 0);
            key_was_pressed = true;
        } else if keys.just_released(KeyCode::Up) {
            direction = IVec2::new(0, 1);
            key_was_pressed = true;
        } else if keys.just_released(KeyCode::Down) {
            direction = IVec2::new(0, -1);
            key_was_pressed = true;
        }
        if key_was_pressed {
            try_to_move_player(
                direction.x,
                direction.y,
                player_query,
                enemy_query,
                &mut commands,
                &mut map_query,
            );
            commands.insert_resource(PlayerJustMoved(true))
        };
    }
}
