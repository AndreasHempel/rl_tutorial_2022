use bevy::prelude::*;

use crate::components::{BlocksMovement, Position};

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
    /// Lists the [Entities](Entity) which are blocking the corresponding tile
    pub blocked_by: Vec<Option<Entity>>,
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
            blocked_by: vec![None; size],
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
            self.blocked_by[i] = None;
        }
    }

    /// Forgets all indexed entities
    fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }

    /// Updates a maps state by moving an entity. Panics if the given positions are illegal
    /// or the entity ID is unknown in the original tile. Also performs no checks if the target
    /// tile is blocking motion or not.
    pub fn move_entity_unchecked(
        &mut self,
        (x1, y1): (u32, u32),
        (x2, y2): (u32, u32),
        entity: Entity,
    ) {
        let from_idx = self
            .xy_to_idx(x1, y1)
            .expect("Original position outside map.");
        let to_idx = self.xy_to_idx(x2, y2).expect("Goal position outside map.");
        let in_vec_idx = self.tile_content[from_idx]
            .iter()
            .position(|e| e == &entity)
            .expect("Entity not found in original tile.");
        let e = self.tile_content[from_idx].swap_remove(in_vec_idx);
        self.tile_content[to_idx].push(e);
        if Some(e) == self.blocked_by[from_idx] {
            // The moving entity is blocking its current tile
            self.blocked[from_idx] = false;
            self.blocked_by[from_idx] = None;

            self.blocked[to_idx] = true;
            self.blocked_by[to_idx] = Some(e);
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
        use crate::map_builder::{
            arbitrary_starting_point::ArbitraryStartingPoint,
            cellular_builder::CellularAutomataBuilder,
            cull_unreachable::CullUnreachable,
            general_objective_spawner::GeneralObjectiveSpawner,
            region_based_builders::{DistanceFunction, RegionBasedSpawner, VoronoiRegion},
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
                    // First add a starting point
                    builder.with(ArbitraryStartingPoint::new());
                    // Then remove unreachable squares
                    builder.with(CullUnreachable::new());
                    // Make sure that a treasure chest is spawned
                    builder.with(GeneralObjectiveSpawner::new(Spawnables::TreasureChest));
                    // Split the tiles into regions
                    builder.with(VoronoiRegion::new(10, DistanceFunction::Manhattan));
                    // Spawn monsters into the regions
                    builder.with(RegionBasedSpawner::new(3));
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

fn index_map(
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
            // NB: This assumes there can only be a single blocking entity per tile which should be true by construction
            map.blocked[idx] = true;
            map.blocked_by[idx] = Some(e);
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
