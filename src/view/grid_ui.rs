use crate::model::state::State;
use crate::model::tile::Tile;
use eframe::egui::{Ui, Vec2};
use log::debug;

// Grid ui
pub struct GridUi {}

impl GridUi {
    pub fn render(ui: &mut Ui, state: &mut State) {
        let window_margin = ui.spacing().window_margin;

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = Vec2::splat(window_margin.leftf());

            for row in 0..state.get_n() {
                ui.horizontal(|ui| {
                    for col in 0..state.get_n() {
                        let coord = state
                            .get_grid()
                            .merge_coordinate(col, row)
                            .expect("Invalid index");

                        let tile: Tile = state.get_tile(coord).expect("Invalid index");
                        if ui.add(tile).clicked() {
                            state.flip_tile(coord);
                        }
                    }
                });
            }
        });
    }
}
