use bevy::prelude::*;

pub mod components;
pub mod config;
pub mod enemy;
pub mod helpers;
pub mod player;
pub mod setup;
pub mod tilemap;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            // We need to add the drawable entities before we draw the tilemap
            .add_startup_system_to_stage(StartupStage::PreStartup, player::add_player.system())
            .add_startup_system_to_stage(StartupStage::PreStartup, enemy::add_enemies.system())
            // Create tilemap
            .add_startup_system(setup::setup.system())
            // Add a camera
            .add_startup_system(helpers::camera::add_camera.system())
            .add_system(player::player_movement.system().label("player_movement"))
            .add_system(enemy::enemy_movement.system().after("player_movement"))
            .add_system(helpers::camera::movement.system())
            .add_system(helpers::texture::set_texture_filters_to_nearest.system())
            .insert_resource(components::PlayerJustMoved(false));
    }
}
