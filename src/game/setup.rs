use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use noise::{Fbm, NoiseFn};

use crate::game::components::{Drawable, Position};
use crate::game::config::*;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut map_query: MapQuery,
    drawable_query: Query<(&Position, &Drawable)>,
) {
    // Load textures
    let texture_handle = asset_server.load("textures/textures.png");
    // let material_handle = materials.add(ColorMaterial::texture(texture_handle));

    // Create map entity and component:
    let map_entity = commands.spawn().id();
    let mut map = Map::new(MAP_ID, map_entity);

    // Creates a new layer builder with a layer entity.
    let (mut layer_builder, _) = LayerBuilder::<TileBundle>::new(
        &mut commands,
        LayerSettings::new(
            MapSize(N_CHUNKS_X, N_CHUNKS_Y), // UVec2::new(N_CHUNKS_X, N_CHUNKS_Y),
            ChunkSize(CHUNK_SIZE, CHUNK_SIZE),
            TileSize(TILE_SIZE, TILE_SIZE),
            TextureSize((N_TEXTURES as f32) * TILE_SIZE, TILE_SIZE),
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
                    .set_tile(TilePos(i, j), tile.into())
                    .expect("Couldn't set tile! :(");
            }
        }
    }

    // Draw entities that have a sprite associated with them
    for (position, drawable) in drawable_query.iter() {
        let position = TilePos(position.x, position.y);
        let tile = Tile {
            texture_index: drawable.texture_index,
            ..Default::default()
        };
        let _ = layer_builder
            .set_tile(position, tile.into())
            .expect("Couldn't set tile! :(");
    }

    // Build the layer
    let layer_entity = map_query.build_layer(&mut commands, layer_builder, texture_handle);

    // Required to keep track of layers for a map internally.
    map.add_layer(&mut commands, TILES_LAYER_ID, layer_entity);

    // Create map
    commands
        .entity(map_entity)
        .insert(map)
        .insert(Transform::from_xyz(-128.0, -128.0, 0.0))
        .insert(GlobalTransform::default());
}
