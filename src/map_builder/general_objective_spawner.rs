use rand::Rng;

use super::{spawner::Spawnables, MapBuildData, MapModifier, MapRng};
use crate::map::TileType;

pub struct GeneralObjectiveSpawner {
    objective: Spawnables,
}

impl GeneralObjectiveSpawner {
    pub fn new(objective: Spawnables) -> Box<Self> {
        Box::new(Self { objective })
    }
}

impl MapModifier for GeneralObjectiveSpawner {
    fn modify_map(&mut self, rng: &mut MapRng, build_data: &mut MapBuildData) {
        // FIXME: This may select an already occupied spawn position
        let random_idx = rng.gen_range(0..build_data.map.length());
        // Try to find a floor tile after the randomly selected index
        if let Some((objective_idx, _)) = build_data
            .map
            .tiles
            .iter()
            .enumerate()
            .skip(random_idx)
            .find(|&(_, t)| *t == TileType::Floor)
        {
            let pos = build_data.map.idx_to_xy(objective_idx).unwrap();
            build_data.metadata.spawn_list.insert(pos, self.objective);
        } else if let Some((objective_idx, _)) = build_data
            // If that fails, search backwards from the index
            .map
            .tiles
            .iter()
            .enumerate()
            .take(random_idx)
            .rev()
            .find(|&(_, t)| *t == TileType::Floor)
        {
            let pos = build_data.map.idx_to_xy(objective_idx).unwrap();
            build_data.metadata.spawn_list.insert(pos, self.objective);
        } else {
            panic!("Cannot find a single floor tile as the starting position!");
        }
        build_data.take_snapshot();
    }
}
