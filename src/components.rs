use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/// Marks the (only) player entity
#[derive(Component, Debug)]
pub struct Player;

/// Marks a monstrous being
#[derive(Component, Debug)]
pub struct Monster;

/// Position of an entity on the map (always non-negative)
#[derive(Component, Debug, Inspectable, Default, PartialEq, Clone, Copy)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

/// Signals an actor's intent to move
#[derive(Debug, Component)]
pub struct WantsToMove {
    pub dx: i32,
    pub dy: i32,
}

/// Marks an entity that may take actions on each tick
#[derive(Debug, Component, Default)]
pub struct Actor;

/// Marker component to indicate [Actor] that are taking a turn this game tick
#[derive(Debug, Component)]
pub struct TakingTurn;

/// Marks entities that have some form of vision
#[derive(Debug, Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Position>,
    pub range: u32,
}

impl Viewshed {
    pub fn new(range: u32) -> Self {
        Viewshed {
            visible_tiles: Vec::new(),
            range,
        }
    }
}

/// Marks entities that block tiles for movement
#[derive(Debug, Component)]
pub struct BlocksMovement;
