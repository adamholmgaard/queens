pub mod distinct_colors;
pub mod errors;
mod model;
mod view;

use crate::model::state::{GameState, State};
use eframe::egui::Context;
use eframe::{egui, Frame};
use errors::QueensResult;
use log::warn;
use crate::view::in_game::queens_ui::QueensUi;
use crate::view::main_menu::main_menu_ui::MainMenuUi;
// ONLY the main functionality

fn main() -> eframe::Result {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Queens",
        options,
        Box::new(|cc| {
            // Use the dark theme
            cc.egui_ctx.set_theme(egui::Theme::Dark);
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<QueensApp>::default())
        }),
    )
}

#[derive(Default)]
struct QueensApp {
    state: State,
    in_game_ui: QueensUi,
    main_menu_ui: MainMenuUi,
}

impl eframe::App for QueensApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        if let Err(e) = match self.state.get_game_state() {
            GameState::InGame => self.render_in_game(ctx),
            GameState::MainMenu => self.render_main_menu(ctx),
            GameState::Won => todo!(),
        } {
            warn!("{}", e); // if not debug give error window?

            self.state = State::default(); // reset to main menu

            ctx.request_discard(e); // refresh rendering
        }
    }
}

impl QueensApp {
    fn render_in_game(&mut self, ctx: &Context) -> QueensResult<()> {
        self.in_game_ui.render(ctx, &mut self.state)
    }

    fn render_main_menu(&mut self, ctx: &Context) -> QueensResult<()> {
        self.main_menu_ui.render(ctx, &mut self.state)
    }
}
