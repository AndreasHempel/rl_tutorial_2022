use bevy::prelude::*;
use pathfinding::directed::astar::astar;
use rand::prelude::*;

use crate::{
    components::{Actor, Monster, MonsterStrategy, Position, TakingTurn, Viewshed, WantsToMove},
    map::GameMap,
    player::Player,
};

/// Bundles AI-related systems
#[derive(Debug)]
pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(monsters_strategize)
            .add_system(wandering_monsters)
            .add_system(chasing_monsters);
    }
}

/// Monsters select different strategies if they can see the [`Player`] or not
#[allow(clippy::type_complexity)]
fn monsters_strategize(
    mut commands: Commands,
    monsters: Query<(Entity, &Viewshed), (With<Actor>, With<Monster>)>,
    player: Query<(Entity, &Position), With<Player>>,
) {
    for (e, view) in monsters.iter() {
        if let Some((p, _)) = player
            .iter()
            .find(|(_, p_pos)| view.visible_tiles.contains(p_pos))
        {
            commands
                .entity(e)
                .insert(MonsterStrategy::Blocking { player: p });
        } else {
            commands.entity(e).insert(MonsterStrategy::Wandering);
        }
    }
}

/// Moves monsters randomly across the screen
#[allow(clippy::type_complexity)]
fn wandering_monsters(
    monsters: Query<
        (Entity, &Position, &MonsterStrategy),
        (With<Actor>, With<Monster>, With<TakingTurn>),
    >,
    map: Res<GameMap>,
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();
    for (e, pos, _) in monsters
        .iter()
        .filter(|&(_, _, strat)| strat == &MonsterStrategy::Wandering)
    {
        let neighbors = map.get_free_neighbors(pos);
        if !neighbors.is_empty() {
            let idx = rng.gen_range(0..neighbors.len());
            let (dx, dy) = &neighbors[idx] - pos;
            commands.entity(e).insert(WantsToMove { dx, dy });
        }
    }
}

/// Moves monsters randomly across the screen
#[allow(clippy::type_complexity)]
fn chasing_monsters(
    monsters: Query<
        (Entity, &Position, &MonsterStrategy),
        (With<Actor>, With<Monster>, With<TakingTurn>),
    >,
    players: Query<&Position, With<Player>>,
    map: Res<GameMap>,
    mut commands: Commands,
) {
    for (e, pos, player) in monsters.iter().filter_map(|(e, pos, strat)| {
        if let MonsterStrategy::Blocking { player } = strat {
            Some((e, pos, *player))
        } else {
            None
        }
    }) {
        let p_pos = players
            .get(player)
            .expect("Could not find a position for player {player:?}!");
        if let Some((path, _)) = astar(
            pos,
            |p| {
                map.get_free_neighbors(p)
                    .iter()
                    .map(|&p| (p, 1))
                    .collect::<Vec<_>>()
            },
            |p| p.distance(p_pos) - 1,
            |p| p.distance(p_pos) == 1,
        ) {
            if path.len() > 1 {
                let (dx, dy) = &path[1] - pos;
                commands.entity(e).insert(WantsToMove { dx, dy });
            } else {
                // Monster is already close enough -> skip its turn
                commands.entity(e).remove::<TakingTurn>();
            }
        } else {
            // Monster is blocked from reaching the player -> skip its turn
            // FIXME: Improve this to have the monster still get closer to the player
            commands.entity(e).remove::<TakingTurn>();
        }
    }
}
