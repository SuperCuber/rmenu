use conrod::color::{self, Color};

#[doc = "Configuration for the GUI"]
pub struct Config {
    #[doc = "Background color"]
    pub canvas_color: Color,
    #[doc = "Input color"]
    pub input_color: Color,
    #[doc = "Color of unselected options in the menu"]
    pub unselected_color: Color,
    #[doc = "Color of selected option in the menu"]
    pub selected_color: Color,
    #[doc = "Size of input box"]
    pub input_size: [f64; 2],
    #[doc = "Size of the output list"]
    pub output_size: [f64; 2],
}

impl Config {
    #[doc = "Default values for the configuration."]
    pub fn default() -> Config {
        Config {
            canvas_color: color::BLACK,
            input_color: color::BLUE,
            unselected_color: color::WHITE,
            selected_color: color::RED,
            input_size: [100.0, 20.0],
            output_size: [100.0, 1000.0],
        }
    }
}
