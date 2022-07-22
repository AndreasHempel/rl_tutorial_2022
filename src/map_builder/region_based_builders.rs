use std::iter::FromIterator;

use rand::Rng;

use super::{MapBuildData, MapModifier, MapRng, Region};
use crate::map::TileType;

/// Generates Voronoi regions around randomly selected seed points in which to spawn entities
#[derive(Debug)]
pub struct VoronoiRegion {
    distance_function: DistanceFunction,
    number_of_regions: u32,
}

/// Available distance functions to use in determining [`VoronoiRegion`]s
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum DistanceFunction {
    Euclidean,
    Manhattan,
    Maximum,
}

impl VoronoiRegion {
    pub fn new(number_of_regions: u32, distance_function: DistanceFunction) -> Box<VoronoiRegion> {
        Box::new(VoronoiRegion {
            distance_function,
            number_of_regions,
        })
    }

    fn find_closest_point(&self, p: (u32, u32), points: &[(u32, u32)]) -> usize {
        let mut min_distance = u32::MAX;
        let mut min_idx = usize::MAX;
        for (idx, &(x, y)) in points.iter().enumerate() {
            let dx = p.0.abs_diff(x);
            let dy = p.1.abs_diff(y);
            let distance = match self.distance_function {
                // No need to take the square root to determine the closest point
                DistanceFunction::Euclidean => dx * dx + dy * dy,
                DistanceFunction::Manhattan => dx + dy,
                DistanceFunction::Maximum => dx.max(dy),
            };

            if distance < min_distance {
                min_distance = distance;
                min_idx = idx;
            }
        }

        min_idx
    }
}

impl MapModifier for VoronoiRegion {
    fn modify_map(&mut self, rng: &mut MapRng, build_data: &mut MapBuildData) {
        let seed_points = {
            let x_range = 0..=(build_data.map.width - 1);
            let y_range = 0..=(build_data.map.height - 1);
            Vec::from_iter((0..self.number_of_regions).map(|_| {
                (
                    rng.gen_range(x_range.clone()),
                    rng.gen_range(y_range.clone()),
                )
            }))
        };

        let mut regions = vec![Region::new(); self.number_of_regions as usize];
        let map = &build_data.map;
        for x in 1..map.width - 1 {
            for y in 1..map.height - 1 {
                let idx = map.xy_to_idx(x, y).unwrap();
                if map.tiles[idx] == TileType::Floor {
                    let region_idx = self.find_closest_point((x, y), &seed_points);
                    regions[region_idx].push((x, y));
                }
            }
        }

        build_data.metadata.regions = Some(regions);
        build_data.take_snapshot();
    }
}

pub struct RegionBasedSpawner {
    max_spawns: u32,
}

impl RegionBasedSpawner {
    pub fn new(max_spawns: u32) -> Box<RegionBasedSpawner> {
        Box::new(RegionBasedSpawner { max_spawns })
    }
}

impl MapModifier for RegionBasedSpawner {
    fn modify_map(&mut self, rng: &mut MapRng, build_data: &mut MapBuildData) {
        let regions = build_data
            .metadata
            .regions
            .as_ref()
            .expect("Need regions to spawn into!");

        let mut spawn_lists = Vec::with_capacity(regions.len());
        for r in regions.iter() {
            let spawn_list = super::spawner::fill_region(rng, r, self.max_spawns);
            spawn_lists.push(spawn_list);
        }
        // Take a snapshot after each separate region has been populated
        for spawn_list in spawn_lists {
            build_data.metadata.spawn_list.extend(spawn_list);
            build_data.take_snapshot();
        }
    }
}
