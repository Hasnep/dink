use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;

use crate::game::components::{
    Action, Drawable, EnemyTag, GoingToTakeAction, MoveRandomlyTag, Position,
};
use crate::game::config::{
    CHUNK_SIZE, ENEMY_TEXTURE_INDEX, MAP_ID, N_CHUNKS_X, N_CHUNKS_Y, WALLS_LAYER_ID,
};

pub fn add(mut commands: Commands, map_query: MapQuery) {
    let mut rng = rand::thread_rng();
    let random_x = Uniform::from(0..(CHUNK_SIZE * N_CHUNKS_X ));
    let random_y = Uniform::from(0..(CHUNK_SIZE * N_CHUNKS_Y ));

    for _ in 1..10 {
        let x = random_x.sample(&mut rng);
        let y = random_y.sample(&mut rng);

        let position_is_empty = map_query
            .get_tile_entity(UVec2::new(x, y), MAP_ID, WALLS_LAYER_ID)
            .is_err();

        if position_is_empty {
            commands.spawn_bundle((
                Position { x: x, y: y },
                EnemyTag,
                Drawable {
                    texture_index: ENEMY_TEXTURE_INDEX,
                },
                GoingToTakeAction { action: None },
                MoveRandomlyTag,
            ));
        }
    }
}

pub fn choose_random_action(
    mut entities_query: Query<(&Position, &mut GoingToTakeAction), With<MoveRandomlyTag>>,
) {
    let possible_actions = [
        Action::StayStill,
        Action::North,
        Action::South,
        Action::West,
        Action::East,
    ];

    for (_, mut action_to_take) in entities_query.iter_mut() {
        // Choose an action at random
        let action = possible_actions.choose(&mut rand::thread_rng()).unwrap();
        action_to_take.action = Some(*action);
    }
}
