use bevy::log::*;
use rand::Rng;
use std::cmp::{max, min};

use super::{rect::Rect, InitialMapBuilder, MapBuildData, MapRng};
use crate::map::{GameMap, TileType};

pub struct SimpleMapBuilder {
    max_rooms: u32,
    min_size: u32,
    max_size: u32,
}

impl InitialMapBuilder for SimpleMapBuilder {
    fn build_map(&mut self, rng: &mut MapRng, build_data: &mut MapBuildData) {
        self.rooms_and_corridors(rng, build_data);
    }
}

impl SimpleMapBuilder {
    pub fn new(max_rooms: u32, min_size: u32, max_size: u32) -> Box<SimpleMapBuilder> {
        Box::new(SimpleMapBuilder {
            max_rooms,
            min_size,
            max_size,
        })
    }

    fn rooms_and_corridors(&mut self, rng: &mut MapRng, build_data: &mut MapBuildData) {
        let mut rooms = Vec::new();

        for _ in 0..self.max_rooms {
            let w = rng.gen_range(self.min_size..=self.max_size);
            let h = rng.gen_range(self.min_size..=self.max_size);
            let x = rng.gen_range(0..build_data.map.width - w - 1);
            let y = rng.gen_range(0..build_data.map.height - h - 1);
            let new_room = Rect::new(x, y, w, h);
            let ok = !rooms.iter().any(|r| new_room.intersect(&r));
            if ok {
                apply_room_to_map(&mut build_data.map, &new_room);
                if !rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = rooms[rooms.len() - 1].center();
                    apply_tunnel(
                        &mut build_data.map,
                        prev_x,
                        prev_y,
                        new_x,
                        new_y,
                        rng.gen_bool(0.5),
                    );
                }

                rooms.push(new_room);
                build_data.take_snapshot();
            }
        }
        build_data.metadata.rooms = Some(rooms);
    }
}

/// Marks all tiles strictly inside the given room as floor tiles
fn apply_room_to_map(map: &mut GameMap, room: &Rect) {
    for y in room.y1 + 1..room.y2 {
        for x in room.x1 + 1..room.x2 {
            if let Ok(idx) = map.xy_to_idx(x, y) {
                map.tiles[idx] = TileType::Floor;
            } else {
                warn!(
                    "Cannot make {x}, {y} part of a room, it is outside the map ({} x {})!",
                    map.width, map.height
                );
            }
        }
    }
}

/// Carves a dogleg path of floor tiles into the map from (x1, y1) to (x2, y2)
fn apply_tunnel(map: &mut GameMap, x1: u32, y1: u32, x2: u32, y2: u32, horizontal_first: bool) {
    if horizontal_first {
        apply_horizontal_tunnel(map, x1, x2, y1);
        apply_vertical_tunnel(map, y1, y2, x2);
    } else {
        apply_vertical_tunnel(map, y1, y2, x1);
        apply_horizontal_tunnel(map, x1, x2, y2);
    }
}

fn apply_horizontal_tunnel(map: &mut GameMap, x1: u32, x2: u32, y: u32) {
    for x in min(x1, x2)..=max(x1, x2) {
        if let Ok(idx) = map.xy_to_idx(x, y) {
            map.tiles[idx] = TileType::Floor;
        } else {
            warn!("Cannot make {x}, {y} part of a tunnel, it is outside the map {map:?}!");
        }
    }
}

fn apply_vertical_tunnel(map: &mut GameMap, y1: u32, y2: u32, x: u32) {
    for y in min(y1, y2)..=max(y1, y2) {
        if let Ok(idx) = map.xy_to_idx(x, y) {
            map.tiles[idx] = TileType::Floor;
        } else {
            warn!("Cannot make {x}, {y} part of a tunnel, it is outside the map {map:?}!");
        }
    }
}
