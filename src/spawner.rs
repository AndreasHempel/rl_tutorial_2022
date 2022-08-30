use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::{
    components::{Actor, BlocksMovement, LevelGoal, Monster, Name, Position, Pushable, Viewshed},
    map::{GameMap, TileType},
    map_builder::{spawner::Spawnables, MapMetadata},
    player::Player,
    render::{TILE_SIZE, ZBUF_CREATURES, ZBUF_ITEMS, ZBUF_PLAYER, ZBUF_TILES},
    GameState,
};

/// Bundles spawning functions into a single plugin
#[derive(Debug)]
pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_player)
            .add_exit_system(GameState::EnterNewLevel, spawn_tiles)
            .add_exit_system(GameState::EnterNewLevel, spawn_things);
    }
}

/// Size of the sprite assets
const SPRITE_SIZE: f32 = 16.0;

/// Spawn the player entity with associated components
fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("Dawnlike/Characters/Pest0.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(SPRITE_SIZE, SPRITE_SIZE), 8, 11);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let mut sprite = TextureAtlasSprite::new(59);
    sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(Vec3::Z * ZBUF_PLAYER),
            sprite,
            ..default()
        })
        .insert(Player::new(100))
        .insert(Name("Player".to_string()))
        // The player position will be set upon map generation based on the starting position
        .insert(Viewshed::new(7))
        .insert(Actor::default())
        .insert(BlocksMovement);
}

/// Spawns everything from the generated [`SpawnList`](crate::map_builder::SpawnList)
fn spawn_things(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    map_metadata: Res<MapMetadata>,
    player: Query<Entity, With<Player>>,
) {
    for ((x, y), s) in map_metadata.spawn_list.iter() {
        use Spawnables::*;
        match s {
            TreasureChest => treasure(
                Position::new(*x, *y),
                &mut commands,
                asset_server.as_ref(),
                texture_atlases.as_mut(),
            ),
            Turtle => turtle(
                Position::new(*x, *y),
                &mut commands,
                asset_server.as_ref(),
                texture_atlases.as_mut(),
            ),
        }
    }

    // Set the player position to match the generated starting position
    if let Some(start) = map_metadata.starting_position {
        let e = player.single();
        commands.entity(e).insert(Position::new(start.0, start.1));
    } else {
        warn!("No starting position for the player available!");
    }
}

fn treasure(
    pos: Position,
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: get_texture_atlas_handle(
                "Dawnlike/Items/Chest0.png",
                8,
                3,
                asset_server,
                texture_atlases,
            ),
            transform: Transform::from_translation(Vec3::Z * ZBUF_ITEMS),
            sprite: get_sprite(1),
            ..default()
        })
        .insert(pos)
        .insert(LevelGoal);
}

fn turtle(
    pos: Position,
    commands: &mut Commands,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: get_texture_atlas_handle(
                "Dawnlike/Characters/Reptile0.png",
                8,
                13,
                asset_server,
                texture_atlases,
            ),
            transform: Transform::from_translation(Vec3::Z * ZBUF_CREATURES),
            sprite: get_sprite(88),
            ..default()
        })
        .insert(pos)
        .insert(Monster)
        .insert(pos)
        .insert(Viewshed::new(7))
        .insert(Actor::default())
        .insert(BlocksMovement)
        .insert(Pushable)
        .insert(Name("Turtle".to_string()));
}

/// Load the specified spritesheet at return a handle to the resulting [`TextureAtlas`]
fn get_texture_atlas_handle(
    spritesheet_path: &str,
    columns: usize,
    rows: usize,
    asset_server: &AssetServer,
    texture_atlases: &mut Assets<TextureAtlas>,
) -> Handle<TextureAtlas> {
    let texture_handle = asset_server.load(spritesheet_path);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(SPRITE_SIZE, SPRITE_SIZE),
        columns,
        rows,
    );
    texture_atlases.add(texture_atlas)
}

/// Build a properly sized [`TextureAtlasSprite`] with the given index
fn get_sprite(index: usize) -> TextureAtlasSprite {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
    sprite
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
        let mut sprite = TextureAtlasSprite::new(243);
        sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
        (sprite, wall_atlas_handle)
    };
    for (idx, tile) in map.tiles.iter().enumerate() {
        let (x, y) = map.idx_to_xy(idx).unwrap();
        match tile {
            TileType::Floor => commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: floor_atlas.clone(),
                    transform: Transform::from_translation(Vec3::Z * ZBUF_TILES),
                    sprite: floor_sprite.clone(),
                    ..default()
                })
                .insert(Position::new(x, y))
                .insert(*tile),
            TileType::Wall => commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: wall_atlas.clone(),
                    transform: Transform::from_translation(Vec3::Z * ZBUF_TILES),
                    sprite: wall_sprite.clone(),
                    ..default()
                })
                .insert(Position::new(x, y))
                .insert(*tile),
        };
    }
}
