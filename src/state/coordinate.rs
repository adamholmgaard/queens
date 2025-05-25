use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Default)]
pub struct CoordinateError {
    row: usize,
    col: usize,
    n: u8,
}

impl Display for CoordinateError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "coord error todo")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinate {
    raw: usize,
}

impl Coordinate {
    pub fn from_context(col: usize, row: usize, n: u8) -> Result<Self, CoordinateError> {
        if col >= n as usize || row >= n as usize {
            return Err(CoordinateError::default());
        }

        Ok(Self {
            raw: col * n as usize + row,
        })
    }

    pub fn from_context_signed(col: isize, row: isize, n: u8) -> Result<Self, CoordinateError> {
        if col < 0 || row < 0 {
            return Err(CoordinateError::default());
        }

        Self::from_context(col as usize, row as usize, n)
    }

    pub fn get(&self) -> usize {
        self.raw
    }

    pub fn to_coords(self, n: usize) -> Result<(usize, usize), CoordinateError> {
        if self.raw >= n.pow(2) {
            return Err(CoordinateError::default());
        }

        let row = self.raw.div_euclid(n);
        let col = self.raw % n;

        Ok((row, col))
    }
}

impl From<usize> for Coordinate {
    fn from(raw: usize) -> Self {
        Self { raw }
    }
}

impl From<Coordinate> for usize {
    // maybe not? since we have get()
    fn from(coord: Coordinate) -> Self {
        coord.raw
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}
