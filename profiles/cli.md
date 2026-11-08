# cli

A project adopts this profile when it builds a program that people or agents
run from a shell.

Every such program is built on clap and
[crumbkit](https://github.com/crumbkit/crumbkit). crumbkit holds the
command-line conventions: one command grammar, failures with stable codes and
a next step, `--json` documents with JSON Schemas, listings that page, a help
guide that reveals one level at a time, a local usage log, and tests that
check all of these. The rules below require crumbkit and add only what
crumbkit does not cover.

| Crate | What a program uses it for |
| --- | --- |
| `crumbkit` | `parse` and `finish`, `Output` and its view flags, `Document`, `Listing` and `Page`, `Failure`, `Code` and `Recovery`, `Guide` and `Leaf`, effects, `--dry-run` and confirmation |
| `crumbkit-derive` | `#[derive(Failure)]`, through crumbkit's `derive` feature |
| `crumbkit-test` | the convention checks, called from ordinary `#[test]` functions |
| `crumbkit-usage` | the local usage log and the `self usage` command |
| `crumbkit-aliases` | aliases that a person defines for commands |
| `crumbkit-child` | a command that hands the terminal to another program |
| `crumbkit-markdown` | the Markdown command reference, usually through `cargo crumbkit docs` |
| `cargo-crumbkit` | `cargo crumbkit weigh` and `cargo crumbkit docs` |

## lint

| ID | Rule | Check |
| --- | --- | --- |
| CLI-1 | One output module owns standard input, output and error, and reaches them through crumbkit's `Output`. Results go to standard output and diagnostics to standard error. | `print_stdout`, `print_stderr`, [baseline/clippy/base.toml](../baseline/clippy/base.toml) |

## test

| ID | Rule | Check |
| --- | --- | --- |
| CLI-2 | `main` parses with `crumbkit::parse` and prints every result through `crumbkit::finish`, or through `crumbkit_child::hand_over` for a command that hands the terminal to another program. Every command prints a crumbkit `Document`, and a `list` command returns a `Listing` paged by `Page`. | `check_guide`, `check_document_ids`, `check_listings` |
| CLI-3 | Every failure implements crumbkit's `Failure`, preferably with `#[derive(Failure)]`, so it has a stable `Code` and a `Recovery` that names the next step. A code's area is a noun from the domain, never a module name or a verb. | `check_codes`, `check_recoveries`, `check_twins` |
| CLI-4 | Each JSON document's schema is pinned in a golden file. Removing, renaming or retyping a field raises the document's version. | `check_schema_goldens` |
| CLI-5 | The program's `Guide` gives every visible leaf an example, its effects and its document. Every command that changes something declares its effects and takes `--dry-run` and `--yes` where crumbkit puts them. | `check_guide`, `check_effects`, `check_examples` |
| CLI-6 | The program's tests run every crumbkit-test check that applies to it: `check_names`, `check_grammar`, `check_filters`, `check_help_sizes`, `check_help_goldens`, `check_path_weights`, `check_exit_codes`, `check_output_budget`, `check_command_reference` and `check_opt_outs` beside the checks named above. Each opt-out is an `Allow` with a reason, and `check_opt_outs` pins the list in a golden file. | the crumbkit-test checks |
| CLI-7 | The program keeps a crumbkit-usage log, started first in `main`, and mounts `self usage`. | a test that a run appends one line to a log in a temporary directory |
| CLI-8 | When standard output closes, such as under `head`, the program stops without a panic or an error message. Ctrl-C stops the program, removes any file it was partway through writing, and exits with code 130. | a test for each case |
| CLI-11 | Output is the same at a terminal and in a pipe. The exceptions are the ones that crumbkit's [OUTPUT-1](https://github.com/crumbkit/crumbkit/blob/main/docs/standards/README.md) lists, such as colour, progress, prompts and the width of a text table. | a test that runs each example at a pseudo-terminal and in a pipe and compares the bytes |
| CLI-12 | Every prompt has a flag that answers it, such as `--yes`. Without a terminal on standard input, or with `--no-input`, a command that would prompt fails with a code that names the flag. | `check_effects`, and a test that runs each prompting command without a terminal |
| CLI-13 | Each exit code other than 0 is declared in the program's `Guide` with the failures that produce it, so `help exit-codes` documents it. | `check_exit_codes` |
| CLI-14 | The program prints no colour when `NO_COLOR` is set to a non-empty value. | a test that runs a command that prints colour at a pseudo-terminal with `NO_COLOR=1` and finds no escape codes |

## review

| ID | Rule | Where it applies |
| --- | --- | --- |
| CLI-9 | A command or flag has one spelling. A removed name stays as a hidden alias, or fails with a code that names its replacement. Help, the command reference and any agent skill change in the same pull request as the command. | `**` |
| CLI-10 | A secret is never a command-line argument. The program reads it from the environment, a file or a prompt. | `**/*.rs`, `**/*.sh`, `bin/**`, `scripts/**`, `**/Makefile`, `**/*.mk`, `**/[Jj]ustfile`, `**/.justfile`, `.github/workflows/**`, `**/Dockerfile*`, `**/*.md` |

## Reasons

- CLI-3: An agent recovers from a failure by matching its code and running
  the next step that the failure names.
- CLI-6: Each check fails with every problem it finds, so a new program can
  turn the checks on one at a time. An opt-out that excuses nothing fails its
  check, so opt-outs disappear with the commands they covered.
- CLI-7: The usage log records the shape of each run, never the values the
  caller typed, and shows where callers waste effort on the program's help or
  on refused command lines.
- CLI-8: A shell reports 130 for a program that Ctrl-C stopped, and scripts
  test for it. Rust ignores SIGPIPE, so a write to a closed pipe returns an
  error that `println!` turns into a panic.
- CLI-10: Other processes on the machine can read a process's arguments.
- CLI-11: A script or an agent reads the output through a pipe, so it must
  see what a person sees at a terminal. A golden file recorded in a pipe
  then also holds at a terminal.
- CLI-12: A prompt with no flag blocks a script or an agent, which has no
  terminal to answer it.
