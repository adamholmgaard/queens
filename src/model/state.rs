use crate::errors::QueensResult;
use crate::model::game_rule_broken::GameRuleBroken;
use crate::model::grid::Grid;
use crate::model::layout::{Layout, LayoutType};
use crate::model::tile::Tile;

#[derive(Clone)]
pub struct State {
    n: usize,
    layout_type: LayoutType,
    game_state: GameState,
}

#[derive(Clone, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame(InGameState),
    Won,
}

impl State {
    pub fn get_game_state(&self) -> &GameState {
        &self.game_state
    }
    pub fn load_in_game(&mut self) {
        self.game_state = GameState::InGame(InGameState::new(self.n, self.layout_type.clone()));
    }

    pub fn set_game_won(&mut self) {
        self.game_state = GameState::Won;
    }

    pub fn get_layout_type(&self) -> &LayoutType {
        &self.layout_type
    }

    pub fn set_layout_type(&mut self, layout_type: LayoutType) {
        self.layout_type = layout_type;
    }

    pub fn set_n(&mut self, n: usize) {
        self.n = n;
    }

    pub fn get_n(&self) -> usize {
        self.n
    }

    pub fn in_game_mut(&mut self) -> &mut InGameState {
        match &mut self.game_state {
            GameState::InGame(ref mut std) => std,
            _ => panic!("game is not ingame"),
        }
    }

    pub fn in_game(&self) -> &InGameState {
        match &self.game_state {
            GameState::InGame(std) => std,
            _ => panic!("game is not ingame"),
        }
    }

    pub fn get_grid(&self) -> Grid {
        self.in_game().get_grid()
    }

    pub fn get_marked(&self) -> Option<usize> {
        self.in_game().get_marked()
    }

    pub fn set_marked(&mut self, marked: Option<usize>) {
        self.in_game_mut().set_marked(marked)
    }

    pub fn get_layout(&self) -> Layout {
        self.in_game().get_layout()
    }

    pub fn get_tile(&self, x: usize) -> QueensResult<Tile> {
        self.in_game().get_tile(x)
    }

    pub fn set_tile(&mut self, x: usize, tile: Tile) {
        self.in_game_mut().set_tile(x, tile)
    }

    pub fn flip_tile(&mut self, c: usize) -> QueensResult<()> {
        self.in_game_mut().flip_tile(c)
    }

    // Get the list of game errors and whether the game has been won.
    pub fn get_win_status(&self) -> QueensResult<(Vec<GameRuleBroken>, bool)> {
        self.in_game().get_win_status(self.n)
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            n: 10,
            layout_type: LayoutType::default(),
            game_state: GameState::default(),
        }
    }
}

#[derive(Clone)]
pub struct InGameState {
    grid: Grid,
    layout: Layout,
    marked: Option<usize>,
}

impl InGameState {
    pub fn new(n: usize, layout_type: LayoutType) -> InGameState {
        let mut grid = Grid::new(n);
        let layout = match layout_type {
            LayoutType::Easy => Layout::easy_layout(n),
            LayoutType::Complex => Layout::complex_layout(n),
            LayoutType::Generated => Layout::generate_layout(n),
        };

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

    pub fn get_tile(&self, x: usize) -> QueensResult<Tile> {
        self.grid.get_tile(x)
    }

    pub fn set_tile(&mut self, x: usize, tile: Tile) {
        self.grid.set_tile(x, tile);
    }

    pub fn flip_tile(&mut self, c: usize) -> QueensResult<()> {
        self.grid.get_tile(c).and_then(|tile| {
            self.grid.set_tile(c, tile.on_click());
            Ok(())
        })
    }

    // Get the list of game errors and whether the game has been won.
    pub fn get_win_status(&self, n: usize) -> QueensResult<(Vec<GameRuleBroken>, bool)> {
        let mut errors = Vec::new();
        let mut rows = Vec::new();
        let mut cols = Vec::new();
        let mut colors = Vec::new();

        for (index, tile) in self.grid.get_data().iter().enumerate() {
            if tile.is_set() {
                let color = tile.get_color();
                let (col, row) = self.grid.split_coordinate(index)?;

                if rows.contains(&row) {
                    errors.push(GameRuleBroken::Row { row })
                } else {
                    rows.push(row);
                }

                if cols.contains(&col) {
                    errors.push(GameRuleBroken::Column { col })
                } else {
                    cols.push(col);
                }

                if colors.contains(&color) {
                    errors.push(GameRuleBroken::Area {
                        area: self
                            .layout
                            .get_area(self.grid.merge_coordinate(col, row)?)?,
                    })
                } else {
                    colors.push(color)
                }

                if index % n != n - 1 {
                    // is not all the way to the right
                    let below_right = index + n + 1;
                    if self.grid.get_tile(below_right).is_ok_and(|t| t.is_set()) {
                        errors.push(GameRuleBroken::Diagonal {
                            c1: index,
                            c2: below_right,
                        })
                    }
                }

                if index % n != 0 {
                    // is not all the way to the left
                    let below_left = index + n - 1;
                    if self.grid.get_tile(below_left).is_ok_and(|t| t.is_set()) {
                        errors.push(GameRuleBroken::Diagonal {
                            c1: index,
                            c2: below_left,
                        })
                    }
                }
            }
        }

        Ok((
            errors.clone(),
            (rows.len() == n)
                && (cols.len() == n)
                && (colors.len() == n)
                && errors.clone().is_empty(),
        ))
    }
}
