use crate::model::coordinate::{Coordinate, CoordinateError};
use crate::model::game_error::GameError;
use crate::model::grid::Grid;
use crate::model::layout::{complex_layout, Layout};
use crate::model::tile::Tile;
use log::warn;

#[derive(Clone)]
pub struct State {
    //  gamestate : enum{ingame, won, lost, pregame...}
    grid: Grid,
    layout: Layout,
}

impl State {
    pub fn get_n(&self) -> usize {
        self.grid.get_n()
    }

    pub fn get_grid(&self) -> Grid {
        self.grid.clone()
    }

    pub fn get_layout(&self) -> Layout {
        self.layout.clone()
    }

    pub fn get_tile(&self, x: Coordinate) -> Tile {
        self.grid.get_tile(x).expect("no tile found")
    }

    pub fn set_tile(&mut self, x: Coordinate, tile: Tile) {
        self.grid.set_tile(x, tile);
    }

    pub fn get_game_status(&self) -> (Vec<GameError>, bool) {
        let n = self.get_n();
        let mut errors = Vec::new();
        let mut rows = Vec::new();
        let mut cols = Vec::new();
        let mut colors = Vec::new();

        for (index, tile) in self.grid.get_data().iter().enumerate() {
            if tile.is_set() {
                let color = tile.get_color();
                let (row, col) = Coordinate::from(index)
                    .to_coords(n)
                    .expect("could not convert");

                if rows.contains(&row) {
                    errors.push(GameError::Row { row })
                } else {
                    rows.push(row);
                }

                if cols.contains(&col) {
                    errors.push(GameError::Column { col })
                } else {
                    cols.push(col);
                }

                if colors.contains(&color) {
                    errors.push(GameError::Area {
                        area: self
                            .layout
                            .get_area(
                                self.grid
                                    .clone()
                                    .coord_from_indices(row, col)
                                    .expect("Index out of bounds"),
                            )
                            .expect("Index out of bounds"),
                    })
                } else {
                    colors.push(color)
                }

                if index % n != n - 1 {
                    // is not all the way to the right
                    let below_right = index + n + 1;
                    if self
                        .grid
                        .get_tile(Coordinate::from(below_right))
                        .is_ok_and(|t| t.is_set())
                    {
                        errors.push(GameError::Diagonal {
                            c1: Coordinate::from(index),
                            c2: Coordinate::from(below_right),
                        })
                    }
                }

                if index % n != 0 {
                    // is not all the way to the left
                    let below_left = index + n - 1;
                    if self
                        .grid
                        .get_tile(Coordinate::from(below_left))
                        .is_ok_and(|t| t.is_set())
                    {
                        errors.push(GameError::Diagonal {
                            c1: Coordinate::from(index),
                            c2: Coordinate::from(below_left),
                        })
                    }
                }
            }
        }

        (
            errors.clone(),
            (rows.len() == n)
                && (cols.len() == n)
                && (colors.len() == n)
                && errors.clone().is_empty(),
        )
    }
}

impl Default for State {
    fn default() -> Self {
        let mut grid = Grid::default();
        let layout = Layout::default();

        for area in layout.get_areas() {
            for index in area.get_sections().clone() {
                grid.set_tile(Coordinate::from(index), Tile::new(false, area.get_color()));
            }
        }

        Self { grid, layout }
    }
}
