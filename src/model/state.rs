use crate::model::game_error::GameError;
use crate::model::grid::{CoordinateError, Grid};
use crate::model::layout::{complex_layout, Layout};
use crate::model::tile::Tile;
use log::{debug, warn};

#[derive(Clone)]
pub struct State {
    //  gamestate : enum{ingame, won, lost, pregame...}
    grid: Grid,
    layout: Layout,
    marked: Option<usize>,
}

impl State {
    pub fn get_n(&self) -> usize {
        self.grid.get_n()
    }

    pub fn get_grid(&self) -> Grid {
        self.grid.clone()
    }

    pub fn get_marked(&self) -> Option<usize> {
        self.marked
    }

    pub fn set_marked(&mut self, marked: Option<usize>) {
        self.marked = marked;
    }

    pub fn get_layout(&self) -> Layout {
        self.layout.clone()
    }

    pub fn get_tile(&self, x: usize) -> Result<Tile, CoordinateError> {
        self.grid.get_tile(x)
    }

    pub fn set_tile(&mut self, x: usize, tile: Tile) {
        self.grid.set_tile(x, tile);
    }

    pub fn flip_tile(&mut self, c: usize) {
        match self.grid.get_tile(c) {
            Ok(tile) => {
                debug!("on_tile_click ({})", c);
                self.grid.set_tile(c, tile.on_click());
            }
            Err(err) => {
                warn!("{:?}", err);
            }
        }
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
                let (col, row) = self
                    .grid
                    .split_coordinate(index)
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
                                    .merge_coordinate(col, row)
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
                    if self.grid.get_tile(below_right).is_ok_and(|t| t.is_set()) {
                        errors.push(GameError::Diagonal {
                            c1: index,
                            c2: below_right,
                        })
                    }
                }

                if index % n != 0 {
                    // is not all the way to the left
                    let below_left = index + n - 1;
                    if self.grid.get_tile(below_left).is_ok_and(|t| t.is_set()) {
                        errors.push(GameError::Diagonal {
                            c1: index,
                            c2: below_left,
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
                grid.set_tile(index, Tile::new(false, area.get_color()));
            }
        }

        Self {
            grid,
            layout,
            marked: None,
        }
    }
}
