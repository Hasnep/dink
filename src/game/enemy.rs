use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::seq::SliceRandom;

use crate::game::components::{CanPlayerMove, Drawable, Position};
use crate::game::config::*;
use crate::game::states::GameState;
use crate::game::tilemap::*;

#[derive(Component)]
pub struct Enemy {}

pub fn add(mut commands: Commands) {
    for i in 2..10 {
        commands.spawn_bundle((
            Position { x: 4, y: i },
            Enemy {},
            Drawable {
                texture_index: ENEMY_TEXTURE_INDEX,
            },
        ));
    }
}

pub fn movement(
    mut enemy_query: Query<&mut Position, With<Enemy>>,
    mut commands: Commands,
    mut map_query: MapQuery,
    mut game_state: ResMut<State<GameState>>,
) {
    for mut enemy_position in enemy_query.iter_mut() {
        // Get spaces next to the enemy
        // let neighbours: Vec<&(IVec2, Option<Entity>)> =

        let neighbours = map_query.get_neighbouring_pos(
            TilePos(enemy_position.x, enemy_position.y),
            MAP_ID,
            TILES_LAYER_ID,
        );

        let neighbours = neighbours
            .iter()
            // Only the neighbours in the cardinal directions
            .take(4)
            // Keep
            .map(|n| n.unwrap())
            .collect::<Vec<Entity>>();

        let to_position_and_tile = neighbours.choose(&mut rand::thread_rng());

        match to_position_and_tile {
            Some(to_position_and_tile) => {
                let from = *enemy_position;
                let to = to_position_and_tile.0;

                let to = Position {
                    x: to.0 as u32,
                    y: to.1 as u32,
                };

                // Move the enemy
                enemy_position.x = to.x;
                enemy_position.y = to.y;
                // Move the enemy's sprite
                move_tile(from, to, &mut commands, &mut map_query, ENEMY_TEXTURE_INDEX);
            }
            None => {}
        }
    }
    println!("It's now the player's turn");
    game_state.replace(GameState::PlayerTurn).unwrap();
}
