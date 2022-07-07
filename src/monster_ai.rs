use bevy::prelude::*;
use rand::prelude::*;

use crate::components::{Actor, Monster, TakingTurn, WantsToMove};

/// Bundles AI-related systems
#[derive(Debug)]
pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(monster_actors);
    }
}

/// Moves monsters randomly across the screen
#[allow(clippy::type_complexity)]
fn monster_actors(
    monsters: Query<Entity, (With<Actor>, With<Monster>, With<TakingTurn>)>,
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();
    for e in monsters.iter() {
        commands.entity(e).insert(WantsToMove {
            dx: rng.gen_range(-1..=1),
            dy: rng.gen_range(-1..=1),
        });
    }
}
