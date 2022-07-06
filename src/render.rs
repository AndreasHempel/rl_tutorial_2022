use bevy::prelude::*;

use crate::components::Position;

/// Bundles systems responsible for rendering
#[derive(Debug)]
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_system(place_characters);
    }
}

/// Size of each tile for rendering
pub const TILE_SIZE: f32 = 32.0;

/// Spawn a camera for rendering
fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

/// Updates the screen coordinates ([Transform]) based on an entity's [Position]
fn place_characters(mut chars: Query<(&Position, &mut Transform)>) {
    for (p, mut t) in chars.iter_mut() {
        t.translation.x = (p.x as f32) * TILE_SIZE;
        t.translation.y = (p.y as f32) * TILE_SIZE;
    }
}
