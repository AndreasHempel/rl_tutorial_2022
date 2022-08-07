use bevy::prelude::*;
use iyes_loopless::condition::IntoConditionalExclusiveSystem;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::GameState;

use crate::{
    components::{Actor, Player, Position, Pushable, TakingTurn, WantsToMove},
    map::GameMap,
    motion_resolver::{MotionResolver, MoveAttempt},
};

/// Bundles all systems responsible for turn-based action management
#[derive(Debug)]
pub struct ActionPlugin;

/// System labels used for system ordering
#[derive(Debug, PartialEq, Eq, Hash, Clone, SystemLabel)]
enum SystemLabels {
    MoveActors,
}

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Ticking, enqueue_actors)
            .add_system(
                move_actors
                    .run_in_state(GameState::Ticking)
                    .label(SystemLabels::MoveActors),
            );
    }
}

/// Updates the [Position] component of all moving actors
fn move_actors(
    movers: Query<(Entity, &WantsToMove), With<TakingTurn>>,
    mut chars: Query<&mut Position>,
    pushables: Query<Entity, With<Pushable>>,
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
            |e| pushables.contains(e),
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
