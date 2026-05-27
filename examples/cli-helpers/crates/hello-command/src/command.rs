use crate::Result;
use cli_helpers::{CommandContext, Runnable};
use std::process::ExitCode;

/// Reusable subcommand that prints a greeting.
#[derive(clap::Args, Debug, Clone)]
pub struct HelloCommand {
    #[arg(long, default_value = "world", help = "Who should be greeted")]
    name: String,
}

impl HelloCommand {
    /// Creates a new hello command.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }

    /// Returns the greeting target.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Runnable for HelloCommand {
    type Error = crate::Error;

    fn run(self, context: &mut CommandContext) -> Result<ExitCode> {
        context.stdout_line(format!("hello {}", self.name))?;
        Ok(ExitCode::SUCCESS)
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used, clippy::unwrap_used)]

    use super::*;
    use std::io::{self, Write};
    use std::sync::{Arc, Mutex};

    #[derive(Clone, Default)]
    struct SharedBuffer(Arc<Mutex<Vec<u8>>>);

    impl SharedBuffer {
        fn contents(&self) -> String {
            let bytes = self.0.lock().unwrap().clone();
            String::from_utf8(bytes).unwrap()
        }
    }

    impl Write for SharedBuffer {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let mut bytes = self
                .0
                .lock()
                .map_err(|_| io::Error::other("shared buffer lock poisoned"))?;
            bytes.extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn writes_greeting_to_stdout() {
        let buffer = SharedBuffer::default();
        let mut context = CommandContext::builder().stdout(buffer.clone()).build();
        let command = HelloCommand::new("mars");

        let exit_code = command.run(&mut context).unwrap();

        assert_eq!(exit_code, ExitCode::SUCCESS);
        assert_eq!(buffer.contents(), "hello mars\n");
    }
}
