use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::system::exit_on_esc_system,
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;

mod game;

// Systems
fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    App::new()
        .insert_resource(WindowDescriptor {
            width: (1920 / 2) as f32,
            height: (1080 / 2) as f32,
            title: String::from("Roguelike"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // .add_system(exit_on_esc_system)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(TilemapPlugin)
        // .add_plugin(game::GamePlugin)
        .run();
}
