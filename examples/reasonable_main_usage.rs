extern crate rmenu;
extern crate conrod;
use conrod::color;

fn main() {
    let cmd = rmenu::Command::new;
    rmenu::reasonable_main(
        &[
            cmd("term", "Terminal", "termite"),
            cmd("ff", "Firefox", "firefox"),
            cmd("time", "Show time", "notify-send \"`date`\""),
        ],
        &rmenu::Config {
            canvas_color: color::GRAY.with_alpha(0.1),
            input_color: color::GRAY,
            unselected_color: color::WHITE,
            selected_color: color::BLACK,
            ..Default::default()
        },
    ).unwrap();
}
