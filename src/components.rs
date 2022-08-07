use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/// Marks the (only) player entity
#[derive(Component, Debug)]
pub struct Player;

/// Marks a monstrous being
#[derive(Component, Debug)]
pub struct Monster;

/// Indicates which strategy this monster is currently employing
#[derive(Component, Debug, PartialEq)]
pub enum MonsterStrategy {
    /// This monster is currently wandering around
    Wandering,

    /// This monster is attempting to block the player
    Blocking { player: Entity },
}

/// Position of an entity on the map (always non-negative)
#[derive(Component, Debug, Inspectable, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// Computes a maximum norm different between two [`Position`s](Position)
    pub fn distance(&self, other: &Position) -> u32 {
        let dx = if self.x > other.x {
            self.x - other.x
        } else {
            other.x - self.x
        };
        let dy = if self.y > other.y {
            self.y - other.y
        } else {
            other.y - self.y
        };
        dx.max(dy)
    }
}

impl From<Position> for (u32, u32) {
    fn from(pos: Position) -> Self {
        (pos.x, pos.y)
    }
}

impl From<&Position> for (u32, u32) {
    fn from(pos: &Position) -> Self {
        (pos.x, pos.y)
    }
}

impl core::ops::Add<(i32, i32)> for &Position {
    type Output = Position;
    fn add(self, rhs: (i32, i32)) -> Self::Output {
        let x = {
            if rhs.0.is_negative() {
                self.x - rhs.0.unsigned_abs()
            } else {
                self.x + rhs.0 as u32
            }
        };
        let y = {
            if rhs.1.is_negative() {
                self.y - rhs.1.unsigned_abs()
            } else {
                self.y + rhs.1 as u32
            }
        };
        Position::new(x, y)
    }
}

impl core::ops::Sub<&Position> for &Position {
    type Output = (i32, i32);

    fn sub(self, rhs: &Position) -> Self::Output {
        let dx = if self.x > rhs.x {
            (self.x - rhs.x) as i32
        } else {
            -((rhs.x - self.x) as i32)
        };
        let dy = if self.y > rhs.y {
            (self.y - rhs.y) as i32
        } else {
            -((rhs.y - self.y) as i32)
        };
        (dx, dy)
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
    /// Create a new [`Viewshed`] with the specified sight range
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

/// Marks entities that can be moved out of the way
#[derive(Debug, Component)]
pub struct Pushable;

/// Marks tiles that are goals to proceed to the next level
#[derive(Debug, Component)]
pub struct LevelGoal;
