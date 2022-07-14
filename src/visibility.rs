use bevy::prelude::*;

use crate::{
    components::{Player, Position, Viewshed},
    map::GameMap,
};

fn compute_visible_tiles(position: &Position, range: u32) -> Vec<Position> {
    let mut visible = Vec::new();
    for i in -(range as i32)..(range as i32) {
        for j in -(range as i32)..(range as i32) {
            let t = Position {
                x: (position.x as i32 + i) as u32,
                y: (position.y as i32 + j) as u32,
            };
            visible.push(t);
        }
    }
    visible
}

pub fn determine_visibility(
    mut viewers: Query<(Entity, &Position, &mut Viewshed)>,
    player: Query<Entity, With<Player>>,
    mut map: ResMut<GameMap>,
) {
    let player = player.single();
    for (e, pos, mut view) in viewers.iter_mut() {
        let range = view.range;
        view.visible_tiles = compute_visible_tiles(pos, range);

        if e == player {
            for Position { x, y } in view.visible_tiles.iter() {
                if let Ok(idx) = map.xy_to_idx(*x, *y) {
                    map.revealed[idx] = true;
                }
            }
        }
    }
}
