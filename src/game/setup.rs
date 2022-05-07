use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noise::{utils::*, Fbm};

use crate::game::components::HaveUpdatedTilemap;
use crate::game::config::{
    CHUNK_SIZE, MAP_ID, N_CHUNKS_X, N_CHUNKS_Y, N_TEXTURES, OBJECTS_LAYER_ID, TILE_SIZE,
    WALLS_LAYER_ID, WALL_TEXTURE_INDEX, WORLDGEN_SCALE,
};

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut map_query: MapQuery,
) {
    // Load textures
    let texture_handle = asset_server.load("textures/textures.png");
    let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(MAP_ID, map_entity);

    // Define the layer settings for each layer
    let default_layer_settings = LayerSettings::new(
        UVec2::new(N_CHUNKS_X, N_CHUNKS_Y),
        UVec2::new(CHUNK_SIZE, CHUNK_SIZE),
        Vec2::new(TILE_SIZE, TILE_SIZE),
        Vec2::new((N_TEXTURES as f32) * TILE_SIZE, TILE_SIZE),
    );

    // Create the walls layer
    let (mut walls_layer_builder, walls_layer_entity) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        default_layer_settings.clone(),
        MAP_ID,
        WALLS_LAYER_ID,
    );
    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, WALLS_LAYER_ID, walls_layer_entity);

    // World generation
    {
        // Construct a noise generator
        let noise_map = PlaneMapBuilder::new(&Fbm::new())
            .set_size(
                (N_CHUNKS_X * CHUNK_SIZE) as usize,
                (N_CHUNKS_Y * CHUNK_SIZE) as usize,
            )
            .set_x_bounds(
                -(N_CHUNKS_X as f64) / WORLDGEN_SCALE,
                N_CHUNKS_X as f64 / WORLDGEN_SCALE,
            )
            .set_y_bounds(
                -(N_CHUNKS_Y as f64) / WORLDGEN_SCALE,
                N_CHUNKS_Y as f64 / WORLDGEN_SCALE,
            )
            .build();

        // Use noise function to set walls
        for i in 0..(N_CHUNKS_X * CHUNK_SIZE) {
            for j in 0..(N_CHUNKS_Y * CHUNK_SIZE) {
                let noise_value = noise_map.get_value(i as usize, j as usize);
                if noise_value > 0.0 {
                    let tile = Tile {
                        texture_index: WALL_TEXTURE_INDEX,
                        ..Default::default()
                    };
                    let _ = walls_layer_builder
                        .set_tile(UVec2::new(i, j), tile.into())
                        .expect("Couldn't set tile! :(");
                }
            }
        }
    }

    // Build the walls layer
    let _ = map_query.build_layer(&mut commands, walls_layer_builder, material_handle.clone());

    // Create the objects layer
    let (objects_layer_builder, objects_layer_entity) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        default_layer_settings,
        MAP_ID,
        OBJECTS_LAYER_ID,
    );
    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, OBJECTS_LAYER_ID, objects_layer_entity);
    // Build the objects layer
    let _ = map_query.build_layer(&mut commands, objects_layer_builder, material_handle);

    // Create map
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
        .insert(GlobalTransform::default());

    // Add entity to monitor whether we have updated the tilemap
    commands.spawn().insert(HaveUpdatedTilemap(false));
}
