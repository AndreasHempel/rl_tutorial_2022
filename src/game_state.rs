use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    actions::ActionCost,
    components::{Actor, LevelGoal, Position, TakingTurn},
    log::{GameEvent, LogBuffer, LogMessage},
    player::{Player, TurnCounterError},
    GameState,
};

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
fn setup_game(
    mut commands: Commands,
    mut players: Query<&mut Player>,
    mut logs: ResMut<LogBuffer>,
) {
    for mut p in players.iter_mut() {
        p.reset();
    }
    logs.clear();
    commands.insert_resource(NextState(GameState::EnterNewLevel));
}

/// Waits for all actors to have taken their turn, ticks one game turn forward,
/// and returns control to the player or signals GameOver
fn wait_for_player(
    mut commands: Commands,
    actors: Query<&Actor, With<TakingTurn>>,
    mut action_cost: EventReader<ActionCost>,
    mut players: Query<(Entity, &mut Player)>,
) {
    let (e_p, mut player) = players.single_mut();
    let total_cost = action_cost
        .iter()
        .filter(|&e| e.actor == e_p)
        .map(|e| e.cost)
        .sum();
    if let Err(TurnCounterError::NoTimeLeft) = player.act(total_cost) {
        commands.insert_resource(NextState(GameState::GameOver));
    } else if actors.is_empty() {
        player.end_turn();
        commands.insert_resource(NextState(GameState::WaitingForPlayer));
    }
}

/// Checks if the player has reached the level goal
fn check_level_goals(
    mut commands: Commands,
    player: Query<(Entity, &Position), With<Player>>,
    goals: Query<&Position, With<LevelGoal>>,
    mut events: EventWriter<LogMessage>,
) {
    if let Ok((e, pos)) = player.get_single() {
        for goal in goals.iter() {
            if pos == goal {
                events.send(LogMessage {
                    actor: e,
                    event: GameEvent::FoundTreasure,
                });
                commands.insert_resource(NextState(GameState::EnterNewLevel));
            }
        }
    }
}

fn finish_level_setup(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::WaitingForPlayer));
}
