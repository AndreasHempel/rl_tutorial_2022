use rand::Rng;
use std::collections::hash_map::Entry;

use super::{random_table::RandomTable, rect::Rect, MapRng, SpawnList};

/// All things that can be spawned onto a map
#[derive(Clone, Copy, Debug)]
pub enum Spawnables {
    TreasureChest,
    Turtle,
    APCrystal,
}

fn spawn_table() -> RandomTable<Spawnables> {
    use Spawnables::*;
    RandomTable::new().add(Turtle, 10).add(APCrystal, 200)
}

pub fn fill_room(rng: &mut MapRng, room: &Rect, max_spawns: u32) -> SpawnList {
    let mut room_tiles = Vec::with_capacity((room.width() * room.height()) as usize);
    for x in room.x1 + 1..room.x2 {
        for y in room.y1 + 1..room.y2 {
            room_tiles.push((x, y));
        }
    }

    fill_region(rng, &room_tiles, max_spawns)
}

/// Randomly selects spawn points from a given list of valid spawn positions (`region`)
pub fn fill_region(rng: &mut MapRng, region: &[(u32, u32)], max_spawns: u32) -> SpawnList {
    let mut spawn_points = SpawnList::new();
    if region.is_empty() {
        return spawn_points;
    }
    let spawn_table = spawn_table();

    {
        let num_spawns = rng.gen_range(0..=max_spawns);

        // Select spawn points from the given region
        for _i in 0..num_spawns {
            let mut added = false;
            let mut tries = 0;
            while !added && tries < 20 {
                let idx = rng.gen_range(0..region.len());
                let pos = region[idx];
                if let Entry::Vacant(e) = spawn_points.entry(pos) {
                    e.insert(spawn_table.roll(rng).unwrap());
                    added = true;
                } else {
                    tries += 1;
                }
            }
        }
    }

    spawn_points
}
