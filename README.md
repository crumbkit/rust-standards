# rust-standards

These are the rules that Luke Randall's open-source Rust projects follow.
They are opinionated on purpose. Each rule says what to do, gives a reason
when the reason is not obvious, and names the cheapest check that enforces
it.

The rules are a draft. Nothing here is versioned or tagged yet.

## Index

```text
README.md                      profiles, lanes, check tiers, versions
AGENTS.md                      rules for editing this repository
adoption.md                    the rust-standards.toml format
profiles/
├── base.md                    BASE-*  every project
├── cli.md                     CLI-*   programs that people or agents run from a shell
├── library.md                 LIB-*   crates that other crates depend on
├── service.md                 SVC-*   network services
├── child-process.md           PROC-*  projects that start other programs
├── durable-state.md           DUR-*   files that must survive a crash or an upgrade
├── deployed.md                DEP-*   programs that run as long-lived hosted containers
├── control-plane.md           CP-*    programs that drive resources toward a desired state
└── personal-data.md           PII-*   projects that handle personal data
baseline/
├── lints/base.toml            [workspace.lints] for every project
├── lints/library.toml         lints that the library profile adds
├── clippy/base.toml           clippy.toml entries for every project
├── clippy/<profile>.toml      clippy.toml entries that a profile adds
├── deny.toml                  cargo-deny configuration
└── nextest.toml               .config/nextest.toml
templates/
├── ci.yml                     a GitHub Actions workflow that runs the lanes below
├── dependabot.yml             weekly updates of every pin (BASE-26)
└── rust-toolchain.toml        an exact toolchain pin (BASE-26)
examples/
└── notekeeper.toml            a worked adoption map for a small CLI
```

## Profiles

- A project adopts `base` and every other profile that describes it.
- A project ignores the rules of a profile it does not adopt.
- A project's `clippy.toml` is the union of the fragments of the profiles it
  adopts.
- A project may use a baseline file from a profile it does not adopt, such
  as `lints/library.toml`. Using the file adopts none of that profile's rules.

## Pull-request lane

Every project runs these steps on each pull request (BASE-25). CI and any
local task runner use the same step names, so a failure reads the same in
both places.

Within one job the steps run cheapest first, so a pull request fails as early
as it can. `format` and `workflows` read only the source
and run before anything fetches or builds dependencies. `deny` and `shear`
read the dependency sources, so they run after those are fetched and before
`clippy`. A project may split the steps across parallel jobs instead.

| Step | Command | Runs when |
| --- | --- | --- |
| `format` | `cargo fmt --check` | always |
| `workflows` | `zizmor .` | the project has `.github/workflows/` |
| `deny` | `cargo deny --locked check bans licenses sources` | always |
| `shear` | `cargo shear` | always |
| `clippy` | `CARGO_BUILD_WARNINGS=deny cargo clippy --locked --workspace --all-targets --all-features` | always |
| `tests` | `cargo nextest run --locked --workspace --all-features` | always |
| `doctests` | `cargo test --locked --doc --workspace --all-features` | the workspace has a library target, because Cargo refuses `--doc` without one |
| `rustdoc` | `RUSTDOCFLAGS="-D warnings" cargo doc --locked --workspace --no-deps` | the project uses `lints/library.toml` |
| `features` | `CARGO_BUILD_WARNINGS=deny cargo hack clippy --locked --workspace --each-feature --all-targets` | the project adopts `library` (LIB-5) |

[templates/ci.yml](templates/ci.yml) runs these steps in this order.

## Scheduled and release checks

These checks run outside the pull-request lane, because their result can
change while the project does not.

| Check | Command | Runs |
| --- | --- | --- |
| `advisories` | `cargo deny --locked check advisories` | daily on main, and in the release lane, which it blocks (BASE-29) |
| `auditable` | `cargo auditable build --locked --release` | in the release lane, for each shipped binary (BASE-30) |
| `semver` | `cargo semver-checks` | in the release lane of a library (LIB-7) |

## Check tiers

Every rule names its check. The tiers run from cheapest to most expensive.

| Tier | Where it runs | Cost |
| --- | --- | --- |
| `lint` | rustc or Clippy, in every build | nothing beyond the build |
| `tool` | one command in the pull-request lane | seconds |
| `drift` | a comparison of the project's files with the baseline | seconds |
| `test` | a test that the project writes | the test's runtime |
| `release` | the release lane | minutes |
| `review` | a reviewer reading the change | the reviewer's time |

- A rule uses the cheapest tier that can check it.
- A `review` rule moves to a cheaper tier as soon as a tool can check it.
- No drift check ships with these standards yet, and one is planned. It
  will be a command in this repository that compares a project's files with
  the baseline of the version that the project's map names, and with the map
  itself. Each project will pin the command at that version, so a new
  version of the check never changes another project's result. Until it
  ships, a reviewer checks each `drift` rule against the project's files.

## Versions

- A rule ID never changes meaning. A withdrawn rule keeps its ID and is
  marked withdrawn.
- A project names the version it follows in `rust-standards.toml`
  ([adoption.md](adoption.md)). Nothing upgrades a project automatically.
- Each version will be a tag `v<version>` on main once the first version is
  released.
