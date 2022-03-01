use bevy::prelude::*;

pub mod components;
pub mod config;
pub mod enemy;
pub mod helpers;
pub mod player;
pub mod setup;
pub mod states;
pub mod tilemap;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // We need to add the drawable entities before we draw the tilemap
            .add_startup_system_to_stage(StartupStage::PreStartup, player::add)
            .add_startup_system_to_stage(StartupStage::PreStartup, enemy::add)
            // Create tilemap
            .add_startup_system(setup::setup)
            // Add a camera
            .add_startup_system(helpers::camera::add_camera)
            // Set the initial game state
            .add_state(states::GameState::PlayerTurn)
            .add_system_set(
                SystemSet::on_update(states::GameState::PlayerTurn).with_system(player::movement),
            )
            .add_system_set(
                SystemSet::on_update(states::GameState::EnemyTurn).with_system(enemy::movement),
            )
            .add_system(helpers::camera::movement);
    }
}
