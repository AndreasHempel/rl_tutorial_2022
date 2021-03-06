use rand::Rng;

use super::{
    rect::Rect,
    spawner::{fill_room, Spawnables},
    MapBuildData, MapModifier, MapRng, SpawnList,
};

const ROOMS_REQUIRED_ERROR: &str =
    "Room based spawning requires rooms to have been generated first!";

pub struct RoomBasedSpawner {
    max_spawns: u32,
}

impl RoomBasedSpawner {
    pub fn new(max_spawns: u32) -> Box<RoomBasedSpawner> {
        Box::new(RoomBasedSpawner { max_spawns })
    }

    fn spawn(&mut self, rng: &mut MapRng, rooms: &[Rect]) -> SpawnList {
        rooms
            .iter()
            .flat_map(|room| fill_room(rng, room, self.max_spawns))
            .collect()
    }
}

impl MapModifier for RoomBasedSpawner {
    fn modify_map(&mut self, rng: &mut MapRng, build_data: &mut MapBuildData) {
        let rooms = build_data
            .metadata
            .rooms
            .as_ref()
            .expect(ROOMS_REQUIRED_ERROR);
        build_data
            .metadata
            .spawn_list
            .extend(self.spawn(rng, rooms));
        build_data.take_snapshot();
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// TODO: Remove this lint-silencing once a use for [`RoomSelectionMode::Random`] has been found
#[allow(dead_code)]
pub enum RoomSelectionMode {
    First,
    Last,
    Random,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PositionSelectionMode {
    Center,
    Random,
}

/// Select the player's starting position from one of the available rooms
pub struct RoomBasedStartingPosition {
    room_mode: RoomSelectionMode,
    pos_mode: PositionSelectionMode,
}

impl RoomBasedStartingPosition {
    pub fn new(room_mode: RoomSelectionMode, pos_mode: PositionSelectionMode) -> Box<Self> {
        Box::new(Self {
            room_mode,
            pos_mode,
        })
    }
}

impl MapModifier for RoomBasedStartingPosition {
    fn modify_map(&mut self, rng: &mut MapRng, build_data: &mut MapBuildData) {
        let rooms = build_data
            .metadata
            .rooms
            .as_ref()
            .expect(ROOMS_REQUIRED_ERROR);
        let start_room = select_room(&self.room_mode, rooms, rng);
        // FIXME: This can select the same tile as another spawning MapModifier
        let start_pos = select_position(&self.pos_mode, start_room, rng);
        build_data.metadata.starting_position = Some(start_pos);
        build_data.take_snapshot();
    }
}

/// Select the player's starting position from one of the available rooms
pub struct RoomBasedObjectiveSpawner {
    room_mode: RoomSelectionMode,
    pos_mode: PositionSelectionMode,
    objective: Spawnables,
}

impl RoomBasedObjectiveSpawner {
    pub fn new(
        room_mode: RoomSelectionMode,
        pos_mode: PositionSelectionMode,
        objective: Spawnables,
    ) -> Box<Self> {
        Box::new(Self {
            room_mode,
            pos_mode,
            objective,
        })
    }
}

impl MapModifier for RoomBasedObjectiveSpawner {
    fn modify_map(&mut self, rng: &mut MapRng, build_data: &mut MapBuildData) {
        let rooms = build_data
            .metadata
            .rooms
            .as_ref()
            .expect(ROOMS_REQUIRED_ERROR);
        let room = select_room(&self.room_mode, rooms, rng);
        // FIXME: This can select the same tile as another spawning MapModifier
        let pos = select_position(&self.pos_mode, room, rng);
        build_data.metadata.spawn_list.insert(pos, self.objective);
        build_data.take_snapshot();
    }
}

fn select_room<'a>(mode: &RoomSelectionMode, rooms: &'a Vec<Rect>, rng: &mut MapRng) -> &'a Rect {
    let num_rooms = rooms.len();
    match mode {
        RoomSelectionMode::First => &rooms[0],
        RoomSelectionMode::Last => &rooms[num_rooms - 1],
        RoomSelectionMode::Random => &rooms[rng.gen_range(0..num_rooms)],
    }
}

fn select_position(mode: &PositionSelectionMode, room: &Rect, rng: &mut MapRng) -> (u32, u32) {
    match mode {
        PositionSelectionMode::Center => room.center(),
        PositionSelectionMode::Random => {
            let rx = rng.gen_range(room.x1 + 1..room.x2);
            let ry = rng.gen_range(room.y1 + 1..room.y2);
            (rx, ry)
        }
    }
}
