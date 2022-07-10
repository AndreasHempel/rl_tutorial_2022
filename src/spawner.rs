use bevy::prelude::*;

use crate::{
    components::{Actor, Monster, Player, Position},
    map::{GameMap, MapMetadata, TileType},
    render::TILE_SIZE,
};

/// Bundles spawning functions into a single plugin
#[derive(Debug)]
pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player)
            .add_startup_system(spawn_monster)
            .add_startup_system(spawn_tiles);
    }
}

/// Size of the sprite assets
const SPRITE_SIZE: f32 = 16.0;

/// Spawn the player entity with associated components
fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    map_metadata: Res<MapMetadata>,
) {
    let texture_handle = asset_server.load("Dawnlike/Characters/Pest0.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 8, 11);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let mut sprite = TextureAtlasSprite::new(59);
    sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));

    let start = map_metadata.starting_position.unwrap();
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            sprite,
            ..default()
        })
        .insert(Player)
        .insert(Position::new(start.0, start.1))
        .insert(Actor::default());
}

/// Spawn a monster into the world
fn spawn_monster(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("Dawnlike/Characters/Demon0.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 8, 9);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let mut sprite = TextureAtlasSprite::new(3);
    sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            sprite,
            ..default()
        })
        .insert(Monster)
        .insert(Position::new(30, 48))
        .insert(Actor::default());
}

/// Spawns an entity for each tile in the map and attaches the corresponding sprite
/// TODO: Adding an entity per map tile (4000+) leads to a significant FPS drop in Debug mode with the `WorldInspectorPlugin`
///         -> Determine if this can be improved with some trickery, is a fundamental limitation, or can be resolved by using a tile-map plugin
fn spawn_tiles(
    map: Res<GameMap>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let (floor_sprite, floor_atlas) = {
        let floor_handle = asset_server.load("Dawnlike/Objects/Floor.png");
        let floor_atlas =
            TextureAtlas::from_grid(floor_handle, Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 21, 39);
        let floor_atlas_handle = texture_atlases.add(floor_atlas);
        let mut sprite = TextureAtlasSprite::new(85);
        sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
        (sprite, floor_atlas_handle)
    };
    let (wall_sprite, wall_atlas) = {
        let wall_handle = asset_server.load("Dawnlike/Objects/Wall.png");
        let wall_atlas =
            TextureAtlas::from_grid(wall_handle, Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 20, 51);
        let wall_atlas_handle = texture_atlases.add(wall_atlas);
        let mut sprite = TextureAtlasSprite::new(63);
        sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
        (sprite, wall_atlas_handle)
    };
    for (idx, tile) in map.tiles.iter().enumerate() {
        let (x, y) = map.idx_to_xy(idx).unwrap();
        match tile {
            TileType::Floor => commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: floor_atlas.clone(),
                    transform: Transform::from_scale(Vec3::splat(1.0)),
                    sprite: floor_sprite.clone(),
                    ..default()
                })
                .insert(Position::new(x, y))
                .insert(tile.clone()),
            TileType::Wall => commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: wall_atlas.clone(),
                    transform: Transform::from_scale(Vec3::splat(1.0)),
                    sprite: wall_sprite.clone(),
                    ..default()
                })
                .insert(Position::new(x, y))
                .insert(tile.clone()),
        };
    }
}
