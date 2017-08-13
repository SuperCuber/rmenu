use conrod::color::{self, Color};

#[doc = "Configuration for the GUI. Check source for what the defaults are.

**Note**: ALWAYS add `..Default::default()` when creating a Config
since I may add more configuration options and I will consider it a non breaking change."]
pub struct Config {
    #[doc = "Background color"]
    pub canvas_color: Color,
    #[doc = "Input color"]
    pub input_color: Color,
    #[doc = "Color of unselected options in the menu"]
    pub unselected_color: Color,
    #[doc = "Color of selected option in the menu"]
    pub selected_color: Color,

    #[doc = "Size of border around input"]
    pub input_border: f64,
    #[doc = "Color of border around input"]
    pub input_border_color: Color,

    #[doc = "Size of input box"]
    pub input_size: [f64; 2],
    #[doc = "Size of the output list"]
    pub output_size: [f64; 2],

    #[doc = "Padding above input"]
    pub input_top_padding: f64,
    #[doc = "Padding between input and output"]
    pub output_top_padding: f64,

    #[doc = "Path to a .ttf file with the font to use"]
    pub font: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            canvas_color: color::BLACK,
            input_color: color::BLUE,
            unselected_color: color::WHITE,
            selected_color: color::RED,

            input_border: 1.0,
            input_border_color: color::BLACK,

            input_size: [200.0, 25.0],
            output_size: [200.0, 1000.0],

            input_top_padding: 0.0,
            output_top_padding: 0.0,

            font: "/usr/share/fonts/TTF/Ubuntu-M.ttf".into(),
        }
    }
}
