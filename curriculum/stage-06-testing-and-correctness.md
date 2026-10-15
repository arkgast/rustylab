# Stage 6: Testing & correctness

**Outcome:** You specify invariants, test them with generated inputs against a reference model, and measure performance before optimizing.

**Window:** 2026-12-18 → 2026-12-29 · tracker: [`ROADMAP.md`](../ROADMAP.md)

Each task: read first, build in `tasks/` (or the crate named), write the journal entry, then run `/task-review <ID>`.

## S6-T1 · Test architecture & coverage
*2 days (6 h) · due 2026-12-19*

**Why it matters.** Tests are documentation of invariants. Structure decides whether they stay useful.

**Read first**
- The Rust Book: Writing Automated Tests
- `cargo-llvm-cov` README

**Build.** Reorganize tests for `state_machine`: unit tests next to code, integration tests in `tests/`, doc tests on public APIs, shared fixtures via a builder.

**Acceptance criteria** (all required)
- [ ] Line coverage on `balances` and `proof_of_existence` is at least 90% (`cargo llvm-cov`)
- [ ] A test fixture builder used by at least 8 tests
- [ ] Every public function has a doc test or a justified omission
- [ ] Tests assert on error variants, not on strings

**Stretch.** Add a `#[should_panic]` test only where a panic is the contract and justify.

**Checkpoint questions** (answer in `journal/S6-T1.md`; the reviewer will ask them again)
1. What should unit vs integration tests each prove?
2. Why is coverage a floor and not a goal?
3. What makes a test brittle?

## S6-T2 · Property-based testing
*2 days (6 h) · due 2026-12-22*

**Why it matters.** Examples test what you thought of. Properties test what you did not.

**Read first**
- `proptest` book
- John Hughes: 'Don't write tests'

**Build.** Properties for `Balances`: total supply conserved; failed transfer is a no-op; transfer a→b then b→a restores state; no balance underflow.

**Acceptance criteria** (all required)
- [ ] At least 5 proptest properties with custom `Strategy` generating valid and invalid ops
- [ ] Shrinking demonstrated: plant a bug, show the minimal counterexample in the journal
- [ ] Regression file committed

**Stretch.** Write a property that finds an intentionally planted off-by-one in `proof_of_existence`.

**Checkpoint questions** (answer in `journal/S6-T2.md`; the reviewer will ask them again)
1. How does shrinking work?
2. How do you write a good generator for stateful operations?
3. What is the difference between a property and an example?

## S6-T3 · Model-based & stateful testing
*2 days (6 h) · due 2026-12-24*

**Why it matters.** The strongest test for a state machine is a trivially correct reference model run in lockstep.

**Read first**
- `proptest-state-machine` docs
- Hillel Wayne: model-based testing

**Build.** A `HashMap`-backed reference ledger, a random op-sequence generator, and a lockstep comparison against the real runtime after every step.

**Acceptance criteria** (all required)
- [ ] Op sequences of length up to 100 with `Transfer`, `Mint`, invalid ops
- [ ] State and result equality after every step
- [ ] A planted bug is caught and shrunk to at most 5 operations

**Stretch.** Add `execute_block` ops and compare block-level outcomes.

**Checkpoint questions** (answer in `journal/S6-T3.md`; the reviewer will ask them again)
1. Why must the model be simpler than the implementation?
2. What bugs does lockstep catch that properties miss?
3. How do you keep generators from producing only invalid ops?

## S6-T4 · Capstone: benchmarks & a measured optimization
*3 days (9 h) · due 2026-12-28*

**Why it matters.** Optimization without measurement is superstition. In a blockchain runtime, performance regressions are consensus risks.

**Read first**
- `criterion` book
- Rust Performance Book

**Build.** Criterion benchmarks for `transfer`, `execute_block` (100/1k/10k txs) and one hot data-structure operation; find a hotspot and fix it with evidence.

**Acceptance criteria** (all required)
- [ ] Benchmarks run with `cargo bench`, results summarized in `docs/perf/s06.md`
- [ ] A before/after comparison with confidence intervals
- [ ] Change keeps all tests and properties green
- [ ] Journal: what you ruled out and why

**Stretch.** Use `cargo flamegraph` or `samply` and include the flame graph.

**Checkpoint questions** (answer in `journal/S6-T4.md`; the reviewer will ask them again)
1. Why benchmark on `--release` only?
2. What is the difference between throughput and latency here?
3. How can an optimization alter determinism?

## S6-GATE · Stage gate
*1 day (3 h) · due 2026-12-29*

**Exam.** Closed book, 75 min: write a proptest for `transfer` that asserts conservation of supply and that failed transfers are no-ops. Explain model-based testing.

**Pass criteria.** Exam solved without notes or AI help, compiles, tests pass, and all oral answers are correct or corrected with a clear explanation. Remaining time: fix weak spots flagged by the reviewer and re-read failed checkpoints.
