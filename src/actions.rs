use bevy::prelude::*;

use crate::GameState;

use crate::motion_resolver::{MotionResolver, MoveAttempt};
use crate::{
    components::{Actor, Player, Position, TakingTurn, WantsToMove},
    map::GameMap,
};

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
    movers: Query<(Entity, &WantsToMove), With<TakingTurn>>,
    mut chars: Query<&mut Position>,
    mut map: ResMut<GameMap>,
    mut commands: Commands,
) {
    // Iterate over all actors that intend to move
    for (e, mov) in movers.iter() {
        let p = chars.get(e).unwrap();

        let resolver = MotionResolver::default();
        if let Ok(next_pos) = resolver.resolve(
            MoveAttempt {
                entity: e,
                from: *p,
                dx: mov.dx,
                dy: mov.dy,
            },
            map.as_mut(),
        ) {
            for (e, next) in next_pos {
                if let Ok(mut p) = chars.get_mut(e) {
                    *p = next;
                } else {
                    warn!("Cannot find position of {e:?} to move it to {next:?}!");
                }
            }
        } else {
            warn!("Could not move {e:?} from {p:?} by ({mov:?})");
        }
        commands
            .entity(e)
            .remove::<WantsToMove>()
            .remove::<TakingTurn>();
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
