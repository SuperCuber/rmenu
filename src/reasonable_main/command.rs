#[derive(Debug)]
#[doc = "Represents a command that is selectable in the menu"]
pub struct Command {
    key: String,
    display: String,
    command: String,
}

impl Command {
    #[doc = "Creates a new instance of Command"]
    pub fn new<K, D, C>(key: K, display: D, command: C) -> Command
    where
        K: Into<String>,
        D: Into<String>,
        C: Into<String>,
    {
        Command {
            key: key.into(),
            display: display.into(),
            command: command.into(),
        }
    }

    #[doc = "Returns the key"]
    pub fn key(&self) -> &str {
        &self.key
    }
    #[doc = "Returns the display string"]
    pub fn display(&self) -> &str {
        &self.display
    }
    #[doc = "Returns the command"]
    pub fn command(&self) -> &str {
        &self.command
    }
}

impl Into<String> for Command {
    #[doc = "Returns a string representation"]
    fn into(self) -> String {
        self.display.clone()
    }
}

impl From<String> for Command {
    #[doc = "Creates a Command where key, display, and command are equal to arg"]
    fn from(arg: String) -> Command {
        Command::new(arg.clone(), arg.clone(), arg)
    }
}

impl Clone for Command {
    fn clone(&self) -> Self {
        Command {
            key: self.key.clone(),
            display: self.display.clone(),
            command: self.command.clone(),
        }
    }
}
