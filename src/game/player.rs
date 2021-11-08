use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::game::components::{Drawable, Position};
use crate::game::config::*;
use crate::game::enemy::Enemy;
use crate::game::states::GameState;
use crate::game::tilemap::*;

#[derive(Clone, Debug)]
pub struct LastUpdate {
    last_update: f64,
}

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

    commands.spawn_bundle((LastUpdate { last_update: 0.0 },));
}

fn try_to_move(
    delta_x: i32,
    delta_y: i32,
    mut player_query: Query<&mut Position, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(Entity, &Position), (With<Enemy>, Without<Player>)>,
    commands: &mut Commands,
    map_query: &mut MapQuery,
) {
    let mut player_position = player_query
        .single_mut()
        .expect("There should always be exactly one player in the game!");

    let from = *player_position;
    let to = Position {
        x: ((from.x as i32) + delta_x) as u32,
        y: ((from.y as i32) + delta_y) as u32,
    };

    if !is_in_bounds(&IVec2::new(to.x as i32, to.y as i32)) {
        return;
    }

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

        println!("Moving from {},{} to {},{}", from.x, from.y, to.x, to.y);
        move_tile(from, to, commands, map_query, PLAYER_TEXTURE_INDEX);
    }
}

pub fn movement(
    keys: Res<Input<KeyCode>>,
    player_query: Query<&mut Position, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(Entity, &Position), (With<Enemy>, Without<Player>)>,
    mut commands: Commands,
    mut map_query: MapQuery,
    mut game_state: ResMut<State<GameState>>,
    time: Res<Time>,
    mut last_update_query: Query<&mut LastUpdate>,
) {
    let current_time = time.seconds_since_startup();
    if let Ok(mut last_update) = last_update_query.single_mut() {
        if current_time - last_update.last_update > 0.0 {
            for key in keys.get_just_released() {
                let delta = match key {
                    KeyCode::Left => IVec2::new(-1, 0),
                    KeyCode::Right => IVec2::new(1, 0),
                    KeyCode::Up => IVec2::new(0, 1),
                    KeyCode::Down => IVec2::new(0, -1),
                    _ => IVec2::new(0, 0),
                };

                if delta.x != 0 || delta.y != 0 {
                    let _did_move = try_to_move(
                        delta.x,
                        delta.y,
                        player_query,
                        enemy_query,
                        &mut commands,
                        &mut map_query,
                    );

                    game_state.replace(GameState::EnemyTurn).unwrap();
                    last_update.last_update = current_time;
                    return;
                }
            }
        }
    }
}
