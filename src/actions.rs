use bevy::prelude::*;

use crate::GameState;

use crate::components::{Actor, Player, Position, TakingTurn, WantsToMove};

/// Bundles all systems responsible for turn-based action management
#[derive(Debug)]
pub struct ActionPlugin;

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_update(GameState::Ticking).with_system(move_actors))
            .add_system_set(SystemSet::on_enter(GameState::Ticking).with_system(enqueue_actors))
            .add_system_set(SystemSet::on_update(GameState::Ticking).with_system(wait_for_player));
    }
}

/// Updates the [Position] component of all moving actors
fn move_actors(
    mut chars: Query<(Entity, &WantsToMove, &mut Position), With<TakingTurn>>,
    mut commands: Commands,
) {
    // Iterate over all actors that intend to move
    chars
        .iter_mut()
        .map(|(e, mov, mut p)| {
            if mov.dx >= 0 {
                p.x += mov.dx as u32;
            } else {
                p.x -= mov.dx.abs() as u32;
            }
            if mov.dy.is_positive() {
                p.y += mov.dy as u32;
            } else {
                p.y -= mov.dy.abs() as u32;
            }
            commands
                .entity(e)
                .remove::<TakingTurn>()
                .remove::<WantsToMove>();
        })
        .count();
}

/// Marks all non-player actors to make their next move
fn enqueue_actors(actors: Query<Entity, (With<Actor>, Without<Player>)>, mut commands: Commands) {
    for a in actors.iter() {
        commands.entity(a).insert(TakingTurn);
    }
}

/// Waits for all actors to have taken their turn and returns control to the player
fn wait_for_player(
    actors: Query<&Actor, With<TakingTurn>>,
    mut game_state: ResMut<State<GameState>>,
) {
    if actors.is_empty() {
        game_state
            .set(GameState::WaitingForPlayer)
            .expect("Failed to wait for player input!");
    }
}
