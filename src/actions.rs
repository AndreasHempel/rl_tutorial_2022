use bevy::prelude::*;

use crate::GameState;

use crate::components::{Actor, Player, Position, TakingTurn, WantsToMove};
use crate::map::{GameMap, TileType};

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
    map: Res<GameMap>,
    mut commands: Commands,
) {
    // Iterate over all actors that intend to move
    for (e, mov, mut p) in chars.iter_mut() {
        let next = Position {
            x: {
                if mov.dx >= 0 {
                    p.x + mov.dx as u32
                } else {
                    p.x - mov.dx.abs() as u32
                }
            },
            y: {
                if mov.dy.is_positive() {
                    p.y + mov.dy as u32
                } else {
                    p.y - mov.dy.abs() as u32
                }
            },
        };
        if let Ok(idx) = map.xy_to_idx(next.x, next.y) {
            if map.tiles[idx] == TileType::Floor {
                *p = next;
                commands.entity(e).remove::<TakingTurn>();
            } else {
                warn!("Cannot move {e:?} to tile {}, {}", next.x, next.y);
            }
        }
        // Need to remove move intent component no matter what
        commands.entity(e).remove::<WantsToMove>();
    }
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
