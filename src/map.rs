use bevy::prelude::*;

use crate::{
    components::{BlocksMovement, Position},
    map_builder,
};

/// Available tile types
#[derive(Debug, Clone, Copy, PartialEq, Component)]
pub enum TileType {
    Floor,
    Wall,
}

/// Represents the concrete tile layout of the game Map
#[derive(Debug, Clone)]
pub struct GameMap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<TileType>,
    pub revealed: Vec<bool>,
    pub blocked: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>,
}

#[derive(Debug, PartialEq)]
pub struct OutsideMapError;

impl GameMap {
    /// Create a new, empty map with the given dimensions
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        if size == 0 {
            panic!("Cannot create a map with dimensions {width} x {height}!");
        }
        GameMap {
            width,
            height,
            tiles: vec![TileType::Wall; size],
            revealed: vec![false; size],
            blocked: vec![false; size],
            tile_content: vec![Vec::new(); size],
        }
    }

    /// Returns the total number of tiles in this [`GameMap`]
    pub fn length(&self) -> usize {
        (self.width as usize) * (self.height as usize)
    }

    /// Transforms a linear index to the corresponding (x,y) position in the map
    pub fn idx_to_xy(&self, idx: usize) -> Result<(u32, u32), OutsideMapError> {
        if idx >= (self.width * self.height) as usize {
            return Err(OutsideMapError);
        }
        let x = idx % self.width as usize;
        let y = idx / self.width as usize;
        Ok((x as u32, y as u32))
    }

    /// Transforms an (x, y) position into the corresponding linear index for parts of the [GameMap]
    pub fn xy_to_idx(&self, x: u32, y: u32) -> Result<usize, OutsideMapError> {
        if x >= self.width || y >= self.height {
            return Err(OutsideMapError);
        }
        Ok((self.width * y + x) as usize)
    }

    /// Marks all blocked tiles based on [`TileType`]
    fn determine_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    /// Forgets all indexed entities
    fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }
}

pub struct MapPlugin {
    pub builder: MapBuilder,
    pub seed: u64,
}

/// Available builder configs to choose from the command line
#[derive(Debug, clap::ValueEnum, Clone)]
pub enum MapBuilder {
    Rooms,
    Cellular,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        use map_builder::{
            arbitrary_starting_point::ArbitraryStartingPoint,
            cellular_builder::CellularAutomataBuilder,
            cull_unreachable::CullUnreachable,
            general_objective_spawner::GeneralObjectiveSpawner,
            room_based_builders::{
                PositionSelectionMode, RoomBasedObjectiveSpawner, RoomBasedSpawner,
                RoomBasedStartingPosition, RoomSelectionMode,
            },
            simple_map_builder::SimpleMapBuilder,
            spawner::Spawnables,
            BuilderChain,
        };

        let builder = BuilderChain::new();
        let builder = {
            match self.builder {
                MapBuilder::Rooms => {
                    let mut builder = builder.start_with(SimpleMapBuilder::new(10, 4, 12));
                    builder.with(RoomBasedStartingPosition::new(
                        RoomSelectionMode::First,
                        PositionSelectionMode::Center,
                    ));
                    builder.with(RoomBasedSpawner::new(1));
                    builder.with(RoomBasedObjectiveSpawner::new(
                        RoomSelectionMode::Last,
                        PositionSelectionMode::Random,
                        Spawnables::TreasureChest,
                    ));
                    builder
                }
                MapBuilder::Cellular => {
                    let mut builder = builder.start_with(CellularAutomataBuilder::new(
                        10,
                        0.4,
                        vec![0, 5, 6, 7, 8],
                    ));
                    builder.with(ArbitraryStartingPoint::new());
                    builder.with(CullUnreachable::new());
                    builder.with(GeneralObjectiveSpawner::new(Spawnables::TreasureChest));
                    builder
                }
            }
        };
        let mut rng = rand::SeedableRng::seed_from_u64(self.seed);
        let (map, map_metadata) = builder.build_map(&mut rng);

        app.insert_resource(map)
            .insert_resource(map_metadata)
            .add_system(index_map);
    }
}

pub fn index_map(
    mut map: ResMut<GameMap>,
    things: Query<(Entity, &Position)>,
    blockers: Query<&BlocksMovement>,
) {
    map.determine_blocked();
    map.clear_content_index();
    for (e, pos) in things.iter() {
        let idx = map
            .xy_to_idx(pos.x, pos.y)
            .expect("Entity {e:?} has a position outside the map: {pos:?}");

        if blockers.get(e).is_ok() {
            map.blocked[idx] = true;
        }

        map.tile_content[idx].push(e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xy_to_idx() {
        let map = GameMap::new(3, 4);

        assert_eq!(map.xy_to_idx(0, 0), Ok(0));
        assert_eq!(map.xy_to_idx(0, 3), Ok(9));
        assert_eq!(map.xy_to_idx(2, 0), Ok(2));
        assert_eq!(map.xy_to_idx(2, 3), Ok(11));
    }

    #[test]
    fn test_xy_to_idx_out_of_bounds() {
        let map = GameMap::new(3, 4);

        assert_eq!(map.xy_to_idx(3, 0), Err(OutsideMapError));
        assert_eq!(map.xy_to_idx(0, 4), Err(OutsideMapError));
        assert_eq!(map.xy_to_idx(13, 23), Err(OutsideMapError));
    }

    #[test]
    fn test_idx_to_xy() {
        let map = GameMap::new(3, 4);

        assert_eq!(map.idx_to_xy(0), Ok((0, 0)));
        assert_eq!(map.idx_to_xy(2), Ok((2, 0)));
        assert_eq!(map.idx_to_xy(11), Ok((2, 3)));
    }

    #[test]
    fn test_idx_to_xy_out_of_bounds() {
        let map = GameMap::new(3, 4);

        assert_eq!(map.idx_to_xy(12), Err(OutsideMapError));
        assert_eq!(map.idx_to_xy(200), Err(OutsideMapError));
        assert_eq!(map.idx_to_xy(usize::max_value()), Err(OutsideMapError));
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn test_map_too_large_1() {
        GameMap::new(u32::max_value(), 2);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn test_map_too_large_2() {
        GameMap::new(u32::max_value() / 2, 3);
    }

    #[cfg(debug_assertions)]
    #[test]
    #[should_panic]
    fn test_map_too_large_3() {
        GameMap::new(65536, 65536);
    }

    #[test]
    #[should_panic]
    fn test_map_degenerate_1() {
        GameMap::new(0, 10);
    }

    #[test]
    #[should_panic]
    fn test_map_degenerate_2() {
        GameMap::new(100, 0);
    }

    #[test]
    #[should_panic]
    fn test_map_too_large_release() {
        GameMap::new(65536, 65536);
    }
}
