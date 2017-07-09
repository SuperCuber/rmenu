extern crate rmenu;
extern crate conrod;
use conrod::color;

fn main() {
    let cmd = rmenu::Command::new;
    let options = vec![
        cmd("11", "Display11", "Cmd11"),
        cmd("12", "Display12", "Cmd12"),
        cmd("2", "Display2", "Cmd2"),
    ];

    let config = rmenu::Config {
        canvas_color: color::GRAY.with_alpha(0.1),
        input_color: color::GRAY,
        unselected_color: color::WHITE,
        selected_color: color::BLACK,
        ..Default::default()
    };

    rmenu::reasonable_main(&options, &config).unwrap();

    println!("");
}
