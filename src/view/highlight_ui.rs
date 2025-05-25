use crate::model::coordinate::Coordinate;
use crate::model::layout::Area;
use crate::model::state::State;
use eframe::egui::{vec2, Color32, Ui, Vec2};

#[derive(Default)]
pub struct HighlightUI {}

impl HighlightUI {
    pub fn render(ui: &mut Ui, state: &State) {
        Self::render_areas(ui, state.clone())
        // someday render mouse highlights
    }

    fn render_areas(ui: &mut Ui, state: State) {
        for x in state.get_layout().get_areas() {
            Self::highlight(ui, state.clone(), x.clone(), Color32::GRAY)
        }
    }

    pub fn highlight(ui: &mut Ui, state: State, area: Area, highlight_color: Color32) {
        // backgroundcolor, bordercolor
        let window_margin = ui.spacing().window_margin;
        let size_1x1 = vec2(32.0, 32.0);
        let n = state.get_n();

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing = Vec2::splat(window_margin.leftf());

            for i in 0..n {
                ui.horizontal(|ui| {
                    for j in 0..n {
                        if area
                            .get_sections()
                            .contains(&Coordinate::from_context(i, j, n as u8).unwrap().get())
                        {
                            // work in progress
                            //ui.add_sized(size_1x1, Button::new("lol").fill(highlight_color.gamma_multiply_u8(127)));
                        }
                    }
                });
            }
        });
    }
}
