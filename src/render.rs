use bevy::{prelude::*, render::camera::Camera2d};

use crate::{
    components::{Player, Position, Viewshed},
    map::{GameMap, TileType},
};

/// Bundles systems responsible for rendering
#[derive(Debug)]
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_cameras)
            .add_startup_system(setup_ui)
            .add_system(place_characters)
            .add_system(player_follow_camera)
            .add_system(render_map)
            .add_system(render_creatures);
    }
}

/// Size of each tile for rendering
pub const TILE_SIZE: f32 = 32.0;

/// Z-buffer plane for player entities
pub const ZBUF_PLAYER: f32 = 10.0;

/// Z-buffer plane for moving entities (creatures...)
pub const ZBUF_CREATURES: f32 = 5.0;

/// Z-buffer plane for static entities (items...)
pub const ZBUF_ITEMS: f32 = 1.0;

/// Z-buffer plane for map tiles
pub const ZBUF_TILES: f32 = 0.0;

/// Spawn a camera for rendering and one for the UI
fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}

/// Updates the screen coordinates ([`Transform`]) based on an entity's [`Position`]
fn place_characters(mut chars: Query<(&Position, &mut Transform), Changed<Position>>) {
    for (p, mut t) in chars.iter_mut() {
        t.translation.x = (p.x as f32) * TILE_SIZE;
        t.translation.y = (p.y as f32) * TILE_SIZE;
    }
}

/// Updates the camera [`Transform`] to always point at the [`Player`]
fn player_follow_camera(
    player: Query<&Position, With<Player>>,
    mut camera: Query<&mut Transform, With<Camera2d>>,
) {
    let p_pos = player.single();
    for mut t in camera.iter_mut() {
        t.translation.x = (p_pos.x as f32) * TILE_SIZE;
        t.translation.y = (p_pos.y as f32) * TILE_SIZE;
    }
}

/// Sets rendering properties for tiles of the [`GameMap`]
fn render_map(
    map: Res<GameMap>,
    player: Query<&Viewshed, With<Player>>,
    mut tiles: Query<(&Position, &TileType, &mut TextureAtlasSprite)>,
) {
    let player_view = player.single();
    for (p, _, mut s) in tiles.iter_mut() {
        if let Ok(idx) = map.xy_to_idx(p.x, p.y) {
            if map.revealed[idx] {
                if player_view.visible_tiles.contains(p) {
                    s.color = Color::WHITE;
                } else {
                    s.color = Color::GRAY;
                }
            } else {
                s.color = Color::BLACK;
            }
        }
    }
}

/// Sets creatures that are outside the [`Player`'s](Player) viewshed invisible
pub fn render_creatures(
    view: Query<&Viewshed, With<Player>>,
    mut objects: Query<(&mut Visibility, &Position), Without<TileType>>,
) {
    for view in view.iter() {
        for (mut visible, p) in objects.iter_mut() {
            if view.visible_tiles.contains(p) {
                visible.is_visible = true;
            } else {
                visible.is_visible = false;
            }
        }
    }
}

fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // root node
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(200.0), Val::Percent(100.0)),
                        border: Rect::all(Val::Px(2.0)),
                        ..default()
                    },
                    color: Color::rgb(0.65, 0.65, 0.65).into(),
                    ..default()
                })
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                                align_items: AlignItems::FlexEnd,
                                ..default()
                            },
                            color: Color::rgb(0.15, 0.15, 0.15).into(),
                            ..default()
                        })
                        .with_children(|parent| {
                            // text
                            parent.spawn_bundle(TextBundle {
                                style: Style {
                                    margin: Rect::all(Val::Px(5.0)),
                                    ..default()
                                },
                                text: Text::with_section(
                                    "Text Example",
                                    TextStyle {
                                        font: asset_server.load("Dawnlike/GUI/SDS_8x8.ttf"),
                                        font_size: 20.0,
                                        color: Color::WHITE,
                                    },
                                    Default::default(),
                                ),
                                ..default()
                            });
                        });
                });
        });
}
