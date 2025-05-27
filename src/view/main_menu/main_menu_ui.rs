use crate::errors::QueensResult;
use crate::model::state::State;
use eframe::egui::{CentralPanel, Context};

#[derive(Default)]
pub struct MainMenuUi {}

impl MainMenuUi {
    pub fn render(&self, ctx: &Context, state: &mut State) -> QueensResult<()> {
        CentralPanel::default().show(ctx, |ui| {
            ui.label("This is the main menu");
        });

        Ok(())
    }
}
