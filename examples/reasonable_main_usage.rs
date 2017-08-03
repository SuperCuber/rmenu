extern crate rmenu;
extern crate conrod;
use conrod::color;

fn main() {
    let cmd = rmenu::Command::new;
    rmenu::reasonable_main(
        &[
            cmd("11", "Display11", "notify-send Cmd11"),
            cmd("12", "Display12", "notify-send Cmd12"),
            cmd("2", "Display2", "notify-send Cmd2"),
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
