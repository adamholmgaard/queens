use crate::errors::QueensResult;
use crate::model::layout::LayoutType;
use crate::model::state::State;
use eframe::egui::Event::Text;
use eframe::egui::{CentralPanel, Context, Key, RadioButton, Slider};

#[derive(Default)]
pub struct MainMenuUi {}

impl MainMenuUi {
    pub fn render(&self, ctx: &Context, state: &mut State) -> QueensResult<()> {
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label("This is the main menu");
                let mut n = state.get_n();
                ui.add(Slider::new(&mut n, 6..=12).text("Set n (can only be 10 right now)"));
                state.set_n(n);

                ui.label("Select a layout type");
                let mut layout_type = state.get_layout_type().clone();
                ui.radio_value(&mut layout_type, LayoutType::Generated, "Generate a layout");
                ui.radio_value(&mut layout_type, LayoutType::Easy, "Easily solvable");
                ui.radio_value(
                    &mut layout_type,
                    LayoutType::Complex,
                    "Complex template (only for n = 10)",
                );
                if layout_type == LayoutType::Complex {
                    state.set_n(10);
                }
                state.set_layout_type(layout_type);

                // todo set whether or not to move marker across sides

                if ui.button("Play").clicked() {
                    state.load_in_game();
                }
            });
        });

        if ctx.input(|x| x.key_pressed(Key::Space) || x.key_pressed(Key::Enter)) {
            state.load_in_game();
        }

        Ok(())
    }
}
