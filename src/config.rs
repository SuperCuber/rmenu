use conrod::color::{self, Color};

pub struct Config {
    pub canvas_color: Color,
    pub input_color: Color,
    pub unselected_color: Color,
    pub selected_color: Color,
}

impl Config {
    pub fn default() -> Config {
        Config {
            canvas_color: color::DARK_CHARCOAL,
            input_color: color::WHITE,
            unselected_color: color::WHITE,
            selected_color: color::RED,
        }
    }
}
