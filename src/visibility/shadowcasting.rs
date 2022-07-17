//! Rust implementation of Albert Ford's symmetric shadowcasting algorithm
//! Please see [https://www.albertford.com/shadowcasting/](https://www.albertford.com/shadowcasting/) for an excellent discussion
//! with references, great visualizations, and a Python version
//! Used under the CC0 license:
//! [https://github.com/370417/symmetric-shadowcasting/blob/master/LICENSE.txt](https://github.com/370417/symmetric-shadowcasting/blob/master/LICENSE.txt)

use crate::components::Position;

/// Cardinal directions describing the four [`Quadrants`](Quadrant) for which the algorithm is applied
enum CardinalDirection {
    North,
    East,
    South,
    West,
}

/// A [`Quadrant`] represents a rectangular conic sector pointing in a [`CardinalDirection`]
struct Quadrant {
    sector: CardinalDirection,
    origin: Position,
}

impl Quadrant {
    /// Create a new [`Quadrant`] pointing from the given `origin` into the given [`CardinalDirection`]
    fn new(sector: CardinalDirection, origin: Position) -> Self {
        Self { sector, origin }
    }

    /// Maps a (row, col) tuple in the current [`Quadrant`] into an (x,y) position in the map.
    /// This maps the unsigned row index and the signed column index with this quadrant's origin
    /// to an unsigned (x, y) tuple referencing a tile in 'world coordinates'.
    fn to_map(&self, row: u32, col: i32) -> Position {
        let add_signed = |a: u32, b: i32| -> u32 {
            if !b.is_negative() {
                a + b as u32
            } else {
                a - b.unsigned_abs()
            }
        };
        match self.sector {
            // FIXME: The u32 operations below may underflow for origins close to the map boundary
            CardinalDirection::North => {
                Position::new(add_signed(self.origin.x, col), self.origin.y + row)
            }
            CardinalDirection::South => {
                Position::new(add_signed(self.origin.x, col), self.origin.y - row)
            }
            CardinalDirection::East => {
                Position::new(self.origin.x + row, add_signed(self.origin.y, col))
            }
            CardinalDirection::West => {
                Position::new(self.origin.x - row, add_signed(self.origin.y, col))
            }
        }
    }
}

/// Represents a contiguous stretch of unblocked tiles between a start and end slope
/// at a given depth from a [`Quadrant's`](Quadrant) origin
struct Row {
    depth: u32,
    start_slope: f32,
    end_slope: f32,
}

impl Row {
    /// Returns a new [`Row`] (sector) defined by its depth and spanned by the given start and end slopes
    fn new(depth: u32, start_slope: f32, end_slope: f32) -> Self {
        Self {
            depth,
            start_slope,
            end_slope,
        }
    }

    /// Returns the next [`Row`]
    fn next(&self) -> Row {
        Row::new(self.depth + 1, self.start_slope, self.end_slope)
    }

    /// Returns all tiles belonging to this [`Row`]
    fn tiles(&self) -> Vec<(u32, i32)> {
        let mut t = Vec::new();
        let min_col = round_ties_up(self.depth as f32 * self.start_slope);
        let max_col = round_ties_down(self.depth as f32 * self.end_slope);
        for col in min_col..=max_col {
            t.push((self.depth, col));
        }
        t
    }
}

/// Computes the slope of a line from the origin through the tile given by (row_depth, col)
fn slope(row_depth: u32, col: i32) -> f32 {
    (2.0 * col as f32 - 1.0) / (2.0 * row_depth as f32)
}

/// Return `true` if a given floor tile can be seen symmetrically from the origin, i.e. if the central point
/// of the tile is in the sector swept out by the row’s start and end slopes. Otherwise, it returns `false`s.
fn is_symmetric(row: &Row, col: i32) -> bool {
    (col as f32 >= row.depth as f32 * row.start_slope)
        && (col as f32 <= row.depth as f32 * row.end_slope)
}

/// Rounds to nearest integer, but decides ties towards infinity (instead of away from zero as [f32::round()])
fn round_ties_up(n: f32) -> i32 {
    (n + 0.5).floor() as i32
}

/// Rounds to nearest integer, but decides ties towards minus infinity (instead of away from zero as [f32::round()])
fn round_ties_down(n: f32) -> i32 {
    n.round() as i32
}

/// Returns all tiles that are visible from the given `origin` up to `max_distance`
pub fn compute_fov<F>(origin: &Position, is_blocking: F, max_distance: u32) -> Vec<Position>
where
    F: Fn(Position) -> bool,
{
    let mut visible = vec![*origin];

    for dir in [
        CardinalDirection::North,
        CardinalDirection::East,
        CardinalDirection::South,
        CardinalDirection::West,
    ] {
        // Assume the origin is always visible
        let quadrant = Quadrant::new(dir, *origin);

        // Only reveals tiles that are within the given max distance
        let mut reveal = |x: u32, y: i32| {
            let y_abs = y.unsigned_abs();
            if (x * x + y_abs * y_abs) <= max_distance * max_distance {
                visible.push(quadrant.to_map(x, y));
            }
        };

        let is_wall = |tile| {
            if let Some((row, col)) = tile {
                let pos = quadrant.to_map(row, col);
                is_blocking(pos)
            } else {
                false
            }
        };

        let is_floor = |tile| {
            if let Some((row, col)) = tile {
                let pos = quadrant.to_map(row, col);
                !is_blocking(pos)
            } else {
                false
            }
        };

        let mut scan_iterative = |row: Row| {
            let mut rows = vec![row];

            while let Some(mut row) = rows.pop() {
                if row.depth > max_distance {
                    // No tile in this row can be visible -> skip processing
                    continue;
                }
                let mut prev_tile = None;
                for (x, y) in row.tiles() {
                    if is_wall(Some((x, y))) || is_symmetric(&row, y) {
                        reveal(x, y);
                    }
                    if is_wall(prev_tile) && is_floor(Some((x, y))) {
                        row.start_slope = slope(x, y);
                    }
                    if is_floor(prev_tile) && is_wall(Some((x, y))) {
                        let mut next_row = row.next();
                        next_row.end_slope = slope(x, y);
                        rows.push(next_row);
                    }
                    prev_tile = Some((x, y));
                }
                if is_floor(prev_tile) {
                    rows.push(row.next());
                }
            }
        };

        let first_row = Row::new(1, -1.0, 1.0);
        scan_iterative(first_row);
    }

    visible
}
