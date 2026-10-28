# Stage 6: Crates, API design & testing

**Phase:** II · Depth
**Outcome:** You structure a crate so its public surface is small and deliberate, design APIs other engineers find obvious, know what counts as a breaking change, and back claims with tests, properties and benchmark numbers instead of opinions.

Each task: `/study` (recall, then probes: predict, run, explain the gap), solve the katas, run them and read the output, make the gate green, write the journal, then `/review <ID>`.

## S6-T1 · Modules, visibility, workspaces & features
*1 day (3 h)*

**Why it matters.** Every `pub` item is a promise you must keep. Polkadot SDK and Solana are huge workspaces gated by cargo features (`std`, `runtime-benchmarks`, `no-entrypoint`); if you cannot read a feature graph you cannot build them.

**Probes**
- Module tree vs file tree; `pub use` re-exports to shape a flat public API over a deep private layout
- `pub(crate)` and `pub(super)` vs everything-`pub`: what leaks into docs and semver
- Features are additive and unified across the graph: why `default-features = false`, `dep:` syntax and `#[cfg(feature = "...")]` must never remove an API

*Previews:* The `serde` feature only adds `#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]`; using serde is S8-T4, its data model S11-T1.

**Reference** (read after the probes, or when stuck)
- The Rust Book ch. 7 (Packages, Crates, and Modules) and ch. 14.3 (Cargo Workspaces)
- The Cargo Book: "Features" (especially "Feature unification" and "Mutually exclusive features")
- The Rust Reference: "Visibility and Privacy" (`pub(crate)`, `pub(super)`, `pub(in path)`)

**Build.** `labs/ledger-ws/`: a workspace with `ledger-core` (lib) and `ledger-cli` (bin). `ledger-core` has private modules `account`, `ops`, `codec`, a re-exported public API, a `serde` feature that adds `Serialize`/`Deserialize` derives, and a `std` default feature.

**Run.** `cd labs/ledger-ws && cargo run -p ledger-cli`; `cargo hack check --feature-powerset` (or the listed combinations in `scripts/check-features.sh`).

**Acceptance criteria** (all required)
- [ ] Builds and tests pass with no features, default features and all features
- [ ] `cargo doc -p ledger-core` shows only the intended public items
- [ ] No feature removes or changes the signature of an item (additive only)
- [ ] Internal helpers are `pub(crate)` or private; no `pub` without a reason

**Stretch.** Add a `ledger-core` integration test that compiles only under `--features serde` and round-trips an account through JSON.

**Checkpoint questions** (answer in `journal/S6-T1.md`; the reviewer asks them again)
1. Why must cargo features be additive, and what goes wrong when two crates in a graph enable different ones?
2. What is the difference between a module being private and an item being `pub(crate)`?
3. Why do runtime crates typically have a `std` feature that is on by default?

## S6-T2 · API design: guidelines, builders, semver & docs
*2 days (6 h)*

**Why it matters.** A senior engineer's code is used by others. Good APIs make misuse hard (typestate, newtypes, non-exhaustive enums) and evolve without breaking users. SDKs for chains live and die by this.

**Probes**
- A 9-argument constructor vs a builder; a builder that can be called out of order vs a typestate-lite builder (`TxBuilder<NoSigner>` -> `TxBuilder<Signed>`, zero-sized markers held in `std::marker::PhantomData<S>`) where `build()` exists only on the final state
- Breaking vs non-breaking: adding a field to a pub struct, a variant to an enum, a method to a trait; `#[non_exhaustive]` and sealed traits
- Doc comments that compile: `# Examples`, `# Errors`, hidden `?` setup lines

**Reference** (read after the probes, or when stuck)
- Rust API Guidelines: Naming (C-CONV, C-GETTER), Interoperability (C-COMMON-TRAITS), Future proofing (C-STRUCT-PRIVATE, C-SEALED), Documentation (C-EXAMPLE, C-FAILURE)
- The Cargo Book: "SemVer Compatibility"
- `cargo-semver-checks` README; rustdoc book: "How to write documentation"

**Build.** `tasks/src/s06/t02_api_design.rs`: `TransferBuilder` (typestate-lite: `from`, `to`, `amount`, optional `memo`, `fee`; `build() -> Result<Transfer, BuildError>`), `#[non_exhaustive] pub enum TxError`, a sealed `Signer` trait, full rustdoc with runnable examples on every public item. Then `labs/semver-demo/`: tag v0.1.0, make one breaking and one compatible change, run `cargo semver-checks`.

**Run.** `scripts/run.sh S6-T2`; `cargo test -p tasks --doc s06`; `cd labs/semver-demo && cargo semver-checks --baseline-rev v0.1.0`.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s06_t02.rs` pass
- [ ] A `compile_fail` doctest proves `build()` cannot be called before `amount` is set
- [ ] Every public item has docs; every fallible function has an `# Errors` section
- [ ] `#![deny(missing_docs)]` on the module compiles clean
- [ ] Journal lists the semver-checks output for both changes and explains each verdict

**Stretch.** Implement `TryFrom<RawTransfer> for Transfer` and compare it to the builder in the journal: when would you offer both?

**Checkpoint questions** (answer in `journal/S6-T2.md`; the reviewer asks them again)
1. Why is adding a variant to a public enum a breaking change, and what does `#[non_exhaustive]` cost users?
2. What does the typestate builder move from runtime to compile time, and what does it cost?
3. Why seal a trait instead of leaving it open?

## S6-T3 · Test architecture: unit, integration & doc tests
*1 day (3 h)*

**Why it matters.** Tests are a design tool and a contract. Unit tests see privates, integration tests see only the public API, doc tests keep examples honest. Chain code needs all three plus invariant tests, because a bug in state transition is a consensus bug.

**Probes**
- `#[cfg(test)] mod tests` vs `tests/*.rs` vs doctests: what each can see and what each costs to compile
- Shared fixtures in `tests/common/mod.rs`; table-driven tests vs ten copy-pasted tests
- Testing invariants (total supply constant) vs examples; asserting on error variants with `matches!`

**Reference** (read after the probes, or when stuck)
- The Rust Book ch. 11 (Writing Automated Tests)
- The Cargo Book: "Cargo Targets" (tests/, `[[test]]`, `harness`)
- Matklad: "How to Test" (sections on test boundaries and data-driven tests)

**Build.** `labs/ledger-ws/ledger-core`: unit tests per module, `tests/transfers.rs` and `tests/invariants.rs` with a shared `common` fixture, a table of at least 12 transfer cases, and doctests on all public functions.

**Run.** `cd labs/ledger-ws && cargo test -p ledger-core -- --show-output`

**Acceptance criteria** (all required)
- [ ] Unit, integration and doc tests all present and green
- [ ] Integration tests use only the public API (no `pub` added for tests' sake)
- [ ] An invariant test asserts total supply is unchanged after any sequence of transfers, including failed ones
- [ ] Error cases asserted by variant, not by string

**Stretch.** Add `insta` snapshot tests for the CLI's output and note in the journal when snapshots help and when they hide bugs.

**Checkpoint questions** (answer in `journal/S6-T3.md`; the reviewer asks them again)
1. When would you test a private function directly, and when is that a design smell?
2. Why is an invariant test stronger than ten example tests?
3. What does a doctest guarantee that a README example does not?

## S6-T4 · Property-based testing with proptest
*2 days (6 h)*

**Why it matters.** You cannot think of every edge case; a generator can. Properties plus a simple reference model find the inputs you never imagined, and shrinking hands you the smallest failing case. This is how runtimes and codecs are hardened.

**Probes**
- Example tests vs properties: round-trip (`decode(encode(x)) == x`), invariants, idempotence, oracle/model
- Strategies: `any::<T>()`, ranges, `prop_oneof!`, `prop::collection::vec`, `prop_compose!`
- A planted bug, the raw failing input, then the shrunk one; `proptest-regressions/` files

**Reference** (read after the probes, or when stuck)
- The proptest book: "Getting started", "Strategies", "Shrinking", "Failure persistence"
- Hypothesis blog: "Testing against a model" (concept transfers directly)

**Build.** `tasks/src/s06/t04_proptest.rs`: a `Ledger` with `apply(Op) -> Result<(), LedgerError>` and a deliberately simple reference model (`BTreeMap` + `u128` math); proptests: (1) model equivalence over random `Vec<Op>`, (2) supply conservation, (3) encode/decode round-trip of `Op`, (4) failed ops leave state unchanged.

**Run.** `scripts/run.sh S6-T4`; `PROPTEST_CASES=10000 cargo test -p tasks --test s06_t04`

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s06_t04.rs` pass at 10 000 cases
- [ ] The model shares no code with the implementation
- [ ] Journal shows one real bug proptest found (planted or not), with the raw and shrunk inputs
- [ ] Regression file committed for that bug

**Stretch.** Write a custom `Strategy` that generates mostly-valid ops (90% to existing accounts) and compare bug-finding speed with uniform random.

**Checkpoint questions** (answer in `journal/S6-T4.md`; the reviewer asks them again)
1. Why must the reference model be simpler than the implementation, and not share code with it?
2. What does shrinking do, and why does it matter for debugging?
3. Name a property that holds for your ledger but that no example test would naturally check.

## S6-T5 · Benchmarks & profiling: a measured optimization
*2 days (6 h)*

**Why it matters.** "Faster" without a number is a guess. Block production has a hard time budget; weights in FRAME and compute units in Solana are benchmarks turned into protocol rules.

**Probes**
- Why `Instant::now()` loops lie: warmup, outliers, `black_box`, noise; criterion's statistics
- Reading a flamegraph: wide frames, allocation hot spots, hashing
- Measure, change one thing, re-measure; record numbers before and after

**Reference** (read after the probes, or when stuck)
- The Rust Performance Book: "Benchmarking", "Profiling", "Heap Allocations", "Hashing"
- criterion.rs user guide: "Getting started", "Comparing functions", "Advanced configuration"
- `cargo flamegraph` README (on macOS: `sudo` / dtrace notes)

**Build.** `labs/bench-lab/`: a naive `apply_block(&mut State, &[Tx])` (String keys, `HashMap` with SipHash, clone per tx), criterion benches for 1 k / 10 k / 100 k txs, a flamegraph, then one optimization at a time (pre-sized maps, borrowed keys, `AccountId(u64)`, avoiding clones). Write `docs/perf/s06-t05.md` with a before/after table.

**Run.** `cd labs/bench-lab && cargo bench`; `cargo flamegraph --bench apply`

**Acceptance criteria** (all required)
- [ ] Benches run; criterion reports saved; flamegraph SVG committed
- [ ] At least two optimizations, each measured separately with numbers and % change
- [ ] Output state is identical before and after (a test compares state hashes)
- [ ] The write-up names the bottleneck the flamegraph showed and why the fix helped

**Stretch.** Add a `dhat` heap profile and report allocations per transaction before and after.

**Checkpoint questions** (answer in `journal/S6-T5.md`; the reviewer asks them again)
1. What does `black_box` prevent, and what would your numbers look like without it?
2. Why is SipHash the default for `HashMap`, and when is replacing it safe?
3. Which of your optimizations would you *not* ship, and why?

## S6-T6 · Capstone: a publish-quality library crate
*3 days (9 h)*

**Why it matters.** Everything in this stage in one artifact a stranger can depend on: clean public API, docs with examples, additive features, property tests, benchmarks, CI and a changelog. This is the bar for crates you will ship around chain SDKs.

**Probes**
- Crate-level docs (`//!`), `#![deny(missing_docs)]`, `#![forbid(unsafe_code)]`, `docs.rs` metadata and `doc(cfg)`
- A CI script: fmt, clippy `-D warnings`, test per feature set, doc, semver-checks, MSRV
- `cargo publish --dry-run`, `cargo package --list`: what actually ships

**Reference** (read after the probes, or when stuck)
- Rust API Guidelines checklist (all sections), The Cargo Book: "Publishing on crates.io", "Manifest" metadata fields
- Keep a Changelog 1.1; a well-kept crate for reference (e.g. `bytes`, `thiserror`)

**Build.** `labs/<your-crate>/`: pick one (a fixed-point `Amount` type with checked ops, a compact varint/bytes codec, or a Merkle-free key/value journal with commit/rollback). Required: public API reviewed against the guidelines checklist, docs with examples, at least one additive feature, proptests with a model, criterion benches, `scripts/ci.sh`, `CHANGELOG.md`, `README.md`, and an ADR in `docs/adr/` on the main API decision. Publishing is optional.

**Run.** `cd labs/<your-crate> && ./scripts/ci.sh && cargo publish --dry-run`

**Acceptance criteria** (all required)
- [ ] `scripts/ci.sh` green: fmt, clippy `-D warnings`, tests under every feature combination, `cargo doc` with no warnings
- [ ] No `unwrap`/`expect`/panicking index outside tests; `#![forbid(unsafe_code)]`
- [ ] Proptests with a reference model; benches with recorded baseline numbers in the README
- [ ] `cargo publish --dry-run` succeeds; `CHANGELOG.md` has a 0.1.0 entry
- [ ] Guidelines checklist in the journal with each item marked done or justified skip

**Stretch.** Have someone (or a fresh Claude session) use the crate from its README alone and record every point of confusion as an issue.

**Checkpoint questions** (answer in `journal/S6-T6.md`; the reviewer asks them again)
1. Which item of your public API is most likely to need a breaking change, and how did you hedge it?
2. What does your CI catch that `cargo test` alone does not?
3. Why did you choose your feature split, and how did you prove it is additive?

## S6-GATE · Stage 6 gate: closed-book exam + catch-up
*1 day (3 h)*

**Exam.** Closed book, 90 min, blank file `tasks/src/s06_gate.rs`: design a `RateLimiter` public API (`new` via builder with `capacity` and `refill_per_tick`, `try_acquire(&mut self, n) -> Result<(), Limited>`, `tick`), with `#[non_exhaustive]` error, rustdoc with one runnable example and an `# Errors` section, 3 unit tests and one proptest with a model asserting tokens never exceed capacity. Classify 6 given API changes as breaking or not. Then answer 5 oral questions drawn from the S6 checkpoints.

**Pass criteria.** Solved without notes or AI, compiles, tests and doctests pass, semver classifications all correct, oral answers correct or corrected with a clear explanation. Remaining time: redo the weakest task's core from a blank file.
