use bevy::prelude::*;

pub mod components;
pub mod config;
pub mod enemy;
pub mod helpers;
pub mod movement;
pub mod player;
pub mod setup;
pub mod states;
pub mod tilemap;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // Create tilemap
            .add_startup_system(setup::setup.system().label("worldgen"))
            .add_system(helpers::texture::set_texture_filters_to_nearest.system())
            // Add a camera
            .add_startup_system(helpers::camera::add_camera.system())
            .add_system(helpers::camera::movement.system())
            // Add initial objects
            .add_startup_system(player::add.system().after("worldgen"))
            .add_startup_system(enemy::add.system().after("worldgen"))
            // Set the initial game state
            .add_state(states::GameState::UpdateTilemap)
            // When it's the player's turn
            .add_system_set(
                SystemSet::on_update(states::GameState::PlayerTurn)
                    // Accept player input
                    .with_system(player::decide_action.system())
                    // End the player's turn
                    .with_system(states::end_player_turn.system()),
            )
            // When it's the AI's turn
            .add_system_set(
                SystemSet::on_update(states::GameState::EnemyTurn)
                    // Choose the AI's action
                    .with_system(enemy::choose_random_action.system())
                    // End the enemy's turn
                    .with_system(states::end_enemy_turn.system()),
            )
            // When it's time to resolve the entities' chosen actions
            .add_system_set(
                SystemSet::on_update(states::GameState::TakeAction)
                    // All the entities take their actions
                    .with_system(movement::take_action.system())
                    // End taking actions
                    .with_system(states::end_action_state.system()),
            )
            // When it's time to update the tilemap
            .add_system_set(
                SystemSet::on_update(states::GameState::UpdateTilemap)
                    // Update the tilemap
                    .with_system(tilemap::update_tilemap.system())
                    // Go back to the player's turn
                    .with_system(states::end_update_tilemap_state.system()),
            );
    }
}
