#[derive(Debug)]
pub struct Command {
    key: String,
    display: String,
    command: String,
}

impl Command {
    pub fn new<A, B, C>(key: A, display: B, command: C) -> Command
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Command {
            key: key.into(),
            display: display.into(),
            command: command.into(),
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }
    pub fn display(&self) -> &str {
        &self.display
    }
    pub fn command(&self) -> &str {
        &self.command
    }
}

impl Into<String> for Command {
    fn into(self) -> String {
        self.display.clone()
    }
}

impl From<String> for Command {
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
