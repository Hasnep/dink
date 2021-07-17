use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;
use noise::{Fbm, NoiseFn};

const MAP_ID: u16 = 0;
const TILES_LAYER_ID: u16 = 0;
const WALL_TEXTURE_INDEX: u16 = 0;
const PLAYER_TEXTURE_INDEX: u16 = 1;
const CHUNK_SIZE: u32 = 8;
const TILE_SIZE: f32 = 32 as f32;
const N_CHUNKS_X: u32 = 5;
const N_CHUNKS_Y: u32 = 5;

mod camera;
mod texture;

#[derive(Clone, Debug, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Player {}

fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    App::build()
        .insert_resource(WindowDescriptor {
            width: (1920 / 2) as f32,
            height: (1080 / 2) as f32,
            title: String::from("Roguelike"),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(TilemapPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(add_camera.system())
        .add_startup_system(add_player.system())
        .add_system(player_movement.system())
        .add_system(camera::movement.system())
        .add_system(texture::set_texture_filters_to_nearest.system())
        .run();
}

fn add_player(mut commands: Commands) {
    commands.spawn_bundle((
        Position { x: 2, y: 2 },
        Player {},
        // Render {
        //     sprite_index: dwarf_sprite_index,
        //     sprite_order: 1,
        // },
    ));
}

fn try_to_move_player(
    delta_x: i32,
    delta_y: i32,
    player_position: &mut Query<&mut Position, With<Player>>,
    commands: &mut Commands,
    map_query: &mut MapQuery,
) {
    for mut pos in player_position.iter_mut() {
        let from = *pos;
        pos.x = pos.x + delta_x;
        pos.y = pos.y + delta_y;
        move_tile(from, *pos, commands, map_query)
    }
}

fn move_tile(from: Position, to: Position, commands: &mut Commands, map_query: &mut MapQuery) {
    let from = UVec2::new(from.x as u32, from.y as u32);
    let to = UVec2::new(to.x as u32, to.y as u32);
    let _ = map_query
        .get_tile_entity(from, MAP_ID, TILES_LAYER_ID)
        .expect("Tried to move a tile that doesn't exist!");
    let _ = map_query
        .despawn_tile(commands, from, MAP_ID, TILES_LAYER_ID)
        .expect("Oh no something went wrong with de-spawning a tile!");
    let _ = map_query
        .set_tile(
            commands,
            to,
            Tile {
                texture_index: PLAYER_TEXTURE_INDEX,
                ..Default::default()
            },
            MAP_ID,
            TILES_LAYER_ID,
        )
        .expect("Couldn't set the new tile!");
    map_query.notify_chunk_for_tile(from, MAP_ID, TILES_LAYER_ID);
    map_query.notify_chunk_for_tile(to, MAP_ID, TILES_LAYER_ID);
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    mut player_position_query: Query<&mut Position, With<Player>>,
    mut commands: Commands,
    mut map_query: MapQuery,
) {
    // player_position_query
    if keys.just_released(KeyCode::Left) {
        try_to_move_player(
            -1,
            0,
            &mut player_position_query,
            &mut commands,
            &mut map_query,
        );
    } else if keys.just_released(KeyCode::Right) {
        try_to_move_player(
            1,
            0,
            &mut player_position_query,
            &mut commands,
            &mut map_query,
        );
    } else if keys.just_released(KeyCode::Up) {
        try_to_move_player(
            0,
            1,
            &mut player_position_query,
            &mut commands,
            &mut map_query,
        );
    } else if keys.just_released(KeyCode::Down) {
        try_to_move_player(
            0,
            -1,
            &mut player_position_query,
            &mut commands,
            &mut map_query,
        );
    }
}

fn add_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    let texture_handle = asset_server.load("textures/textures.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(MAP_ID, map_entity);

    // Creates a new layer builder with a layer entity.
    let (mut layer_builder, _) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        LayerSettings::new(
            UVec2::new(N_CHUNKS_X, N_CHUNKS_Y),
            UVec2::new(CHUNK_SIZE, CHUNK_SIZE),
            Vec2::new(TILE_SIZE, TILE_SIZE),
            Vec2::new((2 as f32) * TILE_SIZE, TILE_SIZE),
        ),
        MAP_ID,
        TILES_LAYER_ID,
    );
    // map.add_layer(&mut commands, 0u16, layer_entity);

    // // Fill the map with walls
    // layer_builder.set_all(TileBundle::default());

    // Construct a noise generator
    let mut noise = Fbm::new();
    noise.frequency = 0.15;

    for i in 0..(N_CHUNKS_X * CHUNK_SIZE) {
        for j in 0..(N_CHUNKS_Y * CHUNK_SIZE) {
            let noise_value = noise.get([i as f64, j as f64]);
            println!("Noise at {},{} is {}", i, j, noise_value);
            if noise_value > 0.0 {
                let tile = Tile {
                    texture_index: WALL_TEXTURE_INDEX,
                    ..Default::default()
                };
                let _ = layer_builder
                    .set_tile(UVec2::new(i, j), tile.into())
                    .expect("Couldn't set tile! :(");
            }
        }
    }
    // layer_builder.set_tile(UVec2::new(1,1),  Tile {texture_index: WALL_TEXTURE_INDEX,..Default::default()},);

    // Builds the layer.
    // Note: Once this is called you can no longer edit the layer until a hard sync in bevy.
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, material_handle);

    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, TILES_LAYER_ID, layer_entity);

    // Spawn Map
    // Required in order to use map_query to retrieve layers/tiles.
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
        .insert(GlobalTransform::default());
}
