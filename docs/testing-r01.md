# R01 testing guide

This branch is a teacher-provided **red** test pack for R01. It declares the public API and expected behavior but intentionally leaves the production implementation behind `todo!()` placeholders.

## Prerequisites

```bash
rustup toolchain install stable --component rustfmt clippy
rustup default stable
rustc --version
cargo --version
```

No service, database, environment variable, or third-party crate is required.

## Create your solution branch

```bash
git fetch origin
git switch -c R01-solution origin/mentor/R01-tests
```

Target `main` when you later open the pull request.

## Confirm the intentional red state

```bash
cargo test --test r01_command_parsing parses_get_and_delete_into_typed_commands -- --exact
```

The crate should compile and the selected test should panic at the parser's intentional `todo!()`. Toolchain, manifest, or unrelated compilation failures are environment/scaffold failures rather than the expected red state.

## Focused commands

```bash
cargo test --test r01_command_parsing
cargo test --test r01_key
cargo test --test r01_key rejects_empty_keys -- --exact
cargo test --test r01_command_parsing parses_get_and_delete_into_typed_commands -- --exact --nocapture
RUST_BACKTRACE=1 cargo test --test r01_command_parsing parses_get_and_delete_into_typed_commands -- --exact --nocapture
```

## Debugging loop

1. Reproduce one focused failure.
2. Read the first relevant compiler diagnostic or assertion difference.
3. Enable `RUST_BACKTRACE=1` for an unexpected panic.
4. Add temporary `dbg!` or `eprintln!` output around only the failing state.
5. Isolate the relevant token, UTF-8 byte length, byte index, or parser branch.
6. Remove temporary diagnostics.
7. Rerun the focused test and then the full verification sequence.

## Failure categories

- Compiler or borrow-checker error: production code does not type-check or violates ownership rules.
- API mismatch: public names, variants, fields, or signatures no longer match the contract.
- Failed assertion: the function returned, but its observable result is wrong.
- Returned error mismatch: inspect the variant and its owned fields.
- Unexpected panic: external input reached a placeholder, unchecked assumption, or explicit panic.
- Nondeterminism or timing failure: not expected in R01; investigate hidden state or the environment.
- Environment failure: Cargo/toolchain/component failure before the intended test executes.

## Full verification

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo doc --workspace --no-deps
cargo build --workspace --release
```

A completed R01 exits successfully for every command. Add at least two meaningful tests of your own as required by issue #2.
