use bevy::prelude::*;

use crate::{
    actions::Actor,
    components::{Monster, Player, Position},
    render::TILE_SIZE,
};

/// Spawn the player entity with associated components
pub fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("Dawnlike/Characters/Pest0.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 8, 11);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let mut sprite = TextureAtlasSprite::new(59);
    sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            sprite: sprite,
            ..default()
        })
        .insert(Player)
        .insert(Position::new(10, 10))
        .insert(Actor::default());
}

/// Spawn a monster into the world
pub fn spawn_monster(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("Dawnlike/Characters/Demon0.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(16.0, 16.0), 8, 9);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let mut sprite = TextureAtlasSprite::new(3);
    sprite.custom_size = Some(Vec2::new(TILE_SIZE, TILE_SIZE));
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(1.0)),
            sprite: sprite,
            ..default()
        })
        .insert(Monster)
        .insert(Position::new(0, 0))
        .insert(Actor::default());
}
