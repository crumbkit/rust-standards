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
| `[[binding]]` | `rule`, `owners` (files), `checks`, `note`: where the project meets a rule | for BASE-24 when the project calls `expose_secret`; for SVC-12, naming the owner of each external store or service, when it adopts `service`; optional for any other rule |
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

## Taking a version that adds a lint

A project's lint table is its own copy of
[baseline/lints/base.toml](baseline/lints/base.toml), so a new lint reaches
the project only when the project copies it. The project takes the version
in a change of its own.

1. Set `standards.version` and `standards.commit` in `rust-standards.toml`
   to the new version.
2. Copy the new lint into the project's `[workspace.lints]` table at
   `"warn"`, and run the `clippy` step without `CARGO_BUILD_WARNINGS=deny` to
   count the lint's firings. A run at `"deny"` undercounts, because an error
   in one crate stops every crate that depends on it. Then set the lint to
   `"deny"`.
3. Handle each firing in one of three ways. Fix the code, keep the code with
   `#[expect(lint, reason = "...")]`, or weaken the lint in the project's
   table and record the weakening as an exception. For BASE-56 the attribute
   goes on the `match` or on the statement that holds it, and never on an arm.

A project that weakens `clippy::wildcard_enum_match_arm` (BASE-56) sets the
lint to `"allow"` in its table and records this exception. `"warn"` is no
weaker than `"deny"`, because CI fails on any warning (BASE-1).

```toml
[[exception]]
rule = "BASE-56"
items = ["lint:clippy::wildcard_enum_match_arm"]
owner = "Luke Randall"
reason = "The project had 31 wildcard arms on enums when it took this version."
correction = "Fix the wildcard arms and set the lint back to deny."
```
