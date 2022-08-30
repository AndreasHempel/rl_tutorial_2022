use bevy::prelude::*;
use iyes_loopless::prelude::AppLooplessStateExt;

use crate::GameState;

/// Marks the (only) player entity and keeps track how many turns they have left to play
#[derive(Component, Debug)]
pub struct Player {
    /// How many action points this [`Player`] had at original creation (used for [`Self::reset()`])
    original_ap: u32,
    /// How many action points this [`Player`] has left
    action_points: u32,
    /// How many action points this [`Player`] has spent over the course of the game
    spent_ap: u32,
    /// How many turns this [`Player`] has completed so far
    completed: u32,
}

impl Player {
    /// Creates a new component for a [`Player`] with limited turns
    pub fn new(remaining: u32) -> Self {
        Self {
            original_ap: remaining,
            action_points: remaining,
            spent_ap: 0,
            completed: 0,
        }
    }

    // FIXME: The doctest below is currently not executed as rustdoc does not run doctests
    // for binary crates (see https://github.com/rust-lang/rust/issues/50784 for details)
    /// Resets this [`Player`] to its original state:
    /// ```
    /// let mut p = Player::new(10);
    /// p.remaining = 3;
    /// p.reset();
    /// assert_eq!(p.remaining, 10);
    /// ```
    pub fn reset(&mut self) {
        self.action_points = self.original_ap;
        self.completed = 0;
        self.spent_ap = 0;
    }

    /// This [`Player`] performed an action with the given cost which are subtracted from their
    /// currently available action points
    pub fn act(&mut self, cost: u32) -> Result<(), TurnCounterError> {
        self.spent_ap += cost;
        if self.action_points < cost {
            self.action_points = 0;
            return Err(TurnCounterError::NoTimeLeft);
        } else {
            self.action_points -= cost;
        }
        Ok(())
    }

    /// Recover the given amount of action points
    pub fn recover_ap(&mut self, amount: u32) {
        self.action_points += amount;
    }

    /// End the game turn for this [`Player`]
    pub fn end_turn(&mut self) {
        self.completed += 1;
    }

    /// Get the finished number of turns
    pub fn get_completed_turns(&self) -> u32 {
        self.completed
    }

    /// Get the remaining number of turns
    pub fn get_remaining_ap(&self) -> u32 {
        self.action_points
    }
}

/// Signals possible issues upon ticking the game turn
pub enum TurnCounterError {
    /// The player ran out of turns
    NoTimeLeft,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::EnterNewLevel, increase_action_points);
    }
}

/// Increases each player's action points by a fixed amount
fn increase_action_points(mut players: Query<&mut Player>) {
    for mut p in players.iter_mut() {
        p.recover_ap(40);
    }
}
