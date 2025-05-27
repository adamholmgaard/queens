use crate::model::layout::{section, Area, Layout};
use crate::model::state::State;
use crate::model::tile::{Tile, TILE_SIZE};
use eframe::egui::CursorIcon::Default;
use eframe::egui::{vec2, Color32, CornerRadius, Pos2, Rect, Stroke, StrokeKind, Ui, Vec2};

#[derive(Default)]
pub struct HighlightUI {}

impl HighlightUI {
    pub fn render(ui: &mut Ui, state: &State) {
        Self::render_areas(ui, state.clone());
        Self::render_keyboard_mark(ui, state.clone());
    }

    fn render_keyboard_mark(ui: &mut Ui, state: State) {
        if let Some(i) = state.get_marked() {
            Self::highlight(ui, state.clone(), Area::from_usize(i, 0), Color32::GRAY);
        }
    }

    fn render_areas(ui: &mut Ui, state: State) {
        for x in state.get_layout().get_areas() {
            //Self::highlight(ui, state.clone(), x.clone(), Color32::GRAY);
        }
    }

    pub fn highlight(ui: &mut Ui, state: State, area: Area, highlight_color: Color32) {
        // backgroundcolor, bordercolor
        let window_margin = ui.spacing().window_margin;
        let pad = window_margin.leftf();
        let n = state.get_n();

        let upper_left_corner = Pos2::new(pad, pad) + Vec2::new(2.5, 2.5);

        ui.spacing_mut().item_spacing = Vec2::splat(window_margin.leftf());

        for column in 0..n {
            for row in 0..n {
                if area.get_sections().contains(
                    &state
                        .get_grid()
                        .merge_coordinate(column, row)
                        .expect("error"),
                ) {
                    let tile_side = TILE_SIZE.x;
                    let upper_left = upper_left_corner
                        + Vec2::new(
                            column as f32 * (tile_side + pad),
                            row as f32 * (tile_side + pad),
                        );
                    let upper_right = TILE_SIZE.to_pos2() + upper_left.to_vec2();

                    ui.painter().rect(
                        Rect::from_min_max(
                            upper_left - Vec2::splat(1.0),
                            upper_right + Vec2::splat(1.0),
                        ),
                        2,
                        highlight_color.gamma_multiply_u8(127),
                        Stroke::new(1.0, Color32::WHITE),
                        StrokeKind::Middle,
                    );
                }
            }
        }
    }
}
