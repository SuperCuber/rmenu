extern crate rmenu;

fn main() {
    let mut options = Vec::new();

    options.push(cmd("11", "Display11", "Cmd11"));
    options.push(cmd("12", "Display12", "Cmd12"));
    options.push(cmd("2", "Display2", "Cmd2"));

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
        rmenu::GuiResult::Option(ref cmd) |
        rmenu::GuiResult::Custom(ref cmd) => {
            println!("Executing {}", cmd.command());
        }
        _ => {}
    }

    println!("");
}

fn cmd<K, D, C>(key: K, display: D, command: C) -> rmenu::Command
where
    K: Into<String>,
    D: Into<String>,
    C: Into<String>,
{
    rmenu::Command::new(key, display, command)
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
