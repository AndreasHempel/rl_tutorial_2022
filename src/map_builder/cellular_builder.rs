use rand::Rng;

use super::{InitialMapBuilder, MapBuildData, MapRng};
use crate::map::TileType;

pub struct CellularAutomataBuilder {
    iterations: i32,
    floor_likelihood: f64,
    neighbors_for_wall: Vec<i32>,
}

impl CellularAutomataBuilder {
    pub fn new(
        iterations: i32,
        floor_likelihood: f64,
        neighbors_for_wall: Vec<i32>,
    ) -> Box<CellularAutomataBuilder> {
        Box::new(CellularAutomataBuilder {
            iterations,
            floor_likelihood,
            neighbors_for_wall,
        })
    }
}

impl InitialMapBuilder for CellularAutomataBuilder {
    fn build_map(&mut self, rng: &mut MapRng, build_data: &mut MapBuildData) {
        // Completely randomize the map initially
        for y in 1..build_data.map.height - 1 {
            for x in 1..build_data.map.width - 1 {
                let idx = build_data.map.xy_to_idx(x, y).unwrap();
                if rng.gen_bool(self.floor_likelihood) {
                    build_data.map.tiles[idx] = TileType::Floor;
                } else {
                    build_data.map.tiles[idx] = TileType::Wall;
                }
            }
        }
        build_data.take_snapshot();

        // Iterate cellular automata rules
        for _i in 0..self.iterations {
            let mut new_tiles = build_data.map.tiles.clone();

            let height = build_data.map.height;
            let width = build_data.map.width;
            for y in 1..height - 1 {
                for x in 1..width - 1 {
                    let idx = build_data.map.xy_to_idx(x, y).unwrap();
                    let mut neighbors = 0;
                    if build_data.map.tiles[idx - 1] == TileType::Wall {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx + 1] == TileType::Wall {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx - width as usize] == TileType::Wall {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx + width as usize] == TileType::Wall {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx - (width as usize - 1)] == TileType::Wall {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx - (width as usize + 1)] == TileType::Wall {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx + (width as usize - 1)] == TileType::Wall {
                        neighbors += 1;
                    }
                    if build_data.map.tiles[idx + (width as usize + 1)] == TileType::Wall {
                        neighbors += 1;
                    }

                    if self.neighbors_for_wall.contains(&neighbors) {
                        new_tiles[idx] = TileType::Wall;
                    } else {
                        new_tiles[idx] = TileType::Floor;
                    }
                }
            }

            build_data.map.tiles = new_tiles;
            build_data.take_snapshot();
        }
    }
}
