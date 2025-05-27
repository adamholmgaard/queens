use crate::model::coordinate::Coordinate;
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

            for i in 0..state.get_n() {
                ui.horizontal(|ui| {
                    for j in 0..state.get_n() {
                        let coord = Coordinate::from_context(i, j, state.get_n() as u8).expect("");

                        let tile: Tile = state.get_tile(coord);
                        if ui.add(tile).clicked() {
                            state.flip_tile(coord);
                        }
                    }
                });
            }
        });
    }
}
