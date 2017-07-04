use conrod::color::{self, Color};

pub struct Config {
    pub canvas_color: Color,
    pub input_color: Color,
    pub unselected_color: Color,
    pub selected_color: Color,
    pub input_size: [f64; 2],
    // pub output_size: [f64; 2],
}

impl Config {
    pub fn default() -> Config {
        Config {
            canvas_color: color::BLACK,
            input_color: color::WHITE,
            unselected_color: color::WHITE,
            selected_color: color::RED,
            input_size: [100.0, 20.0],
            // output_size: []
        }
    }
}
