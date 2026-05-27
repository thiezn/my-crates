# Hello Command Example

This example shows the composition model intended by `cli-helpers`.

- `hello-command` is a reusable library crate that exports a clap `Args` type
  and implements `cli_helpers::Runnable`.
- `alpha-cli`, `beta-cli`, and `gamma-cli` are separate binary crates.
- Each binary owns its own root parser and reuses the same `hello-command`
  building block.

Try any of these commands from the repository root:

```bash
cargo run -p alpha-cli -- hello
cargo run -p beta-cli -- hello --name mathijs
cargo run -p gamma-cli -- --log-level debug hello
```
