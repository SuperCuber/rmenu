use reasonable_main::command;
use gui;
use config;
use gui_result;

use std::process;
use std::io;

#[doc = "A reasonable main function.
Commands are filtered by command.key().starts_with(input),
         then selected_command.command() is executed."]
pub fn reasonable_main(options: &[command::Command], config: &config::Config) -> io::Result<()> {
    let ans = gui::run_config(
        |input| filter(options, |option| option.key().starts_with(input)),
        config,
    );

    // Execution
    match ans {
        gui_result::GuiResult::Option(ref cmd) |
        gui_result::GuiResult::Custom(ref cmd) => {
            let (shell, flag) = if cfg!(target_os = "windows") { ("cmd", "/C") } else { ("sh", "-c") };
            process::Command::new(shell)
                .arg(flag)
                .arg(&cmd.command())
                .spawn()?;
        }
        gui_result::GuiResult::Cancel => {}
    }

    Ok(())
}

fn filter<T, F>(vector: &[T], function: F) -> Vec<T>
where
    F: FnMut(&&T) -> bool,
    T: Clone,
{
    vector.iter().filter(function).cloned().collect()
}
