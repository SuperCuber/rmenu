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
        canvas_color: color::GRAY,
        input_color: color::GRAY,
        unselected_color: color::WHITE,
        selected_color: color::BLACK,
        ..Default::default()
    };

    let ans = rmenu::run_config(
        |input| filter(&options, |option| option.key().starts_with(input)),
        &config,
    );

    // Illustration
    match ans {
        rmenu::GuiResult::Cancel => {
            println!("Cancelled");
        }
        rmenu::GuiResult::Option(ref cmd) => {
            println!("User selected option with cmd: {:?}", cmd);
        }
        rmenu::GuiResult::Custom(ref cmd) => {
            println!("Custom choice: {:?}", cmd);
        }
    }

    // Execution
    match ans {
        rmenu::GuiResult::Option(ref cmd) |
        rmenu::GuiResult::Custom(ref cmd) => {
            println!("Executing {}", cmd.command());
        }
        _ => {}
    }

    println!("");
}

fn filter<T, F>(vector: &[T], function: F) -> Vec<T>
where
    F: FnMut(&&T) -> bool,
    T: Clone,
{
    vector
        .iter()
        .filter(function)
        .map(|item| item.clone())
        .collect()
}
