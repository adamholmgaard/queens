use crate::distinct_colors::{get_distinct_checked_color, get_distinct_color};
use eframe::egui::{vec2, Button, Color32, Response, RichText, Ui, Vec2, Widget};

pub static TILE_SIZE: Vec2 = vec2(32.0, 32.0);

#[derive(Clone, Debug, Copy, Default)]
pub struct Tile {
    set: bool,
    color: u8,
}

impl Tile {
    pub fn new(set: bool, color: u8) -> Self {
        Self { set, color }
    }

    pub fn on_click(&self) -> Self {
        Self {
            set: !self.set,
            color: self.color,
        }
    }

    pub fn get_raw_color(&self) -> u8 {
        self.color
    }

    pub fn get_color(&self) -> Color32 {
        if self.set {
            get_distinct_checked_color(self.color)
        } else {
            get_distinct_color(self.color)
        }
    }

    pub fn is_set(&self) -> bool {
        self.set
    }
}

impl Widget for Tile {
    fn ui(self, ui: &mut Ui) -> Response {
        let button = Button::new(if self.set {
            RichText::new("X").color(Color32::BLACK)
        } else {
            RichText::from("").color(Color32::BLACK)
        })
        .min_size(TILE_SIZE)
        .fill(self.get_color());

        button.ui(ui)
    }
}
