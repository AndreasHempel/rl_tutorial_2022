use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    components::{Actor, LevelGoal, Player, Position, TakingTurn},
    GameState,
};

/// Keeps track of the number of elapsed turns, how many turns the player has left etc.
#[derive(Debug)]
pub struct PlayerTurns {
    remaining: u32,
    completed: u32,
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

/// Signals possible issues upon ticking the game turn
enum TurnCounterError {
    /// The player ran out of turns
    NoTimeLeft,
}

/// Manages the main state machine for [`GameState`] and general game setup steps
pub struct GameStatePlugin;

/// System labels used for system ordering
#[derive(Debug, PartialEq, Eq, Hash, Clone, SystemLabel)]
enum SystemLabels {
    WaitForPlayer,
    CheckLevelGoals,
}

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::StartGame)
            .add_system(setup_game.run_in_state(GameState::StartGame))
            .add_system(finish_level_setup.run_in_state(GameState::EnterNewLevel))
            .add_system_to_stage(
                CoreStage::PostUpdate,
                wait_for_player
                    .run_in_state(GameState::Ticking)
                    .label(SystemLabels::WaitForPlayer),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                check_level_goals
                    .run_in_state(GameState::Ticking)
                    .label(SystemLabels::CheckLevelGoals),
            );
    }
}

/// Initializes resources etc. to their state at the start of a game
fn setup_game(mut commands: Commands) {
    commands.insert_resource(PlayerTurns {
        remaining: 100,
        completed: 0,
    });

    commands.insert_resource(NextState(GameState::EnterNewLevel));
}

/// Waits for all actors to have taken their turn, ticks one game turn forward,
/// and returns control to the player or signals GameOver
fn wait_for_player(
    mut commands: Commands,
    actors: Query<&Actor, With<TakingTurn>>,
    mut turns: ResMut<PlayerTurns>,
) {
    if actors.is_empty() {
        if let Err(TurnCounterError::NoTimeLeft) = turns.tick() {
            commands.insert_resource(NextState(GameState::GameOver));
        } else {
            commands.insert_resource(NextState(GameState::WaitingForPlayer));
        }
    }
}

/// Checks if the player has reached the level goal
fn check_level_goals(
    mut commands: Commands,
    player: Query<&Position, With<Player>>,
    goals: Query<&Position, With<LevelGoal>>,
    mut turns: ResMut<PlayerTurns>,
) {
    if let Ok(pos) = player.get_single() {
        for goal in goals.iter() {
            if pos == goal {
                turns.remaining += 40;
                commands.insert_resource(NextState(GameState::EnterNewLevel));
            }
        }
    }
}

fn finish_level_setup(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::WaitingForPlayer));
}
