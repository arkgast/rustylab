# Stage 9: State machines

**Outcome:** You can design a deterministic state transition system where invalid states are unrepresentable and every failure is a clean rollback.

**Window:** 2027-01-29 → 2027-02-13 · tracker: [`ROADMAP.md`](../ROADMAP.md)

Each task: read first, build in `tasks/` (or the crate named), write the journal entry, then run `/task-review <ID>`.

## S9-T1 · Typestate pattern
*2 days (6 h) · due 2027-01-30*

**Why it matters.** Move invalid states from runtime errors to compile errors.

**Read first**
- Cliffle: 'The Typestate Pattern in Rust'
- `PhantomData` in std docs

**Build.** `Transaction<Unsigned>` → `Transaction<Signed>` → `Transaction<Included>` using zero-sized marker types; only signed transactions can be submitted.

**Acceptance criteria** (all required)
- [ ] `compile_fail` doctests: submit unsigned, sign twice, modify after signing
- [ ] Size of `Transaction<S>` is identical for all `S` (asserted)
- [ ] Journal: typestate vs runtime enum state, tradeoffs

**Stretch.** Make the signer a type parameter too.

**Checkpoint questions** (answer in `journal/S9-T1.md`; the reviewer will ask them again)
1. What does the compiler prove with typestate that tests can't?
2. Where does typestate stop working (dynamic input)?
3. Why `PhantomData` and not a field?

## S9-T2 · Pure transition functions
*2 days (6 h) · due 2027-02-02*

**Why it matters.** `apply(state, tx) -> Result<state, error>` is the clearest definition of a state machine. Determinism is a property you can test.

**Read first**
- Ethereum yellow paper: state transition function (overview)
- Review `execute_block`

**Build.** Refactor one pallet into `fn apply(&State, &Tx) -> Result<State, Error>` plus an in-place wrapper; show equivalence.

**Acceptance criteria** (all required)
- [ ] Property: `apply` is deterministic (same inputs → same state hash)
- [ ] Property: in-place wrapper equals pure version on 1000 random sequences
- [ ] State has a canonical `state_root` (BTreeMap based)
- [ ] Journal: cost of cloning state and when in-place is justified

**Stretch.** Use persistent data structures (`im` crate) and compare.

**Checkpoint questions** (answer in `journal/S9-T2.md`; the reviewer will ask them again)
1. What makes a function deterministic and what could break it?
2. Pure vs in-place: what do you gain/lose?
3. How do you define 'the same state' for comparing results?

## S9-T3 · Overlay storage & nested transactions
*3 days (9 h) · due 2027-02-05*

**Why it matters.** Commit/rollback with savepoints is how runtimes isolate failed calls.

**Read first**
- Substrate docs: storage transactional layers
- SQLite savepoints

**Build.** `Overlay<B: Backend>` with `get/set/remove`, `begin()`, `commit()`, `rollback()` supporting nesting depth N and a changeset iterator.

**Acceptance criteria** (all required)
- [ ] Nested rollback restores exactly the outer state (property test)
- [ ] Reads see uncommitted writes; backend untouched until final commit
- [ ] Deleting then re-setting a key behaves correctly
- [ ] A test for 5 levels of nesting with mixed commit/rollback

**Stretch.** Add a size/depth limit with a typed error.

**Checkpoint questions** (answer in `journal/S9-T3.md`; the reviewer will ask them again)
1. How is the overlay structured (stack of maps vs journal) and why?
2. What is the cost of rollback in your design?
3. How does this relate to `execute_block` atomicity?

## S9-T4 · Blocks, nonces & a mempool
*3 days (9 h) · due 2027-02-09*

**Why it matters.** Replay protection and ordering rules are what turn a state machine into a ledger.

**Read first**
- Bitcoin/Ethereum nonce and mempool overviews

**Build.** `Block { header, extrinsics }`, header hash chaining, per-account nonce validation, `execute_block` over the overlay, and a `Mempool` ordering by fee with per-account nonce gaps.

**Acceptance criteria** (all required)
- [ ] Replayed and out-of-order nonces rejected with typed errors
- [ ] Invalid block leaves state untouched (test)
- [ ] Mempool never yields a transaction before its predecessor nonce
- [ ] Block hash covers header and extrinsics root

**Stretch.** Add a block gas/weight limit.

**Checkpoint questions** (answer in `journal/S9-T4.md`; the reviewer will ask them again)
1. Why are nonces needed and what attack does each prevent?
2. How does the mempool differ from the block's validity rules?
3. Which checks are stateless and which stateful?

## S9-T5 · Capstone: add a new pallet end to end
*3 days (9 h) · due 2027-02-12*

**Why it matters.** Designing a pallet from scratch exercises everything so far: types, traits, errors, macros, tests.

**Read first**
- Pick one with the reviewer: `staking-lite`, `voting`, `multisig`, `escrow`

**Build.** A new pallet in `state_machine` wired via `runtime!`/`call!`, with its own `Config`, `Call`, `Error`, storage and events.

**Acceptance criteria** (all required)
- [ ] Design doc (1 page) before code: state, calls, errors, invariants
- [ ] Properties: at least 3 invariants as proptests; model test against a reference
- [ ] No panics; clippy deny-lints from S4 clean
- [ ] README section describing the pallet and its extrinsics

**Stretch.** Add a genesis config and migrations story note.

**Checkpoint questions** (answer in `journal/S9-T5.md`; the reviewer will ask them again)
1. List the invariants of your pallet and where each is enforced.
2. What happens on partial failure within a call?
3. What would break if two pallets mutate the same account?

## S9-GATE · Stage gate
*1 day (3 h) · due 2027-02-13*

**Exam.** Closed book, 90 min: implement `apply_block` over an overlay with commit/rollback and a nonce check; prove determinism by test.

**Pass criteria.** Exam solved without notes or AI help, compiles, tests pass, and all oral answers are correct or corrected with a clear explanation. Remaining time: fix weak spots flagged by the reviewer and re-read failed checkpoints.
