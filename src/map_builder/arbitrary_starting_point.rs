use rand::Rng;

use super::{MapBuildData, MapModifier, MapRng};
use crate::map::TileType;

pub struct ArbitraryStartingPoint;

impl ArbitraryStartingPoint {
    pub fn new() -> Box<Self> {
        Box::new(Self)
    }
}

impl MapModifier for ArbitraryStartingPoint {
    fn modify_map(&mut self, rng: &mut MapRng, build_data: &mut MapBuildData) {
        let random_idx = rng.gen_range(0..build_data.map.length());
        if let Some((starting_idx, _)) = build_data
            .map
            .tiles
            .iter()
            .enumerate()
            .skip(random_idx)
            .find(|&(_, t)| *t == TileType::Floor)
        {
            let start_pos = build_data.map.idx_to_xy(starting_idx).unwrap();
            build_data.metadata.starting_position = Some(start_pos);
        } else if let Some((starting_idx, _)) = build_data
            .map
            .tiles
            .iter()
            .enumerate()
            .take(random_idx)
            .rev()
            .find(|&(_, t)| *t == TileType::Floor)
        {
            let start_pos = build_data.map.idx_to_xy(starting_idx).unwrap();
            build_data.metadata.starting_position = Some(start_pos);
        } else {
            panic!("Cannot find a single floor tile as the starting position!");
        }
        build_data.take_snapshot();
    }
}
