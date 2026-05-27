use super::CommandContext;

/// Runs a command against a prepared [`CommandContext`].
pub trait Runnable {
    /// The command-specific error type.
    type Error;

    /// Executes the command.
    fn run(
        self,
        context: &mut CommandContext,
    ) -> std::result::Result<std::process::ExitCode, Self::Error>;
}
