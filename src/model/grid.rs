use crate::model::tile::Tile;
use std::fmt;
use std::fmt::{Display, Formatter};

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
    pub fn get_tile(&self, index: usize) -> Result<Tile, CoordinateError> {
        match self.data.get(index) {
            Some(tile) => Ok(*tile),
            None => Err(CoordinateError::OutOfBounds { c: index }),
        }
    }

    pub fn set_tile(&mut self, index: usize, tile: Tile) {
        self.data[index] = tile;
    }

    pub fn get_n(&self) -> usize {
        self.n as usize
    }

    pub fn get_data(&self) -> &Vec<Tile> {
        &self.data
    }

    // Gives (col, row)
    pub fn split_coordinate(&self, c: usize) -> Result<(usize, usize), CoordinateError> {
        let n = self.n as usize;
        if c >= n.pow(2) {
            return Err(CoordinateError::OutOfBounds { c });
        }

        let col = c % n;
        let row = c.div_euclid(n);

        Ok((col, row))
    }

    pub fn merge_coordinate(&self, column: usize, row: usize) -> Result<usize, CoordinateError> {
        let n = self.n as usize;
        if column >= n || row >= n {
            return Err(CoordinateError::Invalid2DCoordinates { column, row, n });
        }

        Ok(row * n + column)
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new(10)
    }
}

#[derive(Debug, Clone)]
pub enum CoordinateError {
    OutOfBounds { c: usize },
    Invalid2DCoordinates { column: usize, row: usize, n: usize },
}

impl Display for CoordinateError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "coord error todo")
    }
}
