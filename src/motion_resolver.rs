use bevy::{prelude::*, utils::HashMap};

use crate::{
    components::Position,
    map::{GameMap, TileType},
};

/// Used to resolve [`MoveAttempt`]s and push blocking entities in the direction of motion.
#[derive(Debug, Default)]
pub struct MotionResolver {
    /// Stack of [`MoveAttempt`]s that need to be resolved
    moves: Vec<MoveAttempt>,
}

impl MotionResolver {
    /// Evaluates whether a given [`MoveAttempt`] is possible, recursively pushing other
    /// creatures out of the way if necessary (and possible). Updates the map appropriately
    /// if a [`MoveAttempt`] is deemed legal and leaves it unchanged if the motion is illegal
    pub fn resolve<F: Fn(Entity) -> bool>(
        mut self,
        mov: MoveAttempt,
        map: &mut GameMap,
        is_pushable: F,
    ) -> Result<HashMap<Entity, Position>, ()> {
        // Guard against 'fake' moves with zero displacement
        if mov.dx == 0 && mov.dy == 0 {
            let mut res = HashMap::new();
            res.insert(mov.entity, mov.from);
            return Ok(res);
        }
        match mov.is_legal(map) {
            MoveStatus::Illegal => Err(()),
            MoveStatus::Legal => {
                self.moves.push(mov);
                Ok(self.commit(map))
            }
            MoveStatus::RequiresPush(e) => {
                // Check if the blocking entity can be pushed in the same direction as the attempted motion
                if is_pushable(e) {
                    let next = MoveAttempt {
                        entity: e,
                        from: mov.from + (mov.dx, mov.dy),
                        dx: mov.dx,
                        dy: mov.dy,
                    };
                    self.moves.push(mov);
                    self.resolve(next, map, is_pushable)
                } else {
                    Err(())
                }
            }
        }
    }

    /// Updates the [`GameMap`] based on the internal stack of required [`MoveAttempt`]s that have been evaluated as legal
    ///
    /// NB: This assumes all motions are valid - checks have been performed when calling [`MoveAttempt::is_legal()`]
    fn commit(self, map: &mut GameMap) -> HashMap<Entity, Position> {
        self.moves
            .iter()
            // Work this backwards like a stack
            .rev()
            .map(|mov| {
                let from = &mov.from;
                let to = *from + (mov.dx, mov.dy);
                map.move_entity_unchecked((from.x, from.y), (to.x, to.y), mov.entity);
                (mov.entity, to)
            })
            .collect()
    }
}

/// Represents an [`Entity`] wanting to move by a given displacement from a starting [`Position`]
#[derive(Debug)]
pub struct MoveAttempt {
    pub entity: Entity,
    pub from: Position,
    pub dx: i32,
    pub dy: i32,
}

/// Indicates if a [`MoveAttempt`] is possible or not
#[derive(Debug)]
enum MoveStatus {
    /// Motion is not possible, e.g. because it is blocked and blocker cannot be pushed out of the way
    Illegal,
    /// Blocking entity must be pushed away
    RequiresPush(Entity),
    /// Motion is possible, i.e. the target tile is free of obstacles
    Legal,
}

impl MoveAttempt {
    /// Checks if this [`MoveAttempt`] is possible or not
    fn is_legal(&self, map: &GameMap) -> MoveStatus {
        let from = &self.from;
        let to = *from + (self.dx, self.dy);
        if let (Ok(from_idx), Ok(to_idx)) =
            (map.xy_to_idx(from.x, from.y), map.xy_to_idx(to.x, to.y))
        {
            if map.tiles[to_idx] == TileType::Wall {
                return MoveStatus::Illegal;
            }
            if map.tile_content[from_idx].iter().any(|&e| e == self.entity) {
                if let Some(e) = map.blocked_by[to_idx] {
                    MoveStatus::RequiresPush(e)
                } else {
                    MoveStatus::Legal
                }
            } else {
                MoveStatus::Illegal
            }
        } else {
            MoveStatus::Illegal
        }
    }
}
