use eframe::egui::Color32;

// todo generalise
// maybe try https://colorbrewer2.org/#type=qualitative&scheme=Paired&n=12
// and if that, also just upper bound by 12 (seems reasonable)
pub fn get_distinct_color(c: u8) -> Color32 {
    let res = match c {
        0 => Color32::from_hex("#00008b"),
        1 => Color32::from_hex("#b03060"),
        2 => Color32::from_hex("#ff4500"),
        3 => Color32::from_hex("#ffff00"),
        4 => Color32::from_hex("#00ff00"),
        5 => Color32::from_hex("#00ffff"),
        6 => Color32::from_hex("#ff00ff"),
        7 => Color32::from_hex("#6495ed"),
        8 => Color32::from_hex("#ffdead"),
        9 => Color32::from_hex("#006400"),
        _ => Ok(Color32::GRAY),
    };

    match res {
        Ok(color) => color,
        _ => Color32::GRAY,
    }
}

pub fn get_distinct_checked_color(c: u8) -> Color32 {
    get_distinct_color(c).gamma_multiply_u8(191)
}
