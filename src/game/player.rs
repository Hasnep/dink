use bevy::prelude::*;

use crate::game::components::{Action, Drawable, GoingToTakeAction, PlayerTag, Position};
use crate::game::config::PLAYER_TEXTURE_INDEX;

pub fn add(mut commands: Commands) {
    commands.spawn_bundle((
        PlayerTag,
        Position { x: 10, y: 10 },
        Drawable {
            texture_index: PLAYER_TEXTURE_INDEX,
        },
        GoingToTakeAction { action: None },
    ));
}

pub fn decide_action(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<&mut GoingToTakeAction, With<PlayerTag>>,
) {
    for key in keys.get_just_released() {
        let chosen_action = match key {
            KeyCode::Left => Some(Action::West),
            KeyCode::Right => Some(Action::East),
            KeyCode::Up => Some(Action::North),
            KeyCode::Down => Some(Action::South),
            _ => None,
        };

        if chosen_action.is_some() {
            let mut player_action = player_query
                .single_mut()
                .expect("There should be exactly one player in the game.");
            player_action.action = chosen_action;
        }
    }
}
