use crate::state::state::State;
use eframe::egui::{Button, Color32, CornerRadius, Pos2, Rect, Stroke, StrokeKind, Ui, Vec2};
use log::debug;

pub struct UnderlayUi {}

impl UnderlayUi {
    pub fn render(ui: &mut Ui, state: &State) {
        let pad = ui.spacing().item_spacing.x;
        let queens_length = state.get_n() as f32 * 32.0 + (state.get_n() as f32 - 1.0) * 6.0;
        let queens_size = Pos2::new(6.0 + queens_length + pad, 6.0 + queens_length + pad);

        ui.painter().rect(
            Rect::from_min_max(Pos2::new(0.0, 3.0), queens_size),
            CornerRadius::same(15),
            Color32::GRAY.gamma_multiply_u8(23),
            Stroke::new(1.0, Color32::WHITE),
            StrokeKind::Middle,
        );
    }
}
