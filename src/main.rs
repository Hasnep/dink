use bevy::{
    asset::LoadState,
    input::system::exit_on_esc_system,
    prelude::*,
    sprite::{TextureAtlas, TextureAtlasBuilder},
    window::WindowMode,
};
use bevy_tilemap::prelude::*;
use std::cmp::{max, min};

const CHUNK_WIDTH: u32 = 8;
const CHUNK_HEIGHT: u32 = 8;
const TILEMAP_WIDTH: u32 = CHUNK_WIDTH * 2;
const TILEMAP_HEIGHT: u32 = CHUNK_HEIGHT * 2;

#[derive(Default, Clone)]
struct SpriteHandles {
    handles: Vec<HandleUntyped>,
}

// #[derive(Default, Clone)]
// struct GameState {
//     map_loaded: bool,
//     spawned: bool,
// }

#[derive(Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug)]
struct Render {
    sprite_index: usize,
    sprite_order: usize,
}

#[derive(Clone, Debug)]
struct Player {}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Roguelike".to_string(),
            width: (1920 / 2) as f32,
            height: (1080 / 2) as f32,
            vsync: false,
            resizable: true,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .init_resource::<SpriteHandles>()
        .add_plugins(DefaultPlugins)
        .add_plugins(TilemapDefaultPlugins)
        .add_startup_system(get_sprite_handles.system())
        .add_startup_system(spawn_player.system())
        .add_startup_system(create_camera.system())
        .add_startup_system(load_textures_and_create_tilemap.system())
        .add_startup_system(add_wall_tiles.system())
        .add_system(player_input.system())
        .add_system(exit_on_esc_system.system())
        .run();
}

fn spawn_player(mut commands: Commands) {
    commands.spawn_bundle((
        Position { x: 2, y: 2 },
        Player {},
        // Render {
        //     sprite_index: dwarf_sprite_index,
        //     sprite_order: 1,
        // },
    ));
}

fn player_input(
    keys: Res<Input<KeyCode>>,
    mut player_position: Query<&mut Position, With<Player>>,
) {
    if keys.just_released(KeyCode::Left) {
        try_to_move_player(-1, 0, &mut player_position);
    }
    if keys.just_released(KeyCode::Right) {
        try_to_move_player(1, 0, &mut player_position);
    }
    if keys.just_released(KeyCode::Up) {
        try_to_move_player(0, -1, &mut player_position);
    }
    if keys.just_released(KeyCode::Down) {
        try_to_move_player(0, 1, &mut player_position);
    }
}

fn try_to_move_player(
    delta_x: i32,
    delta_y: i32,
    player_position: &mut Query<&mut Position, With<Player>>,
) {
    for mut pos in player_position.iter_mut() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
        println!("Player moved to {},{}", pos.x, pos.y);
    }
}

// fn move_sprite(
//     map: &mut Tilemap,
//     previous_position: Position,
//     position: Position,
//     render: &Render,
// ) {
//     map.clear_tile((previous_position.x, previous_position.y), 1)
//         .unwrap();
//     let tile = Tile {
//         point: (position.x, position.y),
//         sprite_index: render.sprite_index,
//         sprite_order: render.sprite_order,
//         ..Default::default()
//     };
//     map.insert_tile(tile).unwrap();
// }

fn get_sprite_handles(mut sprite_handles: ResMut<SpriteHandles>, asset_server: Res<AssetServer>) {
    sprite_handles.handles = asset_server.load_folder("textures").unwrap();
}

fn load_textures_and_create_tilemap(
    mut commands: Commands,
    sprite_handles: Res<SpriteHandles>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    asset_server: Res<AssetServer>,
) {
    // Load textures
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    if let LoadState::Loaded =
        asset_server.get_group_load_state(sprite_handles.handles.iter().map(|handle| handle.id))
    {
        for handle in sprite_handles.handles.iter() {
            let texture = textures.get(handle).unwrap();
            texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), &texture);
        }

        let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
        let atlas_handle = texture_atlases.add(texture_atlas);

        // let mut tilemap = Tilemap::default();
        // tilemap.set_texture_atlas(atlas_handle);

        let tilemap = Tilemap::builder()
            // Square grid
            .topology(GridTopology::Square)
            // Set the shape of the overall tilemap
            .dimensions(TILEMAP_WIDTH, TILEMAP_HEIGHT)
            // Set the shape of each chunk
            .chunk_dimensions(CHUNK_WIDTH, CHUNK_HEIGHT, 1)
            // AUtomatically create chunks
            .auto_chunk()
            // Add layers for tiles and player
            .add_layer(
                TilemapLayer {
                    kind: LayerKind::Dense,
                },
                0,
            )
            .add_layer(
                TilemapLayer {
                    kind: LayerKind::Sparse,
                },
                1,
            )
            // Set the texture properties
            .texture_atlas(atlas_handle)
            .texture_dimensions(32, 32)
            // Finish defining the tilemap
            .finish()
            .unwrap();

        let tilemap_components = TilemapBundle {
            tilemap,
            visible: Visible {
                is_visible: true,
                is_transparent: true,
            },
            transform: Default::default(),
            global_transform: Default::default(),
        };

        commands.spawn().insert_bundle(tilemap_components);
    }
}

fn create_camera(mut commands: Commands) {
    commands
        .spawn()
        .insert_bundle(OrthographicCameraBundle::new_2d());
}

fn add_wall_tiles(
    texture_atlases: Res<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    mut tilemap_query: Query<&mut Tilemap>,
) {
    println!("Adding the walls to the tilemap.");
    for mut map in tilemap_query.iter_mut() {
        println!("Getting the wall texture.");

        // Get wall texture
        let texture_atlas = texture_atlases.get(map.texture_atlas()).unwrap();
        let wall_texture: Handle<Texture> = asset_server.get_handle("textures/square-wall.png");
        let wall_texture_index = texture_atlas.get_texture_index(&wall_texture).unwrap();

        let chunk_width = (map.width().unwrap() * map.chunk_width()) as i32;
        let chunk_height = (map.height().unwrap() * map.chunk_height()) as i32;

        println!("{} by {}", chunk_width, chunk_height);

        // let mut tiles_to_add = Vec::new();
        for x in 1..100 {
            for y in 1..100 {
                // let x = x - chunk_width / 2;
                // let y = y - chunk_height / 2;

                let tile = Tile {
                    point: (x, y),
                    sprite_index: wall_texture_index,
                    ..Default::default()
                };
                // tiles_to_add.push(tile);
                map.insert_tile(tile).unwrap();
            }
        }
        // map.insert_tiles(tiles_to_add).unwrap();
    }
}
