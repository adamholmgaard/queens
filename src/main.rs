mod state;
mod view;

use crate::state::state::State;
use eframe::egui::Context;
use eframe::{egui, Frame};
use view::queens_ui::QueensUi;
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
    ui: QueensUi, // Ingame ui
                  // todo between game ui
}

impl eframe::App for QueensApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.ui.render(ctx, &mut self.state);
        // between game ui render
    }
}
