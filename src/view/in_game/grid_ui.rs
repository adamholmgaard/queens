use crate::errors::QueensResult;
use crate::model::state::{GameState, State};
use crate::model::tile::Tile;
use eframe::egui::{Ui, Vec2};

// Grid ui
pub struct GridUi {}

impl GridUi {
    pub fn render(ui: &mut Ui, state: &mut State) -> QueensResult<()> {
        let window_margin = ui.spacing().window_margin;

        let mut res = Ok(());
        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = Vec2::splat(window_margin.leftf());

            for row in 0..state.get_n() {
                ui.horizontal(|ui| {
                    for col in 0..state.get_n() {
                        res = res.and_then(|_| {
                            let in_game_state = state.in_game();
                            let coord = in_game_state.get_grid().merge_coordinate(col, row)?;

                            let tile: Tile = in_game_state.get_tile(coord)?;
                            if ui.add(tile).clicked() {
                                state.in_game_mut().flip_tile(coord)?;
                            }
                            Ok(())
                        });
                    }
                });
            }
        });

        res
    }
}
