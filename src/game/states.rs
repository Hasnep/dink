use bevy::prelude::*;

use crate::game::components::{GoingToTakeAction, HaveUpdatedTilemap, PlayerTag};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    PlayerTurn,
    EnemyTurn,
    TakeAction,
    UpdateTilemap,
}

pub fn end_player_turn(
    player_query: Query<&GoingToTakeAction, With<PlayerTag>>,
    mut game_state: ResMut<State<GameState>>,
) {
    let has_player_decided = player_query
        .iter()
        .all(|decision| decision.action.is_some());

    if has_player_decided {
        // println!("Transitioning from PlayerTurn to EnemyTurn.");
        game_state.replace(GameState::EnemyTurn).unwrap();
    }
}

pub fn end_enemy_turn(
    nonplayer_entities_query: Query<&GoingToTakeAction, Without<PlayerTag>>,
    mut game_state: ResMut<State<GameState>>,
) {
    let have_all_entities_decided = nonplayer_entities_query
        .iter()
        .all(|decision| decision.action.is_some());

    if have_all_entities_decided {
        // println!("Transitioning from EnemyTurn to TakeAction.");
        game_state.replace(GameState::TakeAction).unwrap();
    }
}

pub fn end_action_state(
    moving_entities_query: Query<&GoingToTakeAction>,
    mut game_state: ResMut<State<GameState>>,
) {
    let have_all_entities_moved = moving_entities_query
        .iter()
        .all(|decision| decision.action.is_none());

    if have_all_entities_moved {
        // println!("Transitioning from TakeAction to UpdateTilemap.");
        game_state.replace(GameState::UpdateTilemap).unwrap();
    }
}

pub fn end_update_tilemap_state(
    have_updated_tilemap_query: Query<&HaveUpdatedTilemap>,
    mut game_state: ResMut<State<GameState>>,
) {
    let have_updated_tilemap = have_updated_tilemap_query
        .single()
        .expect("Cannot have more than one HaveUpdatedTilemap entity.")
        .0;

    if have_updated_tilemap {
        // println!("Transitioning from TakeAction to UpdateTilemap.");
        game_state.replace(GameState::PlayerTurn).unwrap();
    }
}
