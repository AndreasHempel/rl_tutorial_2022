use crate::components::Position;

/// Assumes perfect vision through all obstacles up to `range` tiles away.
/// Treats diagonally adjacent tiles the same as orthogonally adjacent tiles.
#[allow(dead_code)]
fn square_xray_vision(position: &Position, range: u32) -> Vec<Position> {
    let mut visible = Vec::new();
    for i in -(range as i32)..(range as i32) {
        for j in -(range as i32)..(range as i32) {
            let t = Position {
                x: (position.x as i32 + i) as u32,
                y: (position.y as i32 + j) as u32,
            };
            visible.push(t);
        }
    }
    visible
}
