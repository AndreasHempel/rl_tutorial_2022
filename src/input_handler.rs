use bevy::prelude::*;

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

/// Map keyboard input to player actions and update the [GameState]
fn keyboard_event_handler(
    mut keys: ResMut<Input<KeyCode>>,
    mut player: Query<Entity, With<Player>>,
    mut game_state: ResMut<State<GameState>>,
    mut commands: Commands,
) {
    let mut pushed = Vec::new();

    let action = if keys.pressed(KeyCode::Right) {
        pushed.push(KeyCode::Right);
        Some(PlayerAction::Move { dx: 1, dy: 0 })
    } else if keys.pressed(KeyCode::Left) {
        pushed.push(KeyCode::Left);
        Some(PlayerAction::Move { dx: -1, dy: 0 })
    } else if keys.pressed(KeyCode::Up) {
        pushed.push(KeyCode::Up);
        Some(PlayerAction::Move { dx: 0, dy: 1 })
    } else if keys.pressed(KeyCode::Down) {
        pushed.push(KeyCode::Down);
        Some(PlayerAction::Move { dx: 0, dy: -1 })
    } else {
        None
    };

    if let Some(action) = action {
        let e = player.single_mut();
        if game_state.current() == &GameState::WaitingForPlayer {
            commands.entity(e).insert(TakingTurn);

            match action {
                PlayerAction::Move { dx, dy } => commands.entity(e).insert(WantsToMove { dx, dy }),
            };
            // If a keypress leads to a state change, the keyboard state needs to be reset
            // (see https://bevy-cheatbook.github.io/programming/states.html#with-input for details)
            game_state.set(GameState::Ticking).unwrap();
            for kc in pushed {
                keys.reset(kc);
            }
        }
    }
}
