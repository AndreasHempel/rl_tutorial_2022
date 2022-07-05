use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    actions::{Actor, TakingTurn, WantsToMove},
    components::Monster,
};

/// Moves monsters randomly across the screen
pub fn monster_actors(
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
