use bevy::prelude::*;

use crate::GameState;

use crate::components::{Player, Position};

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

/// Updates the [Position] component of all moving actors
pub fn move_actors(
    mut chars: Query<(Entity, &WantsToMove, &mut Position), With<TakingTurn>>,
    mut commands: Commands,
) {
    // Iterate over all actors that intend to move
    chars
        .iter_mut()
        .map(|(e, mov, mut p)| {
            p.x += mov.dx;
            p.y += mov.dy;
            commands
                .entity(e)
                .remove::<TakingTurn>()
                .remove::<WantsToMove>();
        })
        .count();
}

/// Marks all non-player actors to make their next move
pub fn enqueue_actors(
    actors: Query<Entity, (With<Actor>, Without<Player>)>,
    mut commands: Commands,
) {
    for a in actors.iter() {
        commands.entity(a).insert(TakingTurn);
    }
}

/// Waits for all actors to have taken their turn and returns control to the player
pub fn wait_for_player(
    actors: Query<&Actor, With<TakingTurn>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if actors.is_empty() {
        game_state
            .set(GameState::WaitingForPlayer)
            .expect("Failed to wait for player input!");
    }
}
