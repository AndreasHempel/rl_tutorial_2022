use crate::map::{GameMap, MapMetadata};

pub type MapRng = rand::rngs::StdRng;

pub mod arbitrary_starting_point;
pub mod cellular_builder;
pub mod rect;
pub mod simple_map_builder;

/// Combines abstract map properties, the concrete tile layout, and potentially a history of snapshots
pub struct MapBuildData {
    map: GameMap,
    metadata: MapMetadata,
    pub history: Vec<(GameMap, MapMetadata)>,
}

impl MapBuildData {
    /// Adds a snapshot of the current map state to the history
    pub fn take_snapshot(&mut self) {
        let snapshot = self.map.clone();
        self.history.push((snapshot, self.metadata.clone()));
    }
}

/// Interface to build an initial base map. There can only be a single [InitialMapBuilder] in a [BuilderChain].
pub trait InitialMapBuilder: Send + Sync {
    fn build_map(&mut self, rng: &mut MapRng, build_data: &mut MapBuildData);
}

/// Interface for 'map modifiers' that may be applied in a stack.
pub trait MapModifier: Send + Sync {
    fn modify_map(&mut self, rng: &mut MapRng, build_data: &mut MapBuildData);
}

pub struct Uninitialized;
pub struct HasInitial {
    builder: Box<dyn InitialMapBuilder>,
}

pub trait InitialMapBuilderTrait {}
impl InitialMapBuilderTrait for Uninitialized {}
impl InitialMapBuilderTrait for HasInitial {}

/// Collects a sequence of map building steps with at most one [InitialMapBuilder] and multiple [MapModifier]s
pub struct BuilderChain<Initialized: InitialMapBuilderTrait> {
    initial: Initialized,
    modifiers: Vec<Box<dyn MapModifier>>,
    build_data: MapBuildData,
}

const WIDTH: u32 = 80;
const HEIGHT: u32 = 53;

impl BuilderChain<Uninitialized> {
    pub fn new() -> BuilderChain<Uninitialized> {
        BuilderChain {
            initial: Uninitialized,
            modifiers: Vec::new(),
            build_data: MapBuildData {
                map: GameMap::new(WIDTH, HEIGHT),
                metadata: MapMetadata::default(),
                history: Vec::new(),
            },
        }
    }

    pub fn start_with(self, initial: Box<dyn InitialMapBuilder>) -> BuilderChain<HasInitial> {
        BuilderChain {
            initial: HasInitial { builder: initial },
            modifiers: self.modifiers,
            build_data: self.build_data,
        }
    }
}

impl<I: InitialMapBuilderTrait> BuilderChain<I> {
    pub fn with(&mut self, builder: Box<dyn MapModifier>) -> &mut Self {
        self.modifiers.push(builder);
        self
    }
}

impl BuilderChain<HasInitial> {
    pub fn build_map(mut self, rng: &mut MapRng) -> (GameMap, MapMetadata) {
        self.initial.builder.build_map(rng, &mut self.build_data);

        for builder in self.modifiers.iter_mut() {
            builder.modify_map(rng, &mut self.build_data);
        }

        (self.build_data.map, self.build_data.metadata)
    }
}
