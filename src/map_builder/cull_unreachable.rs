use pathfinding::directed::dijkstra::dijkstra_all;

use super::{MapBuildData, MapModifier, MapRng};
use crate::map::TileType;

/// Builder that identifies all rechable tiles from a set starting position and forces all unreachable tiles to be walls
/// TODO: Make this configurable to allow diagonal movement or not
pub struct CullUnreachable;

impl CullUnreachable {
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl MapModifier for CullUnreachable {
    fn modify_map(&mut self, _rng: &mut MapRng, build_data: &mut MapBuildData) {
        let start_pos = build_data
            .metadata
            .starting_position
            .expect("Cannot determine unreachable areas without a starting position!");

        let reachable = dijkstra_all::<_, i32, _, _>(&start_pos, |&(x, y)| {
            vec![(x + 1, y), (x, y + 1), (x, y - 1), (x - 1, y)]
                .into_iter()
                .filter(|&(x, y)| {
                    if let Ok(idx) = build_data.map.xy_to_idx(x, y) {
                        build_data.map.tiles[idx] != TileType::Wall
                    } else {
                        false
                    }
                })
                .map(|p| (p, 1))
        });

        for idx in 0..build_data.map.length() {
            let pos = build_data
                .map
                .idx_to_xy(idx)
                .expect("Tile index {idx} is outside the map!");
            if !reachable.contains_key(&pos) && pos != start_pos {
                build_data.map.tiles[idx] = TileType::Wall;
            }
        }
        build_data.take_snapshot();
    }
}
