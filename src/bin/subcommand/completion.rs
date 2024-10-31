use clap::{Args, CommandFactory};
use clap_complete::{generate, Shell};
use padbox::Error;

#[derive(Args)]
/// Generate completion script for the shell
pub struct Completion {
    shell: Shell,
}

impl Completion {
    pub fn run(&self) -> Result<(), Error> {
        let mut cmd = crate::Cli::command();
        generate(self.shell, &mut cmd, "padbox", &mut std::io::stdout());
        Ok(())
    }
}
