use crate::model::coordinate::Coordinate;
use crate::model::state::State;
use crate::model::tile::Tile;
pub(crate) use crate::view::grid_ui::GridUi;
use crate::view::highlight_ui::HighlightUI;
use crate::view::underlay_ui::UnderlayUi;
use eframe::egui::{vec2, Align2, Area, Button, CentralPanel, Context, Id, Key, Ui, Vec2, Window};
use log::debug;

// Ingame ui
#[derive(Default)]
pub struct QueensUi {}

impl QueensUi {
    pub fn render(&self, ctx: &Context, state: &mut State) {
        let panel = CentralPanel::default();
        let n = state.get_n();

        if ctx.input(|x| x.key_pressed(Key::ArrowRight)) {
            let new_coord = match state.get_marked() {
                None => Coordinate::from(0),
                Some(c) => {
                    // todo impl ord for coordinate
                    let raw = c.get();
                    if raw % n == n - 1 {
                        c + 1 - n
                    } else {
                        c + 1
                    }
                }
            };
            state.set_marked(Some(new_coord));
        }
        if ctx.input(|x| x.key_pressed(Key::ArrowLeft)) {
            let new_coord = match state.get_marked() {
                None => Coordinate::from(0),
                Some(c) => {
                    let raw = c.get();
                    if raw % n == 0 {
                        c + n - 1
                    } else {
                        c - 1
                    }
                }
            };
            state.set_marked(Some(new_coord));
        }
        if ctx.input(|x| x.key_pressed(Key::ArrowDown)) {
            let new_coord = match state.get_marked() {
                None => Coordinate::from(0),
                Some(c) => {
                    let raw = c.get();
                    if raw >= n * (n - 1) {
                        c + n - n * n
                    } else {
                        c + n
                    }
                }
            };
            state.set_marked(Some(new_coord));
        }
        if ctx.input(|x| x.key_pressed(Key::ArrowUp)) {
            let new_coord = match state.get_marked() {
                None => Coordinate::from(0),
                Some(c) => {
                    let raw = c.get();
                    if raw < n {
                        c + n * (n - 1)
                    } else {
                        c - n
                    }
                }
            };
            state.set_marked(Some(new_coord));
        }
        if ctx.input(|x| x.key_pressed(Key::Escape)) {
            state.set_marked(None);
        }
        if ctx.input(|x| x.key_pressed(Key::Space) || x.key_pressed(Key::Enter)) {
            if let Some(c) = state.get_marked() {
                state.flip_tile(c);
            }
        }

        debug!("{:?} is marked", state.get_marked());

        panel.show(ctx, |ui| {
            UnderlayUi::render(ui, state);
            GridUi::render(ui, state);
            HighlightUI::render(ui, state);
        });

        let (errors, game_won) = state.get_game_status();

        // TODO only show these windows if debug
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
