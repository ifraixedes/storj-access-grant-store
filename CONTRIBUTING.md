# Contributing

This document is work-in-progress.

## Git conventions

This repository follows the [Conventional Commits v1.0.0](https://www.conventionalcommits.org/en/v1.0.0/).

The current accepted _types_ are:

- ci: Everything related to CI, except bumping versions of related systems used by the CI.
- chore: Maintenance stuff, for example, bumping dependencies, etc.
- doc: Documentation stuff, for example, README, source code comments, etc.
- enhance: Improvements on current implementation which aren't bug fixes nor refactoring.
- feat: Adding new features.
- fix: Fixing bugs, typos, build warnings, etc.
- refactor: Improvements in the current code base without changing behavior nor the public API.
- test: Everything related to testing.

Subject description must be in imperative mood and it has a soft limit of 50 characters long,
however it is good if it is clear at the expense of having to slightly pass the limit.

Don't use `!` in the subject line to draw attention.

When they contain __breaking changes__ always add the _BREAKING_CHANGE_ footer.

Commit without body message are allowed, but when they are straightforward changes and the subject
line can describe them without any further explanation.

Body messages should contain _what_ and _why_ of the changes, never the _how_. They may also
contain relevant information useful when they are read in a far future (15 years?).

All commits in must be signed and preferably
[verified by Github](https://docs.github.com/en/authentication/managing-commit-signature-verification/signing-commits).

The repository follows a linear commit history pull request cannot contain merge commits. To apply
upstream changes to a branch, please rebase it to the base branch.
