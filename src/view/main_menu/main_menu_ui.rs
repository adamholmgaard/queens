use crate::errors::QueensResult;
use crate::model::state::{GameState, State};
use eframe::egui::{CentralPanel, Context, Key, Slider};
use log::debug;

#[derive(Default)]
pub struct MainMenuUi {}

impl MainMenuUi {
    pub fn render(&self, ctx: &Context, state: &mut State) -> QueensResult<()> {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("This is the main menu");
                let mut n = state.get_n();
                ui.add(Slider::new(&mut n, 6..=12).text("Set n (can only be 10 right now)"));
                state.set_n(10);

                // todo set whether or not to move marker across sides

                if ui.button("Play").clicked() {
                    state.set_game_state(GameState::InGame);
                }
            });
        });

        if ctx.input(|x| x.key_pressed(Key::Space) || x.key_pressed(Key::Enter)) {
            state.set_game_state(GameState::InGame);
        }

        Ok(())
    }
}
