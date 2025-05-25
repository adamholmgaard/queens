use crate::state::state::State;
use crate::state::tile::Tile;
pub(crate) use crate::view::grid_ui::GridUi;
use crate::view::underlay_ui::UnderlayUi;
use eframe::egui::{vec2, Align2, Area, Button, CentralPanel, Context, Id, Ui, Vec2, Window};
use log::debug;
use crate::view::highlight_ui::HighlightUI;

// Ingame ui
#[derive(Default)]
pub struct QueensUi {}

impl QueensUi {
    pub fn render(&self, ctx: &Context, state: &mut State) {
        let panel = CentralPanel::default();

        panel.show(ctx, |ui| {
            UnderlayUi::render(ui, state);
            GridUi::render(ui, state);
            HighlightUI::render(ui, state);
        });

        let (errors, game_won) = state.get_game_status();

        if !errors.is_empty() {
            Window::new("Error list")
                .anchor(Align2::RIGHT_TOP, Vec2::new(0.0, 15.0))
                .show(ctx, |ui| {
                    for error in errors {
                        ui.label(format!("{}", error));
                    }
                });
        }

        if game_won {
            Window::new("You won!")
                .anchor(Align2::RIGHT_TOP, Vec2::new(0.0, 15.0))
                .show(ctx, |_| {});
        }
    }
}
