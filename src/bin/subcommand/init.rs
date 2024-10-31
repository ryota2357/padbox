use clap::Args;
use clap_complete::Shell;
use padbox::Error;

#[derive(Args)]
/// Used to setup the shell environment for 'pad' command
pub struct Init {
    shell: Shell,
}

const SCRIPT_BASH: &str = r#"
pad() {
  if ! command -v __pad &> /dev/null; then
    echo "Error: '__pad' command not found."
    return 1
  fi
  local dir
  if ! dir=$(__pad "$@"); then
    return 1
  fi
  cd "$dir" || return 1
}
"#;

const SCRIPT_FISH: &str = r#"
function pad --description 'Enter a new playground'
  if not type -q __pad
    echo "Error: '__pad' command not found."
    return 1
  end
  set -l dir (__pad $argv)
  if test $status -ne 0
    return $status
  end
  cd $dir
end
"#;

impl Init {
    pub fn run(&self) -> Result<(), Error> {
        let script = match self.shell {
            Shell::Bash => SCRIPT_BASH,
            Shell::Zsh => SCRIPT_BASH,
            Shell::Fish => SCRIPT_FISH,
            Shell::Elvish => todo!(),
            Shell::PowerShell => todo!(),
            _ => todo!(),
        };
        println!("{}", script.trim());
        Ok(())
    }
}
