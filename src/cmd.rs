use std::env;
use std::process::{Command, Stdio};

pub trait CommandExt {
    fn inherit(&mut self) -> &mut Self;

    fn print(&mut self) -> &mut Self;
}

impl CommandExt for Command {
    /// Configure command to inherit parent's stdout and stderr
    fn inherit(&mut self) -> &mut Self {
        self.stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
    }

    /// Print the command as it will be executed
    fn print(&mut self) -> &mut Self {
        if env::var_os("FLV_CMD").is_some() {
            println!(">> {}", format!("{:?}", self).replace("\"", ""));
        }
        self
    }
}
