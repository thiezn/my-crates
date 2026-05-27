# CLI Helpers

CLI Helpers is an opinionated support crate for Rust terminal applications.
It is built for a specific workflow: reusable command crates should stay small,
typed, and easy to compose into multiple binaries without each project needing
to invent its own CLI runtime glue.

## What The Crate Provides

- A command runtime layer centered on `CommandContext` and `Runnable`
- Typed output configuration with validated field selectors
- Process-wide tracing setup that stays owned by the binary crate
- Clap adapters for the shared arguments most binaries need
- Utility modules for config loading, interactive prompts, path resolution,
  progress styling, and Markdown parsing

## Core Concepts

### `Runnable`

Reusable command crates implement `Runnable` and keep their own error types.
That keeps the business logic testable and independent from the root parser.

### `CommandContext`

The root binary constructs a `CommandContext` once and passes it into commands.
That context owns process-facing services such as stdout, stderr, structured
output configuration, and color preferences.

### `cli_helpers::clap`

The optional clap adapter layer translates parsed arguments into the core runtime
types. This keeps clap out of the lower-level output and tracing modules while
still giving binaries a shared set of flags.

### Explicit Composition

The intended composition model is static and typed:

1. A reusable command crate exports a clap `Args` type and implements `Runnable`.
2. A binary crate defines its own root `Subcommand` enum.
3. The binary matches its enum and delegates execution to the shared command.

That keeps ownership clear. The binary owns parsing and process setup. The
reusable command crate owns command behavior.

## Example Shape

```rust,ignore
use clap::{Parser, Subcommand};
use cli_helpers::clap::CommonArgs;
use cli_helpers::Runnable;

#[derive(Parser, Debug)]
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let cli = Cli::parse();
	cli.common.init_tracing()?;

	let mut context = cli.common.command_context()?;
	match cli.command {
		Command::Hello(command) => {
			command.run(&mut context)?;
		}
	}

	Ok(())
}
```

See the `examples/hello_command` workspace for a complete composition example
with one reusable command crate consumed by multiple binaries.

