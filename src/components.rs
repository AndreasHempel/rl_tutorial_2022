use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/// Marks the (only) player entity
#[derive(Component, Debug)]
pub struct Player;

/// Marks a monstrous being
#[derive(Component, Debug)]
pub struct Monster;

/// Position of an entity on the map
#[derive(Component, Debug, Inspectable, Default)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
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
