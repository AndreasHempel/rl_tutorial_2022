use bevy::prelude::*;

/// Available tile types
#[derive(Debug, Clone, PartialEq, Component)]
pub enum TileType {
    Floor,
    Wall,
}

/// Represents the game Map
pub struct GameMap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<TileType>,
}

impl Default for GameMap {
    fn default() -> Self {
        let width = 80u32;
        let height = 53u32;
        let mut map = GameMap::new(width, height);
        const WALL_TILES: [(u32, u32); 4] = [(13, 14), (14, 14), (15, 14), (18, 17)];
        WALL_TILES
            .iter()
            .map(|(x, y)| {
                let idx = map.xy_to_idx(*x, *y).unwrap();
                map.tiles[idx] = TileType::Wall;
            })
            .count();
        map
    }
}

#[derive(Debug, PartialEq)]
pub struct OutsideMapError;

impl GameMap {
    /// Create a new, empty map with the given dimensions
    pub fn new(width: u32, height: u32) -> Self {
        GameMap {
            width,
            height,
            tiles: vec![TileType::Floor; (width * height) as usize],
        }
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
    fn xy_to_idx(&self, x: u32, y: u32) -> Result<usize, OutsideMapError> {
        if x >= self.width || y >= self.height {
            return Err(OutsideMapError);
        }
        Ok((self.width * y + x) as usize)
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameMap::default());
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

    #[test]
    #[should_panic]
    fn test_map_too_large_1() {
        GameMap::new(u32::max_value(), 2);
    }

    #[test]
    #[should_panic]
    fn test_map_too_large_2() {
        GameMap::new(u32::max_value() / 2, 3);
    }

    #[test]
    #[should_panic]
    fn test_map_too_large_3() {
        GameMap::new(65536, 65536);
    }
}
