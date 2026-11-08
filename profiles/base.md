# base

Every project follows these rules.

## lint

| ID | Rule | Check |
| --- | --- | --- |
| BASE-1 | Every crate inherits the workspace lint table from [baseline/lints/base.toml](../baseline/lints/base.toml), and CI fails on any warning. | `[lints] workspace = true`, `CARGO_BUILD_WARNINGS=deny` |
| BASE-2 | Suppress a lint with `#[expect(lint, reason = "...")]`, never `#[allow]`. | `allow_attributes`, `allow_attributes_without_reason` |
| BASE-3 | Write no `unsafe` code unless the map's `unsafe` decision names the module and its invariant. | `unsafe_code` |
| BASE-4 | Code outside tests never panics on any input, so a panic that remains marks a broken invariant. An index or slice whose bound is proven carries `#[expect(clippy::indexing_slicing, reason = "...")]` that states the proof. Arithmetic overflow panics in release builds too (BASE-27). Each test crate root in `tests/` starts with `//!` lines that say what it tests, followed by `#![cfg(test)]`, so Clippy treats its helpers as test code. | `unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented`, `dbg_macro`, `unreachable`, `indexing_slicing` |
| BASE-5 | Give each effect, such as the clock, the file system, the network or the standard streams, one owning module. The owner suppresses the lint with `#[expect]`, and other code takes a value, a writer or a trait from it. | `disallowed_methods`, `disallowed_types`, [baseline/clippy/](../baseline/clippy/) |
| BASE-6 | Read the environment once, in the entry point, into a snapshot that the rest of the program takes as a value. No code sets or removes a variable of its own process, and a library reads none. | `disallowed_methods` on `std::env::*` |
| BASE-7 | Return typed errors from production code, and never build an untyped one. Return the exit code from `main` instead of calling `std::process::exit`. | `disallowed_macros` listing the project's untyped-error macros, such as `anyhow::anyhow`; `disallowed_methods` on `std::process::exit` |
| BASE-8 | Keep visibility no wider than its use, and delete dead code. | `unreachable_pub`, `dead_code` |
| BASE-9 | A module with children is `foo.rs` beside `foo/`, never `foo/mod.rs`. | `mod_module_files` |
| BASE-10 | A function has at most 150 lines, counted by Clippy. A longer function fails the build, and is split. | `too_many_lines` |
| BASE-11 | Wait for a signal the code can observe, and never sleep to let something happen. A retry's backoff sleeps through the clock owner (BASE-5). When the source has no event to wait on, such as a file another process writes without notice, poll it at an interval paced by the injected clock, and say at the poll that the source has no event. A timed wait on a real event, such as a condvar, `poll(2)` or a socket read, meets this rule without the clock owner, and may recheck the project's own cancellation flag when it times out. A source the project owns, such as its own thread, channel or lock, always gets an event: a message when the thread finishes, one channel carrying an enum in place of several channels, or a condvar in place of polling a lock. Every poll interval and wait timeout is a named constant or a validated configuration value. | `disallowed_methods` on `std::thread::sleep`, `tokio::time::sleep` |
| BASE-12 | A numeric conversion that can lose information returns an error or states how it rounds. Use `From` for a lossless conversion and `TryFrom` for a fallible one. A float becomes an integer only through one helper that rounds explicitly and checks the range. Keep `as` for conversions that cannot lose anything, such as reading an enum's discriminant. | `cast_possible_truncation`, `cast_sign_loss`, `cast_possible_wrap`, `cast_precision_loss`, `cast_lossless` |
| BASE-13 | Name each imported item outside tests. Do not use glob imports such as `use foo::*` or `use Enum::*`. A glob from a module named `prelude`, and a `pub use` re-export glob, are allowed. | `wildcard_imports`, `enum_glob_use` |
| BASE-14 | A function takes and returns `Option<&T>`, never `&Option<T>`. An exported function follows the rule when it is added or its signature changes. A trait implementation, or a signature another crate's contract fixes, such as a serde `serialize_with` function, follows that contract and carries an `#[expect]` where the lint fires. | `ref_option` for private functions, review for exported ones |
| BASE-15 | Write a loop with a side effect as a `for` loop, never `for_each`. | `needless_for_each` on a collection's iterator, review for other iterators |
| BASE-16 | A function takes at most three `bool` parameters. Use an enum for any further choice. | `fn_params_excessive_bools` |

## tool

| ID | Rule | Check |
| --- | --- | --- |
| BASE-17 | Format all code with rustfmt. | `cargo fmt --check` |
| BASE-18 | Every dependency has a licence and a source that [baseline/deny.toml](../baseline/deny.toml) allows. | `cargo deny check bans licenses sources` |
| BASE-19 | Every declared dependency is used. | `cargo shear` |
| BASE-20 | The test runner never retries a failed test. | [baseline/nextest.toml](../baseline/nextest.toml) |

## drift

| ID | Rule | Check |
| --- | --- | --- |
| BASE-21 | A production `.rs` file has at most 1,000 lines, counting every line. A file over the limit is split by responsibility, or listed as an exception in the map with a correction. Files that hold only tests, such as `foo/tests.rs` and files under `tests/` and `benches/`, have no limit. | drift |
| BASE-22 | Put a module's unit tests in `foo/tests.rs`, declared as `#[cfg(test)] mod tests;`. Under `src/`, `#[cfg(test)]` appears only on `mod` declarations, so production code has no test-only branch. | drift |
| BASE-23 | Every dependency entry in every `Cargo.toml` that states a version or a git source has a comment line above it that says why the project needs it. An entry such as `foo.workspace = true` needs no comment, because the workspace entry has one. | drift |
| BASE-24 | Hold every secret in a `secrecy` type, such as `SecretString`, whose `Debug` prints a placeholder. Call `expose_secret` only in the files that the map's BASE-24 binding lists. | drift |
| BASE-25 | The pull-request lane runs the steps in [README.md](../README.md#pull-request-lane) within a time budget that the map's `lanes` decision states. A check that does not fit the budget, such as fuzzing, long property tests, benchmarks or a platform matrix, runs in another lane that the decision also records. | drift for the decision, review for its content |
| BASE-26 | Pin every build input. `rust-toolchain.toml` names an exact Rust version, such as `1.97.1`. `Cargo.lock` is committed, and CI passes `--locked` to every Cargo command that resolves dependencies. CI installs each tool at a stated version, and pins each GitHub Action to a commit. Every package uses edition 2024. Dependabot or Renovate opens a pull request each week that moves these pins. | drift for the toolchain file, the lockfile and the edition; review for the rest |
| BASE-27 | `[profile.release]` in the root manifest sets `overflow-checks = true` and leaves `panic` at `unwind`. A project that ships a binary records its `lto`, `codegen-units` and `strip` settings in the map's `release-profile` decision. | drift for `overflow-checks`, review for the rest |
| BASE-54 | Every property test runs with one fixed seed and without proptest's failure persistence files. The `[env]` table in `.cargo/config.toml` sets `PROPTEST_RNG_SEED` and `PROPTEST_DISABLE_FAILURE_PERSISTENCE`, and `cargo test` and `cargo nextest` pass both to every test. A fix for a failing case that a property test found adds a unit test that holds the failing input. | drift |

## test

| ID | Rule | Check |
| --- | --- | --- |
| BASE-28 | Keep a catalog of every environment variable the program reads, with its purpose, its accepted form, and what the program does when it is absent, empty or invalid. | a test that the catalog matches the snapshot's reads (BASE-6) |

## release

| ID | Rule | Check |
| --- | --- | --- |
| BASE-29 | Ship no dependency with a known security advisory. `cargo deny check advisories` runs daily on main and blocks every release. An ignored advisory in `deny.toml` names its ID, why it does not apply, and the date by which it is reviewed again. | `cargo deny check advisories`, scheduled and in the release lane |
| BASE-30 | Build every shipped binary with `cargo auditable`, so the binary records the dependency versions it contains. | the release lane's build command |

## review

| ID | Rule | Where it applies |
| --- | --- | --- |
| BASE-31 | Give each domain meaning one type with one validation rule. Prefer a standard or library type to a new one. | `**/*.rs` |
| BASE-32 | Parse input into its type once, where it enters the program. | `**/*.rs` |
| BASE-33 | Make policy testable without performing its effects. Put an effect behind a narrow trait only when a test double makes a meaningful test possible. | `**/*.rs` |
| BASE-34 | Give every read a size limit, every retry a count, and each operation one deadline that covers all its steps. | `**/*.rs`, `**/*.toml`, `**/*.yaml`, `**/*.yml`, `.github/workflows/**` |
| BASE-35 | A test that involves time starts its deadline after its fixture is ready, injects the clock when it tests a deadline or expiry, and never compares `.elapsed()` with an upper bound. A test of a timed wait on a real event (BASE-11) triggers the event or the cancellation, and tests the timeout with a short configured value. It never waits out the full real timeout, so such a wait takes its timeout as a parameter or configuration value with the named constant as its default. | `**/*.rs` |
| BASE-36 | Name each test with a sentence that states the behaviour it checks. | `**/tests.rs`, `**/tests/**`, `**/benches/**` |
| BASE-37 | A comment states an invariant, a limit or a reason, and never history. A deliberate shortcut carries a comment that says what it misses and what replaces it. | `**` |
| BASE-38 | Add code only when removing code cannot make the change. Add an abstraction only when it removes two copies or another rule needs it. | `**` |
| BASE-39 | Use a standard format or a maintained tool before writing a new one. | `**` |
| BASE-40 | A new CI check names the failure it catches, why existing checks miss it, its lane and its cost. Prefer a type or compiler check to a text scan. | `**` |
| BASE-41 | Documents describe current behaviour, and each fact has one home. Plans and logs go in the pull request. | `**/*.md`, `**/*.rs` |
| BASE-42 | Once a value with an invariant has its type (BASE-31, BASE-32), every function inside the program takes that type, and the raw value appears only where the value leaves the program, such as in a request, a file or a message. The type keeps its fields private, and every way to build it, including `Deserialize`, goes through one checking constructor, for example with `#[serde(try_from = "String")]`. Use `TryFrom` or `FromStr` for a value from one input, and a named constructor such as `Range::new(start, end)` for a value built from several. Do not implement `Deref` to the raw value. A function that takes the raw value must check it again, and a missing check is a defect. | `**/*.rs` |
| BASE-43 | Model steps that must run in order and hold several invariants, such as a protocol or a multi-stage request, as one type per state. A transition takes `self` and returns the next state, or an enum of the states it can reach, so a step out of order does not compile. A transition computes the next state from values and performs no effect; the caller performs each effect and passes its result to the next transition. Model a linear flow with few invariants as one type and plain functions. | `**/*.rs` |
| BASE-44 | Make each outcome the domain expects, such as a refusal or an empty result, a variant of the operation's outcome enum, carrying what explains it. Keep `Result::Err` for technical failures outside the domain's outcomes, such as an unreachable service or a malformed answer. A checking constructor's refusal (BASE-42) is its `Err`, and the operation that receives the raw value turns it into a variant when the domain expects invalid input. Do not use a general two-way type such as `Either` in place of the outcome enum. | `**/*.rs` |
| BASE-45 | A type admits only the states the code handles. Where `bool` and `Option` fields can combine into a state the code cannot handle, such as two `Option`s of which exactly one is set, or where a `String` holds one of a few known values, use an enum with one variant per state that holds that state's data. | `**/*.rs` |
| BASE-46 | Convert a typed value to another type through a conversion between the types, such as a `From` implementation, a `match`, or the target's checking constructor applied to the source's fields. Never format or serialize the whole value and parse the text as the other type. Where no other path exists, such as between two crates' types that share only a text form, return the parse's refusal as an error (BASE-4). | `**/*.rs` |
| BASE-47 | An error that wraps another keeps it as its `source()` and does not copy it into its message. Code either logs an error or returns it, never both. An error's `Display` text is lowercase and has no trailing full stop, as in std. | `**/*.rs` |
| BASE-48 | Enable only the dependency features the project uses. Set `default-features = false` when the default features pull in crates the project does not use. | `**/Cargo.toml` |
| BASE-49 | Spell names out. Use an abbreviation only when it is common outside the project, such as `id`, `url` or `http`. A closure whose body is one expression may name a parameter with one letter, such as `\|e\| e.code()`. | `**/*.rs` |
| BASE-50 | Take each `std::sync` lock through one owning module, so the decision about poisoning has one home. The owner recovers a poisoned lock with `PoisonError::into_inner` only when the guarded data cannot be left half-changed: the lock guards `()`, or every write is one step. A comment at the recovery says which. A lock whose data holds an invariant across several steps turns poisoning into a typed error (BASE-7), and neither recovers nor panics. | `**/*.rs` |
| BASE-51 | Make every outbound HTTP call through one client module (BASE-5) that bounds each attempt by what remains of the operation's deadline (BASE-34) and resends only requests that are safe to resend. A server derives that deadline from its caller's (SVC-7). A call through a provider's SDK sets the SDK's timeout for the whole operation, including its retries, from what remains of the deadline. | `**/*.rs` |
| BASE-52 | Every plan includes a module diagram. The diagram shows each module that the change adds or touches, marks which are new, and draws the dependencies between them, including dependencies the change adds or removes. It is written in a text format that renders where the plan is read, such as a Mermaid block in the pull request. | `**` |
| BASE-53 | Write the test first. A change starts with a test that fails for want of the behaviour, and a bug fix starts with a test that reproduces the bug. When no automated test can show the change, such as a change to a document's wording, the pull request says how it was checked. | `**` |
| BASE-55 | Each limit names the ordered steps that give growth a place to go, each under its own looser limit: a document past its word budget splits into a tree of topic documents with a node cap; a slow job moves to a slower lane with a time cap; code past a size or complexity limit splits into modules or crates. Switching the limit off is never a step. The last step is always the owner's recorded decision to change or retire the limit. Set each threshold with a heuristic that can be rerun, measure it by activity in the governed scope instead of calendar time, and start with a lenient default until there is data. | `**` |

## Reasons

- BASE-1: The baseline turns on pedantic lints one at a time. The `pedantic`
  group gains lints with every Clippy release, so enabling the whole group
  would fail builds on a toolchain update.
- BASE-2: An `#[expect]` warns when its lint stops firing, so a stale
  suppression is found and removed.
- BASE-4: Overflow checks in release builds (BASE-27) turn a silent wrap into
  a panic at the place where the invariant broke.
- BASE-11: A sleep that waits for something to happen is either too long,
  which makes the program slow, or too short, which makes it flaky. The clock
  owner's pause cannot be interrupted, so a timed wait on a real event that
  went through it would react late to the event and to cancellation.
- BASE-12: An `as` cast truncates, wraps or rounds silently, so an
  out-of-range length or price becomes a wrong value instead of an error.
  `as_conversions` is not enabled, because it also flags lossless casts.
- BASE-13 to BASE-16, BASE-49 and BASE-50 draw on Google's Rust style guide
  (<https://google.github.io/styleguide/rustguide.html>). BASE-50 relaxes the
  guide's rule never to take data out of a poisoned mutex.
- BASE-13: A glob import hides where a name comes from, and a name added
  upstream can collide with a local one when a dependency updates.
- BASE-14: A caller that holds only a `&T` cannot pass it as `&Option<T>`
  without cloning it into an `Option<T>`.
- BASE-15: Inside `for_each`, `break` and `?` are unavailable, and `return`
  ends only the current item.
- BASE-16: A call such as `run(true, false, true)` does not say what each
  argument means, and swapped arguments still compile.
- BASE-20: A retry turns a flaky test into a passing one and hides it.
- BASE-21: Long files mix responsibilities and are too large for a person
  or an agent to read whole.
- BASE-22: A test-only constructor under `#[cfg(test)]` is a second way to
  build a type, which BASE-42 forbids.
- BASE-26: An unpinned toolchain or tool changes the build when a new version
  is released, and a pull request then fails for a change it did not make.
- BASE-27: Without overflow checks a release build wraps silently while the
  debug build that tests use panics, so the two behave differently.
  `panic = "abort"` would skip destructors and stop other requests from
  unwinding cleanly.
- BASE-29: An advisory can be published after a dependency is locked, so a
  check on pull requests alone misses advisories against code already on
  main.
- BASE-30: Scanners such as `cargo audit bin` read the recorded dependency
  list, so a later advisory can be matched against binaries already shipped.
- BASE-35: A test that compares elapsed time with an upper bound fails under
  load.
- BASE-42, BASE-43 and BASE-44 follow the experience report "Functional State
  Machines in Rust: Typestate and Newtype Patterns"
  (doi:10.1145/3830438.3830958). In that report a newtype that was built once
  and taken everywhere removed invalid states cheaply, and a function that
  took the raw value lost the guarantee. A type per state removed
  out-of-order faults and made each step testable with fewer mocks, but added
  boilerplate for linear flows. The report recommends effect-free
  transitions, named outcome variants over a generic `Either`, and `TryFrom`
  for construction. It also suggests `Deref` to ease adoption, which BASE-42
  rejects because `Deref` lets raw-value signatures accept the type without a
  check.
- BASE-45 follows Yaron Minsky's principle "make illegal states
  unrepresentable" ("Effective ML Revisited", 2011). The compiler checks that
  a `match` on an enum handles every variant, and nothing checks a field
  combination.
- BASE-46: Clippy cannot see a value written as text and parsed back. The
  parse needs an `expect` or an error path, so a change to either type's rule
  panics or fails at run time where a direct conversion would fail to
  compile.
- BASE-47: A copied source loses its type, so a caller cannot match on it.
  An error that is both logged and returned is logged again by every caller
  that handles it.
- BASE-50: BASE-27 keeps `panic = "unwind"`, so a thread that panics while
  it holds a `std::sync` lock poisons the lock for the other threads. A write
  that the panic interrupted can leave the data broken. `tokio::sync` locks
  do not poison.
- BASE-51: A client built directly on an HTTP crate usually sets its own
  timeout per attempt and its own retries. A per-attempt timeout can outlast
  the caller's whole deadline, attempts in series add up, and an immediate
  retry can repeat a request whose action is not idempotent.
- BASE-52: A module diagram shows the shape of a change before any code is
  read. A reviewer sees a new dependency cycle, a module that gains too many
  responsibilities, or a change that reaches further than its description
  says.
- BASE-53: A test written after the code tends to check what the code does,
  and a test that has never failed may not be able to fail. A test that
  fails first shows that it detects the missing behaviour.
- BASE-54: A random seed lets a pull request fail on a case that its change
  did not cause, and a passing run says nothing about the next one.
  Persistence files are written where the test ran and are not committed, so
  a failing case found in CI is lost. The unit test keeps the case in the
  repository, where every run checks it.
