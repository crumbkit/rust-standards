# Working on rust-standards

[README.md](README.md) indexes the repository.

- A rule is one table row: its ID, its statement and its check. A `review`
  rule's row also says where it applies (README.md).
- A rule names the cheapest check tier that can enforce it (README.md).
- A rule gives a reason only when the reason is not obvious. Long reasons go
  in the profile's Reasons section.
- A rule has one home. Other files cite its ID.
- A new rule names, in its pull request, the failure it prevents in a
  project that adopts it.
- Rule IDs are never renumbered. A withdrawn rule stays in its table, marked
  withdrawn.
- Write plain declarative sentences with a named subject. A limit says what
  it counts and what happens when it is reached.
- Commit messages and pull request bodies describe the rules that changed
  and carry no attribution lines.
