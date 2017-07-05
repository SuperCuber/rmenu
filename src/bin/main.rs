extern crate rmenu;

fn main() {
    let cmd = rmenu::Command::new;

    let options = vec![
        cmd("11", "Display11", "Cmd11"),
        cmd("12", "Display12", "Cmd12"),
        cmd("2", "Display2", "Cmd2"),
    ];

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

fn process(text: &str, options: &[rmenu::Command]) -> Vec<rmenu::Command> {
    let mut answer = Vec::new();
    for option in options {
        if option.key().starts_with(text) {
            answer.push(option.clone());
        }
    }
    answer
}
