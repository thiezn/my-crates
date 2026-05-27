//! Example binary that reuses the shared hello command.

use clap::{Parser, Subcommand};
use cli_helpers::Runnable;
use cli_helpers::clap::CommonArgs;
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(name = "some-cli", about = "Example CLI that reuses hello-command")]
struct Cli {
    #[command(flatten)]
    common: CommonArgs,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Hello(hello_command::HelloCommand),
}

fn main() -> Result<ExitCode, Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    cli.common.init_tracing()?;

    let mut context = cli.common.command_context()?;
    let exit_code = match cli.command {
        Command::Hello(command) => command.run(&mut context)?,
    };

    Ok(exit_code)
}
