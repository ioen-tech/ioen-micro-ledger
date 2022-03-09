As an Open Source project Holochain welcomes many sorts of contributions, bug-report & fixes, code contributes, documentation, tests, feedback and more.

# Social
We are committed to foster a vibrant thriving community, including growing a culture that breaks cycles of marginalization and dominance behavior. In support of this, some open source communities adopt Codes of Conduct. We are still working on our social protocols, and empower each team to describe its own Protocols for Inclusion. Until our teams have published their guidelines, please use the link above as a general guideline.

## Coordination
For support be sure to explore our Developer Resources and Documentation
Please chat with us on our online forums

## Test Driven Development
We use test driven development. When you add a new function or feature, be sure to add both unit tests and integration tests to shows that it works. Pull requests without tests will most-likely not be accepted!

## Compiler warnings
Compilation warnings are NOT OK in shared/production level code.

Warnings have a nasty habit of piling up over time. This makes your code increasingly unpleasant for other people to work with.

CI MUST fail or pass, there is no use in the ever noisier "maybe" status.

If you are facing a warning locally you can try:

0. Fixing it
1. Using #[allow(***)] inline to surgically override a once-off issue
2. Proposing a global allow for a specific rule
- this is an extreme action to take
- this should only be considered if it can be shown that:
- the issue is common (e.g. dozens of #allow[***])
-- disabling it won't cause issues/mess to pile up elsewhere
-- the wider Rust community won't find our codebase harder to work with
-- If you don't know the best approach, please ask for help!

It is NOT OK to disable deny for warnings globally at the CI or makefile/nix level.

You can allow warnings locally during development by setting the RUSTFLAGS environment variable.

#### Code style
We use rust-fmt to enforce code style so that we don't spend time arguing about this.
