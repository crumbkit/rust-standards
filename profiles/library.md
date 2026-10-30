# library

A project adopts this profile when other crates depend on it.

## lint

| ID | Rule | Check |
| --- | --- | --- |
| LIB-1 | The library reads no environment variable and writes to no standard stream. It takes writers, values and clocks from its caller. | [baseline/clippy/base.toml](../baseline/clippy/base.toml), `print_stdout`, `print_stderr` |
| LIB-2 | Every public item has a doc comment. A public function's doc comment says when it returns an error and when it panics, and every intra-doc link resolves. | `missing_docs`, `missing_errors_doc` and `missing_panics_doc` in [baseline/lints/library.toml](../baseline/lints/library.toml); `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --no-deps`, because only rustdoc runs the link lint |
| LIB-3 | Mark a public function `#[must_use]` when its only effect is its return value. | `must_use_candidate` |

## tool

| ID | Rule | Check |
| --- | --- | --- |
| LIB-4 | `rust-version` in `Cargo.toml` states the minimum supported Rust version, and every target builds on it with every feature. | `cargo +<rust-version> check --workspace --all-targets --all-features` |
| LIB-5 | The library builds without warnings with no features, with each feature alone and with all features. | `CARGO_BUILD_WARNINGS=deny cargo hack clippy --locked --workspace --each-feature --all-targets` |

## drift

| ID | Rule | Check |
| --- | --- | --- |
| LIB-6 | No feature is on by default, and each feature has a comment line that says what it adds. | drift |

## release

| ID | Rule | Check |
| --- | --- | --- |
| LIB-7 | A release's version follows semantic versioning for the public API, and the changelog lists each API change. | `cargo semver-checks` |

## review

| ID | Rule |
| --- | --- |
| LIB-8 | The library keeps no global state. |
| LIB-9 | Mark a public struct or enum `#[non_exhaustive]` from its first release when it may gain a field or a variant, and give such a struct a constructor or a builder. Leave an enum unmarked when the domain fixes its cases, such as a compass direction. |

## Reasons

- LIB-5: `--all-features` hides a feature that does not build alone, because
  another feature supplies an import or an item that it needs.
- LIB-6: A default feature is on for every dependant that does not opt out,
  so it adds build time and crates that most callers never use.
- LIB-9: Adding a field or a variant to an exhaustive public type breaks
  every dependant that builds or matches it, so the change needs a major
  version.
