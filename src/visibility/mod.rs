use bevy::prelude::*;

use crate::{
    components::{Position, Viewshed},
    map::{GameMap, TileType},
    player::Player,
};

mod shadowcasting;
mod square_xray;

use shadowcasting::compute_fov;

pub fn determine_visibility(
    mut viewers: Query<(Entity, &Position, &mut Viewshed)>,
    player: Query<Entity, With<Player>>,
    mut map: ResMut<GameMap>,
) {
    let player = player.single();
    for (e, pos, mut view) in viewers.iter_mut() {
        let range = view.range;
        let is_blocking = |pos: Position| {
            if let Ok(idx) = map.xy_to_idx(pos.x, pos.y) {
                map.tiles[idx] == TileType::Wall
            } else {
                // Consider tiles outside the map as walls
                true
            }
        };
        view.visible_tiles = compute_fov(pos, is_blocking, range);

        if e == player {
            for Position { x, y } in view.visible_tiles.iter() {
                if let Ok(idx) = map.xy_to_idx(*x, *y) {
                    map.revealed[idx] = true;
                }
            }
        }
    }
}
