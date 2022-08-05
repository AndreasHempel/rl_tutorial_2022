use bevy::prelude::*;
use rand::prelude::StdRng;

use crate::{
    components::{Player, Position},
    map::GameMap,
    map_builder::MapMetadata,
    GameState,
};

/// Plugin responsible for level generation and cleanup
pub struct LevelPlugin {
    pub builder: MapBuilder,
    pub seed: u64,
}

/// Settings used for level generation
pub struct LevelSettings {
    /// [`MapBuilder`] to use for level generation (identical for all levels)
    pub builder: MapBuilder,
    /// Seed to use when restarting game (to allow a second try upon failing)
    pub original_seed: u64,
}

/// Available builder configs to choose from the command line
#[derive(Debug, clap::ValueEnum, Clone, Copy)]
pub enum MapBuilder {
    Rooms,
    Cellular,
}

/// System labels used for system ordering
#[derive(Debug, PartialEq, Eq, Hash, Clone, SystemLabel)]
enum SystemLabels {
    GenerateLevel,
}

/// Newtype wrapping the RNG used for level generation
pub struct MapRNG(pub StdRng);

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSettings {
            builder: self.builder,
            original_seed: self.seed,
        })
        // Insert dummy map data to make sure the resource exists
        .insert_resource(GameMap::new(1, 1))
        .insert_resource(MapMetadata::default())
        .add_system_set(
            SystemSet::on_enter(GameState::EnterNewLevel)
                .with_system(generate_level)
                .label(SystemLabels::GenerateLevel),
        )
        .add_system_set(
            SystemSet::on_enter(GameState::EnterNewLevel)
                .with_system(despawn_map_entities)
                .before(SystemLabels::GenerateLevel),
        );
    }
}

/// Generates a map and performs other setup steps necessary upon entering a level
fn generate_level(
    lvl_settings: Res<LevelSettings>,
    mut rng: ResMut<MapRNG>,
    mut res_map: ResMut<GameMap>,
    mut res_map_metadata: ResMut<MapMetadata>,
    mut state: ResMut<State<GameState>>,
) {
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
        match lvl_settings.builder {
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
                let mut builder =
                    builder.start_with(CellularAutomataBuilder::new(10, 0.4, vec![0, 5, 6, 7, 8]));
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
    let (map, map_metadata) = builder.build_map(&mut rng.0);

    *res_map = map;
    *res_map_metadata = map_metadata;

    state
        .set(GameState::WaitingForPlayer)
        .expect("Failed to wait for player after generating a new level!");
}

/// Remove all entities on the current map
fn despawn_map_entities(
    things: Query<Entity, (With<Position>, Without<Player>)>,
    mut commands: Commands,
) {
    for e in things.iter() {
        commands.entity(e).despawn();
    }
}
