use clap::{Parser, Subcommand};
mod subcommand;

#[derive(Parser)]
#[command(author, version, about, long_about = None, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New(subcommand::New),
    Init(subcommand::Init),
    Completion(subcommand::Completion),
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Commands::New(cmd) => cmd.run(),
        Commands::Init(cmd) => cmd.run(),
        Commands::Completion(cmd) => cmd.run(),
    };
    if let Err(e) = result {
        e.eprintln();
    }
}
