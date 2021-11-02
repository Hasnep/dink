use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::components::{CanPlayerMove, Drawable, Position};
use crate::game::config::*;
use crate::game::enemy::Enemy;
use crate::game::tilemap::*;

#[derive(Clone, Debug)]
pub struct Player {}

pub fn add(mut commands: Commands) {
    commands.spawn_bundle((
        Position { x: 2, y: 4 },
        Player {},
        Drawable {
            texture_index: PLAYER_TEXTURE_INDEX,
        },
    ));
}

fn try_to_move(
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

pub fn movement(
    keys: Res<Input<KeyCode>>,
    player_query: Query<&mut Position, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(Entity, &Position), (With<Enemy>, Without<Player>)>,
    mut commands: Commands,
    mut map_query: MapQuery,
    can_player_move: ResMut<CanPlayerMove>,
) {
    // Only let the player move when the game loop is ready
    if can_player_move.0 {
        let delta = if keys.just_released(KeyCode::Left) {
            IVec2::new(-1, 0)
        } else if keys.just_released(KeyCode::Right) {
            IVec2::new(1, 0)
        } else if keys.just_released(KeyCode::Up) {
            IVec2::new(0, 1)
        } else if keys.just_released(KeyCode::Down) {
            IVec2::new(0, -1)
        } else {
            IVec2::new(0, 0)
        };

        // If a movement key was pressed
        if delta.x != 0 || delta.y != 0 {
            try_to_move(
                delta.x,
                delta.y,
                player_query,
                enemy_query,
                &mut commands,
                &mut map_query,
            );
            commands.insert_resource(CanPlayerMove(false))
        };
    }
}
