use crate::errors::QueensResult;
use crate::model::layout::{section, Area, Layout};
use crate::model::state::{InGameState, State};
use crate::model::tile::{Tile, TILE_SIZE};
use eframe::egui::CursorIcon::Default;
use eframe::egui::{vec2, Color32, CornerRadius, Pos2, Rect, Stroke, StrokeKind, Ui, Vec2};

#[derive(Default)]
pub struct HighlightUI {}

impl HighlightUI {
    pub fn render(ui: &mut Ui, state: State) -> QueensResult<()> {
        Self::render_areas(ui, state.clone())?;
        Self::render_keyboard_mark(ui, state)?;
        Ok(())
    }

    fn render_keyboard_mark(ui: &mut Ui, state: State) -> QueensResult<()> {
        if let Some(i) = state.get_marked() {
            Self::highlight(ui, state, Area::from_usize(i, 0), Color32::GRAY)?;
        }
        Ok(())
    }

    fn render_areas(ui: &mut Ui, state: State) -> QueensResult<()> {
        for area in state.get_layout().get_areas() {
            Self::highlight(ui, state.clone(), area.clone(), Color32::GRAY)?;
        }
        Ok(())
    }

    pub fn highlight(
        ui: &mut Ui,
        state: State,
        area: Area,
        highlight_color: Color32,
    ) -> QueensResult<()> {
        // backgroundcolor, bordercolor
        let window_margin = ui.spacing().window_margin;
        let pad = window_margin.leftf();
        let n = state.get_n();

        let upper_left_corner = Pos2::new(pad, pad) + Vec2::new(2.5, 2.5);

        ui.spacing_mut().item_spacing = Vec2::splat(window_margin.leftf());

        for column in 0..n {
            for row in 0..n {
                if area
                    .get_sections()
                    .contains(&state.get_grid().merge_coordinate(column, row)?)
                {
                    let tile_side = TILE_SIZE.x;
                    let upper_left = upper_left_corner
                        + Vec2::new(
                            column as f32 * (tile_side + pad) - pad / 2f32,
                            row as f32 * (tile_side + pad) - pad / 2f32,
                        );
                    let upper_right = upper_left + Vec2::new(tile_side + 2f32 + pad / 2f32, 0f32);
                    let bottom_right = upper_left
                        + Vec2::new(tile_side + 2f32 + pad / 2f32, tile_side + 2f32 + pad / 2f32);
                    let bottom_left = upper_left + Vec2::new(0f32, tile_side + 2f32 + pad / 2f32);

                    // TODO coordinate type!
                    if !(column != 0
                        && state
                            .get_grid()
                            .merge_coordinate(column - 1, row)
                            .is_ok_and(|t| area.get_sections().contains(&t)))
                    {
                        ui.painter().line_segment(
                            [upper_left, bottom_left],
                            Stroke::new(1.0, Color32::WHITE),
                        );
                    }
                    if !(row != 0
                        && state
                            .get_grid()
                            .merge_coordinate(column, row - 1)
                            .is_ok_and(|t| area.get_sections().contains(&t)))
                    {
                        ui.painter().line_segment(
                            [upper_left, upper_right],
                            Stroke::new(1.0, Color32::WHITE),
                        );
                    }
                    if !state
                        .get_grid()
                        .merge_coordinate(column + 1, row)
                        .is_ok_and(|t| area.get_sections().contains(&t))
                    {
                        ui.painter().line_segment(
                            [upper_right, bottom_right],
                            Stroke::new(1.0, Color32::WHITE),
                        );
                    }
                    if !state
                        .get_grid()
                        .merge_coordinate(column, row + 1)
                        .is_ok_and(|t| area.get_sections().contains(&t))
                    {
                        ui.painter().line_segment(
                            [bottom_left, bottom_right],
                            Stroke::new(1.0, Color32::WHITE),
                        );
                    }

                    // also quadratic bezier curves for rounded corners?
                }
            }
        }
        Ok(())
    }
}
