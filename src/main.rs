use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    input::system::exit_on_esc_system,
    prelude::*,
};
use bevy_ecs_tilemap::prelude::*;
use noise::{Fbm, NoiseFn};
use rand::seq::SliceRandom;

mod helpers;

// Tilemap
const MAP_ID: u16 = 0;
const TILES_LAYER_ID: u16 = 0;
const CHUNK_SIZE: u32 = 8;
const TILE_SIZE: f32 = 32 as f32;
const N_CHUNKS_X: u32 = 10;
const N_CHUNKS_Y: u32 = 10;

// Texture indices
const N_TEXTURES: i32 = 3;
const WALL_TEXTURE_INDEX: u16 = 0;
const PLAYER_TEXTURE_INDEX: u16 = 1;
const ENEMY_TEXTURE_INDEX: u16 = 2;

// Components
#[derive(Clone, Debug, Copy)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Clone, Debug)]
struct Player {}

#[derive(Clone, Debug)]
struct Enemy {}

#[derive(Clone, Debug)]
struct Drawable {
    texture_index: u16,
}

#[derive(Default)]
struct PlayerJustMoved(bool);

// Systems
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
        .add_system(exit_on_esc_system.system())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(TilemapPlugin)
        // We need to add the drawable entities before we draw the tilemap
        .add_startup_system_to_stage(StartupStage::PreStartup, add_player.system())
        .add_startup_system_to_stage(StartupStage::PreStartup, add_enemy.system())
        // Create tilemap
        .add_startup_system(setup.system())
        // Add a camera
        .add_startup_system(add_camera.system())
        .add_system(player_movement.system().label("player_movement"))
        .add_system(enemy_movement.system().after("player_movement"))
        .add_system(helpers::camera::movement.system())
        .add_system(helpers::texture::set_texture_filters_to_nearest.system())
        .insert_resource(PlayerJustMoved(false))
        .run();
}

fn add_player(mut commands: Commands) {
    commands.spawn_bundle((
        Position { x: 2, y: 4 },
        Player {},
        Drawable {
            texture_index: PLAYER_TEXTURE_INDEX,
        },
    ));
}

fn add_enemy(mut commands: Commands) {
    commands.spawn_bundle((
        Position { x: 4, y: 4 },
        Enemy {},
        Drawable {
            texture_index: ENEMY_TEXTURE_INDEX,
        },
    ));
}

fn try_to_move_player(
    delta_x: i32,
    delta_y: i32,
    mut player_query: Query<&mut Position, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(Entity, &Position), (With<Enemy>, Without<Player>)>,
    commands: &mut Commands,
    map_query: &mut MapQuery,
) {
    for mut player_position in player_query.iter_mut() {
        let from = *player_position;
        let to = Position {
            x: ((from.x as i32) + delta_x) as u32,
            y: ((from.y as i32) + delta_y) as u32,
        };

        let to_tile = map_query.get_tile_entity(UVec2::new(to.x, to.y), MAP_ID, TILES_LAYER_ID);

        if to_tile.is_ok() {
            for (enemy_id, enemy_position) in enemy_query.iter() {
                // If there is an enemy there
                if enemy_position.x == to.x && enemy_position.y == to.y {
                    // The player kills the enemy
                    commands.entity(enemy_id).despawn();
                    break;
                }
            }
            // If that space has a tile then the player digs that tile
            destroy_tile(to, commands, map_query);
        } else {
            // If that space is empty then move the player
            player_position.x = to.x;
            player_position.y = to.y;

            move_tile(from, to, commands, map_query, PLAYER_TEXTURE_INDEX);
        }
    }
}

fn destroy_tile(tile_position: Position, commands: &mut Commands, map_query: &mut MapQuery) {
    let tile_position = UVec2::new(tile_position.x, tile_position.y);
    let _ = map_query
        .despawn_tile(commands, tile_position, MAP_ID, TILES_LAYER_ID)
        .expect("Oh no something went wrong with de-spawning a tile!");
    map_query.notify_chunk_for_tile(tile_position, MAP_ID, TILES_LAYER_ID);
}

fn move_tile(
    from: Position,
    to: Position,
    commands: &mut Commands,
    map_query: &mut MapQuery,
    texture_index: u16,
) {
    let from = UVec2::new(from.x, from.y);
    let to = UVec2::new(to.x, to.y);
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
                texture_index: texture_index,
                ..Default::default()
            },
            MAP_ID,
            TILES_LAYER_ID,
        )
        .expect("Couldn't set the new tile!");
    map_query.notify_chunk_for_tile(from, MAP_ID, TILES_LAYER_ID);
    map_query.notify_chunk_for_tile(to, MAP_ID, TILES_LAYER_ID);
}

fn is_in_bounds(position: &IVec2) -> bool {
    return position.x >= 0
        && position.y >= 0
        && position.x <= (CHUNK_SIZE * N_CHUNKS_X) as i32
        && position.y <= (CHUNK_SIZE * N_CHUNKS_Y) as i32;
}

fn enemy_movement(
    mut enemy_query: Query<&mut Position, With<Enemy>>,
    mut commands: Commands,
    mut map_query: MapQuery,
    player_just_moved: ResMut<PlayerJustMoved>,
) {
    if player_just_moved.0 {
        for mut enemy_position in enemy_query.iter_mut() {
            // Get spaces next to the enemy
            // let neighbours: Vec<&(IVec2, Option<Entity>)> =

            let neighbours = map_query.get_tile_neighbors(
                UVec2::new(enemy_position.x, enemy_position.y),
                MAP_ID,
                TILES_LAYER_ID,
            );

            let neighbours = neighbours
                .iter()
                // Only the neighbours in the cardinal directions
                .take(4)
                // Check the space is empty
                .filter(|neighbour| neighbour.1.is_none())
                // Check the space is in-bounds
                .filter(|neighbour| is_in_bounds(&neighbour.0))
                .collect::<Vec<&(IVec2, Option<Entity>)>>();

            let to_position_and_tile = neighbours.choose(&mut rand::thread_rng());

            match to_position_and_tile {
                Some(to_position_and_tile) => {
                    let from = *enemy_position;
                    let to = to_position_and_tile.0;

                    let to = Position {
                        x: to.x as u32,
                        y: to.y as u32,
                    };

                    // Move the enemy
                    enemy_position.x = to.x;
                    enemy_position.y = to.y;
                    // Move the enemy's sprite
                    move_tile(from, to, &mut commands, &mut map_query, ENEMY_TEXTURE_INDEX)
                }
                None => {}
            }
        }
        commands.insert_resource(PlayerJustMoved(false));
    }
}

fn player_movement(
    keys: Res<Input<KeyCode>>,
    player_query: Query<&mut Position, (With<Player>, Without<Enemy>)>,
    enemy_query: Query<(Entity, &Position), (With<Enemy>, Without<Player>)>,
    mut commands: Commands,
    mut map_query: MapQuery,
) {
    let mut player_moved = false;
    let mut direction = IVec2::new(0, 0);
    // player_position_query
    if keys.just_released(KeyCode::Left) {
        direction = IVec2::new(-1, 0);
        player_moved = true;
    } else if keys.just_released(KeyCode::Right) {
        direction = IVec2::new(1, 0);
        player_moved = true;
    } else if keys.just_released(KeyCode::Up) {
        direction = IVec2::new(0, 1);
        player_moved = true;
    } else if keys.just_released(KeyCode::Down) {
        direction = IVec2::new(0, -1);
        player_moved = true;
    }
    if player_moved {
        try_to_move_player(
            direction.x,
            direction.y,
            player_query,
            enemy_query,
            &mut commands,
            &mut map_query,
        );
        commands.insert_resource(PlayerJustMoved(true))
    };
}

fn add_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
    drawable_query: Query<(&Position, &Drawable)>,
) {
    // Load textures
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
            Vec2::new((N_TEXTURES as f32) * TILE_SIZE, TILE_SIZE),
        ),
        MAP_ID,
        TILES_LAYER_ID,
    );

    // Construct a noise generator
    let mut noise = Fbm::new();
    noise.frequency = 0.1;

    // Use noise function to set walls
    for i in 0..(N_CHUNKS_X * CHUNK_SIZE) {
        for j in 0..(N_CHUNKS_Y * CHUNK_SIZE) {
            let noise_value = noise.get([i as f64, j as f64]);
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

    // Draw entities that have a sprite associated with them
    for (position, drawable) in drawable_query.iter() {
        let position = UVec2::new(position.x, position.y);
        let tile = Tile {
            texture_index: drawable.texture_index,
            ..Default::default()
        };
        let _ = layer_builder
            .set_tile(position, tile.into())
            .expect("Couldn't set tile! :(");
    }

    // Build the layer
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, material_handle);

    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, TILES_LAYER_ID, layer_entity);

    // Create map
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
        .insert(GlobalTransform::default());
}
