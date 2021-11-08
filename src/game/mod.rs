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
    fn build(&self, app: &mut AppBuilder) {
        app
            // We need to add the drawable entities before we draw the tilemap
            .add_startup_system_to_stage(StartupStage::PreStartup, player::add.system())
            .add_startup_system_to_stage(StartupStage::PreStartup, enemy::add.system())
            // Create tilemap
            .add_startup_system(setup::setup.system())
            // Add a camera
            .add_startup_system(helpers::camera::add_camera.system())
            // Set the initial game state
            .add_state(states::GameState::PlayerTurn)
            .add_system_set(
                SystemSet::on_update(states::GameState::PlayerTurn)
                    .with_system(player::movement.system()),
            )
            .add_system_set(
                SystemSet::on_update(states::GameState::EnemyTurn)
                    .with_system(enemy::movement.system()),
            )
            .add_system(helpers::camera::movement.system())
            .add_system(helpers::texture::set_texture_filters_to_nearest.system());
    }
}
