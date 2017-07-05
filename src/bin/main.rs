extern crate rmenu;

fn main() {
    let mut options = Vec::new();

    options.push(rmenu::Command::new("11", "Display11", "Cmd11"));
    options.push(rmenu::Command::new("12", "Display12", "Cmd12"));
    options.push(rmenu::Command::new("2", "Display2", "Cmd2"));

    let ans = rmenu::run(|s| process(s, &options));

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
        rmenu::GuiResult::Option(cmd) |
        rmenu::GuiResult::Custom(cmd) => {
            println!("Executing {}", cmd.command());
        }
        _ => {}
    }

    println!("");
}

fn process(text: &str, options: &[rmenu::Command]) -> Vec<rmenu::Command> {
    let mut answer = Vec::new();
    for option in options {
        if option.key().starts_with(text) {
            answer.push(option.clone());
        }
    }
    answer
}
