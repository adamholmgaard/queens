use crate::errors::{QueensError, QueensResult};
use crate::model::state::{GameState, InGameState, State};
use crate::view::in_game::grid_ui::GridUi;
use crate::view::in_game::highlight_ui::HighlightUI;
use crate::view::in_game::underlay_ui::UnderlayUi;
use eframe::egui::{vec2, Align2, Area, Button, CentralPanel, Context, Id, Key, Ui, Vec2, Window};
use std::thread::sleep;
use std::time::Duration;

#[derive(Default)]
pub struct InGameUi {}

impl InGameUi {
    pub fn render(&self, ctx: &Context, state: &mut State) -> QueensResult<()> {
        let panel = CentralPanel::default();

        self.handle_keyboard_input(ctx, state)?;

        let mut res = Ok(());
        panel.show(ctx, |ui| {
            UnderlayUi::render(ui, state);
            res = res
                .and_then(|_| GridUi::render(ui, state))
                .and_then(|_| HighlightUI::render(ui, state.in_game().clone()));
        });
        res?;

        let in_game_state = state.in_game_mut();

        let (errors, game_won) = in_game_state.get_win_status()?;

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
            state.set_game_won();
        }

        Ok(())
    }

    fn handle_keyboard_input(&self, ctx: &Context, state: &mut State) -> QueensResult<()> {
        let n = state.get_n();

        let cmd_ctrl_pressed = ctx.input(|x| x.modifiers.command_only());
        let default_marked = 0;

        if ctx.input(|x| x.key_pressed(Key::ArrowRight)) {
            let new_coord = match state.in_game().get_marked() {
                None => default_marked,
                Some(c) => {
                    if cmd_ctrl_pressed {
                        if c % n == n - 1 {
                            c
                        } else {
                            c + (n - (c % n)) - 1
                        }
                    } else {
                        if c % n == n - 1 {
                            c + 1 - n
                        } else {
                            c + 1
                        }
                    }
                }
            };
            state.in_game_mut().set_marked(Some(new_coord));
        }
        if ctx.input(|x| x.key_pressed(Key::ArrowLeft)) {
            let new_coord = match state.in_game().get_marked() {
                None => default_marked,
                Some(c) => {
                    if cmd_ctrl_pressed {
                        if c % n == 0 {
                            c
                        } else {
                            c - (c % n)
                        }
                    } else {
                        if c % n == 0 {
                            c + n - 1
                        } else {
                            c - 1
                        }
                    }
                }
            };
            state.in_game_mut().set_marked(Some(new_coord));
        }
        if ctx.input(|x| x.key_pressed(Key::ArrowDown)) {
            let new_coord = match state.in_game().get_marked() {
                None => default_marked,
                Some(c) => {
                    if cmd_ctrl_pressed {
                        if c >= n * (n - 1) {
                            c
                        } else {
                            n * (n - 1) + (c % n)
                        }
                    } else {
                        if c >= n * (n - 1) {
                            c + n - n * n
                        } else {
                            c + n
                        }
                    }
                }
            };
            state.in_game_mut().set_marked(Some(new_coord));
        }
        if ctx.input(|x| x.key_pressed(Key::ArrowUp)) {
            let new_coord = match state.in_game().get_marked() {
                None => default_marked,
                Some(c) => {
                    if cmd_ctrl_pressed {
                        if c < n {
                            c
                        } else {
                            c % n
                        }
                    } else {
                        if c < n {
                            c + n * (n - 1)
                        } else {
                            c - n
                        }
                    }
                }
            };
            state.in_game_mut().set_marked(Some(new_coord));
        }
        if ctx.input(|x| x.key_pressed(Key::Escape)) {
            if state.in_game().get_marked().is_some() {
                state.in_game_mut().set_marked(None);
            } else {
                return Err(QueensError::RefreshRequested);
            }
        }
        if ctx.input(|x| x.key_pressed(Key::Space) || x.key_pressed(Key::Enter)) {
            if let Some(c) = state.in_game().get_marked() {
                state.in_game_mut().flip_tile(c).expect("Could not flip tile");
            }
        }
        Ok(())
    }
}
