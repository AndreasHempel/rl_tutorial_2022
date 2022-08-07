use bevy::prelude::*;
use iyes_loopless::state::{CurrentState, NextState};

use super::GameState;

use crate::components::{Player, TakingTurn, WantsToMove};

/// Bundles systems handling keyboard inputs
#[derive(Debug)]
pub struct KeyboardInputPlugin;

impl Plugin for KeyboardInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(keyboard_event_handler);
    }
}

/// Possible actions the player can take
#[derive(Debug)]
enum PlayerAction {
    Move { dx: i32, dy: i32 },
}

/// Map keyboard input to player actions and update the [`GameState`]
fn keyboard_event_handler(
    keys: Res<Input<KeyCode>>,
    mut player: Query<Entity, With<Player>>,
    game_state: Res<CurrentState<GameState>>,
    mut commands: Commands,
) {
    let action = if keys.pressed(KeyCode::Right) {
        Some(PlayerAction::Move { dx: 1, dy: 0 })
    } else if keys.pressed(KeyCode::Left) {
        Some(PlayerAction::Move { dx: -1, dy: 0 })
    } else if keys.pressed(KeyCode::Up) {
        Some(PlayerAction::Move { dx: 0, dy: 1 })
    } else if keys.pressed(KeyCode::Down) {
        Some(PlayerAction::Move { dx: 0, dy: -1 })
    } else {
        None
    };

    if let Some(action) = action {
        let e = player.single_mut();
        if game_state.0 == GameState::WaitingForPlayer {
            commands.entity(e).insert(TakingTurn);

            match action {
                PlayerAction::Move { dx, dy } => commands.entity(e).insert(WantsToMove { dx, dy }),
            };
            commands.insert_resource(NextState(GameState::Ticking));
        }
    }
}
