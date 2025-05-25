use crate::model::coordinate::{Coordinate, CoordinateError};
use crate::model::tile::Tile;

// 0-indexed square matrix of tiles.
#[derive(Clone, Debug)]
pub struct Grid {
    data: Vec<Tile>,
    n: u8,
}

impl Grid {
    pub fn new(n: u8) -> Grid {
        let mut data = Vec::new();
        data.resize(n.pow(2) as usize, Tile::default());

        Grid { data, n }
    }
    pub fn get_tile(&self, index: Coordinate) -> Result<Tile, CoordinateError> {
        match self.data.get(index.get()) {
            Some(tile) => Ok(*tile),
            None => Err(CoordinateError::default()),
        }
    }

    pub fn get_tile_by_indices(&self, row: isize, col: isize) -> Result<Tile, CoordinateError> {
        // fix the usize underflow problem!!
        if row < 0 {
            return Err(CoordinateError::default());
        }
        if col < 0 {
            return Err(CoordinateError::default());
        }

        let index = self
            .clone()
            .coord_from_indices(row as usize, col as usize)?;
        self.get_tile(index)
    }

    pub fn set_tile(&mut self, index: Coordinate, tile: Tile) {
        self.data[index.get()] = tile;
    }

    pub fn get_n(&self) -> usize {
        self.n as usize
    }

    pub fn get_data(&self) -> &Vec<Tile> {
        &self.data
    }

    pub fn coord_from_indices(self, col: usize, row: usize) -> Result<Coordinate, CoordinateError> {
        Coordinate::from_context(col, row, self.n)
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(10)
    }
}
