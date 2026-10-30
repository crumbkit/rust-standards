# durable-state

A project adopts this profile when it writes files that must survive a
crash, an interruption or an upgrade.

## lint

| ID | Rule | Check |
| --- | --- | --- |
| DUR-1 | One module writes durable files. It writes a temporary file beside the target, syncs it, renames it over the target and syncs the directory. | [baseline/clippy/durable-state.toml](../baseline/clippy/durable-state.toml) |
| DUR-2 | One module takes file locks, with a deadline and a way to cancel the wait. | [baseline/clippy/durable-state.toml](../baseline/clippy/durable-state.toml) |

## test

| ID | Rule | Check |
| --- | --- | --- |
| DUR-3 | Each stored document has one current version. A change that removes, renames or retypes a key comes with a tested upgrade from each supported old version. A reader refuses a newer version and tells the user to upgrade the program. | a test that upgrades a fixture of each supported old version |
| DUR-4 | Remove a file or resource only when its recorded identity and its current owner agree. Otherwise leave it and report what the caller can do. | a test of cleanup with a competing owner |

## Reasons

- DUR-1: A file written in place is half old and half new if the process
  stops partway. A rename within one directory replaces the file in one
  step, and the syncs make the new contents and the rename survive a power
  loss.
