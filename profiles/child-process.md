# child-process

A project adopts this profile when it starts other programs.

## lint

| ID | Rule | Check |
| --- | --- | --- |
| PROC-1 | One module starts processes, through one function per operation, such as "run the installer". It never accepts an arbitrary program and arguments. A command that hands the terminal to a child through `crumbkit_child::hand_over` (CLI-2) builds the child's `Command` in this module. | [baseline/clippy/child-process.toml](../baseline/clippy/child-process.toml) |

## test

| ID | Rule | Check |
| --- | --- | --- |
| PROC-2 | Each operation states the five items below. | a test of each operation's arguments and failures, and review |

PROC-2's items:

- program: how it is found, and who is trusted to supply its bytes;
- arguments: separate values, never a shell string, unless the operation is
  defined as running a shell and says why that is trusted;
- environment: which variables the child inherits or receives;
- streams and deadline: where standard input comes from, how much output is
  captured, and the deadline;
- end: who stops and reaps the child, how its exit status is read, and what
  partial work it can leave behind.

## review

| ID | Rule |
| --- | --- |
| PROC-3 | Never describe a deadline or an argument allowlist as isolation. A module that stops only its direct child, and not the child's descendants, says so. |
