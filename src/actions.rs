use bevy::prelude::*;

use crate::GameState;

use crate::components::{LevelGoal, Pushable};
use crate::level::{LevelSettings, MapRNG};
use crate::motion_resolver::{MotionResolver, MoveAttempt};
use crate::{
    components::{Actor, Player, Position, TakingTurn, WantsToMove},
    map::GameMap,
};

/// Bundles all systems responsible for turn-based action management
#[derive(Debug)]
pub struct ActionPlugin;

/// Keeps track of the number of elapsed turns, how many turns the player has left etc.
#[derive(Debug)]
pub struct PlayerTurns {
    remaining: u32,
    completed: u32,
}

/// Signals possible issues upon ticking the game turn
enum TurnCounterError {
    /// The player ran out of turns
    NoTimeLeft,
}

impl PlayerTurns {
    /// End the game turn
    fn tick(&mut self) -> Result<(), TurnCounterError> {
        self.completed += 1;
        self.remaining -= 1;
        if self.remaining == 0 {
            return Err(TurnCounterError::NoTimeLeft);
        }
        Ok(())
    }

    /// Get the finished number of turns
    pub fn get_completed(&self) -> u32 {
        self.completed
    }

    /// Get the remaining number of turns
    pub fn get_remaining(&self) -> u32 {
        self.remaining
    }
}

/// System labels used for system ordering
#[derive(Debug, PartialEq, Eq, Hash, Clone, SystemLabel)]
enum SystemLabels {
    MoveActors,
    CheckLevelGoals,
    WaitForPlayer,
}

impl Plugin for ActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::StartGame).with_system(setup_game))
            .add_system_set(SystemSet::on_enter(GameState::Ticking).with_system(enqueue_actors))
            .add_system_set(
                SystemSet::on_update(GameState::Ticking)
                    .with_system(move_actors)
                    .label(SystemLabels::MoveActors),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Ticking)
                    .with_system(check_level_goals)
                    .label(SystemLabels::CheckLevelGoals)
                    .after(SystemLabels::MoveActors),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Ticking)
                    .with_system(wait_for_player)
                    .label(SystemLabels::WaitForPlayer)
                    .after(SystemLabels::MoveActors),
            );
    }
}

/// Initializes resources etc. to their state at the start of a game
fn setup_game(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    level_settings: Res<LevelSettings>,
) {
    commands.insert_resource(PlayerTurns {
        remaining: 100,
        completed: 0,
    });

    // Reset map generation RNG to the same seed upon restarting the game
    let rng = rand::SeedableRng::seed_from_u64(level_settings.original_seed);
    commands.insert_resource(MapRNG(rng));

    state
        .set(GameState::EnterNewLevel)
        .expect("Could not generate a new level after setting up the game!");
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

/// Waits for all actors to have taken their turn, ticks one game turn forward,
/// and returns control to the player or signals GameOver
fn wait_for_player(
    actors: Query<&Actor, With<TakingTurn>>,
    mut game_state: ResMut<State<GameState>>,
    mut turns: ResMut<PlayerTurns>,
) {
    if actors.is_empty() {
        if let Err(TurnCounterError::NoTimeLeft) = turns.tick() {
            game_state
                .set(GameState::GameOver)
                .expect("Failed to signal game over!");
        } else {
            game_state
                .set(GameState::WaitingForPlayer)
                .expect("Failed to wait for player input!");
        }
    }
}

/// Checks if the player has reached the level goal
fn check_level_goals(
    player: Query<&Position, With<Player>>,
    goals: Query<&Position, With<LevelGoal>>,
    mut state: ResMut<State<GameState>>,
    mut turns: ResMut<PlayerTurns>,
) {
    if let Ok(pos) = player.get_single() {
        for goal in goals.iter() {
            if pos == goal {
                turns.remaining += 40;
                state
                    .set(GameState::EnterNewLevel)
                    .expect("Failed to enter level generation after finding the level goal!");
            }
        }
    }
}
