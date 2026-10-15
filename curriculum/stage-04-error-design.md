# Stage 4: Error design

**Outcome:** You design error types as part of the API, audit code for panics, and make state changes atomic.

**Window:** 2026-11-27 → 2026-12-07 · tracker: [`ROADMAP.md`](../ROADMAP.md)

Each task: read first, build in `tasks/` (or the crate named), write the journal entry, then run `/task-review <ID>`.

## S4-T1 · Error taxonomy with thiserror
*2 days (6 h) · due 2026-11-28*

**Why it matters.** Errors are public API. Callers match on them to decide what to do next.

**Read first**
- `thiserror` docs
- Rust API Guidelines: Error types
- Blog: 'Error handling in a correctness-critical Rust project'

**Build.** A `BlockStoreError` hierarchy (Io, Decode, Integrity, NotFound) using `thiserror`, with `#[from]`, `#[source]`, and a helper that prints the full cause chain.

**Acceptance criteria** (all required)
- [ ] Each variant carries context (ids, offsets), not just strings
- [ ] A test asserting `source()` chain depth and `Display` text
- [ ] No `Box<dyn Error>` in the public API of the module
- [ ] Journal: which variants are retryable vs fatal

**Stretch.** Add `#[non_exhaustive]` and justify.

**Checkpoint questions** (answer in `journal/S4-T1.md`; the reviewer will ask them again)
1. Library errors vs application errors: which crate for which?
2. Why is a `String` error a smell?
3. How should an error expose its cause?

## S4-T2 · Panic audit of state_machine
*2 days (6 h) · due 2026-12-01*

**Why it matters.** A runtime that panics halts the chain. Finding and removing panics is real work you will do on real codebases.

**Read first**
- Clippy lints: `unwrap_used`, `expect_used`, `indexing_slicing`, `arithmetic_side_effects`

**Build.** Add the four lints as `deny` in `state_machine`, list every violation, fix them or justify with a documented `#[allow]`. Work on a branch.

**Acceptance criteria** (all required)
- [ ] `cargo clippy -p state_machine --all-targets -- -D warnings` is clean
- [ ] A journal table: location, kind of panic, fix
- [ ] Every remaining `#[allow]` has a comment proving the invariant
- [ ] Existing behavior preserved: all current tests still pass

**Stretch.** Enable `overflow-checks = true` in the release profile and justify.

**Checkpoint questions** (answer in `journal/S4-T2.md`; the reviewer will ask them again)
1. Which panics were real bugs and which were 'cannot happen'?
2. Why is `arithmetic_side_effects` important for a ledger?
3. What is the difference between `unwrap` and a documented invariant?

## S4-T3 · Atomic operations & rollback
*2 days (6 h) · due 2026-12-03*

**Why it matters.** A failed transaction must leave no trace. This is the single most important invariant of a state machine.

**Read first**
- Database transactions 101
- std docs: `mem::replace`, `mem::take`, `Clone`

**Build.** `Ledger::apply_batch(&mut self, Vec<Transfer>) -> Result<(), BatchError>`: all-or-nothing, implemented twice: (a) snapshot + restore, (b) validate-then-commit.

**Acceptance criteria** (all required)
- [ ] A test that fails mid-batch and asserts state is byte-identical to before
- [ ] Both strategies pass the same suite
- [ ] Journal: cost model of (a) vs (b), when each wins
- [ ] Failure index is reported in the error

**Stretch.** Make (b) work for batches where later transfers depend on earlier ones.

**Checkpoint questions** (answer in `journal/S4-T3.md`; the reviewer will ask them again)
1. Why is validate-then-commit cheaper but harder?
2. What must be true of the failure path for (a) to be correct?
3. How would this change with a database that can't clone?

## S4-T4 · Capstone: error policy for execute_block
*2 days (6 h) · due 2026-12-05*

**Why it matters.** Deciding which errors reject a block, which reject a transaction and which are just events is a protocol-design decision.

**Read first**
- Re-read `execute_block` and the dispatch code in `state_machine`

**Build.** Write `docs/adr/0001-error-policy.md` and align `state_machine` with it: classification of every error, who decides, what is logged and what happens to state.

**Acceptance criteria** (all required)
- [ ] ADR has a table: error, severity (tx-invalid / block-invalid / event), state effect
- [ ] Code matches the ADR, with tests for each row
- [ ] No catch-all `Box<dyn Error>`

**Stretch.** Add a `DispatchOutcome` receipt type and return one per extrinsic.

**Checkpoint questions** (answer in `journal/S4-T4.md`; the reviewer will ask them again)
1. What is the difference between an invalid transaction and a failed one?
2. Why does determinism forbid environment-dependent errors (IO, time)?
3. Who pays for a failed extrinsic and why does it matter?

## S4-GATE · Stage gate
*1 day (3 h) · due 2026-12-07*

**Exam.** Closed book, 60 min: given a function with 4 failure modes, design the error enum, write `Display`, and implement an all-or-nothing batch operation with a rollback test.

**Pass criteria.** Exam solved without notes or AI help, compiles, tests pass, and all oral answers are correct or corrected with a clear explanation. Remaining time: fix weak spots flagged by the reviewer and re-read failed checkpoints.
