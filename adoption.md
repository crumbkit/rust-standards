# Adoption map

A project that adopts these standards keeps `rust-standards.toml` at its
root. The map says which profiles the project adopts, where it meets the
rules that need a local owner, which choices it made where a rule leaves one
open, and where it breaks a rule.
[examples/notekeeper.toml](examples/notekeeper.toml) is a worked example.

## Tables

| Table | Holds | Required |
| --- | --- | --- |
| `schema` | `"rust-standards/adoption/v1"` | yes |
| `standards` | `version`, and the `commit` of this repository that the map was last reviewed against | yes |
| `profiles` | the adopted profiles, always including `"base"` | yes |
| `[[binding]]` | `rule`, `owners` (files), `checks`, `note`: where the project meets a rule | for BASE-24 when the project calls `expose_secret`; for PII-7 when it adopts `personal-data`; for SVC-12, naming the owner of each external store or service, when it adopts `service`; for DDB-1 when it adopts `dynamodb`, and for DDB-3 when a batch job scans; optional for any other rule |
| `[[decision]]` | a choice that a rule leaves to the project | see below |
| `[[exception]]` | `rule`, `paths`, `items`, `owner`, `reason`, `correction` | for each place that breaks a rule |

## Decisions

A decision has four fields, and none may be empty. `requirement` and
`purpose` have the names and meanings of the fields of a Board control
(`controls/control/v1`), so a board that mounts the project can hold the
decision as a control without renaming them.

- `id`: one of the names in the table below, or a name the project chooses
  for a rule it sets itself.
- `requirement`: what the project does.
- `purpose`: why it does that, and, for a check that nothing waits for, the
  failure the check catches and why finding it after merge is safe.
- `realizations`: the files that carry the decision out. Each path exists.
  A Board control that holds the decision lists them in `applies_to`.

| Decision | Required when | `requirement` states |
| --- | --- | --- |
| `lanes` | always (BASE-25) | each lane: what runs, its trigger, what waits for it, and the pull-request lane's time budget |
| `unsafe` | the project uses `unsafe` (BASE-3) | the modules that use it and the invariant each one upholds |
| `release-profile` | the project ships a binary (BASE-27) | the `lto`, `codegen-units` and `strip` settings of `[profile.release]` |
| `shutdown` | the project adopts `deployed` (DEP-5) | the drain deadline and the platform's stop timeout |
| `request-cap` | the project adopts `deployed` and serves traffic (DEP-8) | the cap per instance and the load test it came from, or, until a measurement exists, that the cap is owed |

- A merge waits for a check only when branch protection requires it. The
  `lanes` decision names the required checks, and a reviewer compares them
  with branch protection, because the repository's files do not show it.
- A project may record any stricter rule it sets for itself, such as a lower
  file length limit, as a decision.

## Exceptions

- An exception covers one rule at the listed `paths` (files, or directories
  ending in `/`) or `items` (baseline entries such as
  `lint:clippy::too_many_lines` or `deny:licenses.allow:MPL-2.0`).
- An exception lists at least one path or item.
- `owner` names the person who removes the exception. `reason` says why the
  rule is broken. `correction` says what removes the exception.
- An exception whose paths no longer exist, or that covers nothing, is
  deleted in the change that makes it stale.
- A lint suppressed at the code with `#[expect(lint, reason = "...")]`
  needs no exception. The attribute's reason is the record (BASE-2).
