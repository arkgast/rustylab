# RustyLab Roadmap

> This file is the source of truth for progress. Do not tick boxes by hand: a task changes to ✅ only when `/task-review` passes (see `CLAUDE.md`).

## Progress

<!-- progress:start -->
- Tasks done: **0 / 50**
- Study days planned: **125** (3 h/day = 375 h)
- Started: **2026-10-16** · Planned finish: **2027-03-10**
- Slip: **0** study days
<!-- progress:end -->

## Rules

- **3 hours of focused study every day, Monday to Saturday.** Sunday is rest, spaced repetition and catch-up.
- A day = 20 min recall (blank page, no notes) · 30 min concept + reading · 100 min building · 20 min self-review (`fmt`, `clippy`, refactor) · 10 min journal.
- A task is **done** only when the review gate passes: automated gate green + rubric score ≥ 80 + no blocker. See [`curriculum/README.md`](curriculum/README.md).
- Missing a due date does not skip a task. Remaining tasks slip by the same number of study days (`scripts/roadmap.py` does this for you).
- Each stage ends with a **gate day**: a closed-book exam plus catch-up. Fail it and you repeat the weak task before moving on.

Status legend: ⬜ todo · 🔨 doing · 🔁 retry (review failed) · ✅ done

## Stage 1: Foundations

*You can predict, before compiling, whether code passes the borrow checker, and you model failure with Option/Result instead of panics.*  
Spec: [`curriculum/stage-01-foundations.md`](curriculum/stage-01-foundations.md) · Window: 2026-10-16 → 2026-10-29

| ID | Task | Days | Due | Status | Done | Score |
|----|------|------|-----|--------|------|-------|
| S1-T1 | Ownership & borrowing lab | 2 | 2026-10-17 | ⬜ todo | — | — |
| S1-T2 | Slices, strings & UTF-8 | 2 | 2026-10-20 | ⬜ todo | — | — |
| S1-T3 | Option & Result without unwrap | 2 | 2026-10-22 | ⬜ todo | — | — |
| S1-T4 | Enums & exhaustive matching | 2 | 2026-10-24 | ⬜ todo | — | — |
| S1-T5 | Capstone: in-memory key-value store | 3 | 2026-10-28 | ⬜ todo | — | — |
| S1-GATE | Stage 1 gate: closed-book exam + catch-up | 1 | 2026-10-29 | ⬜ todo | — | — |

## Stage 2: Types & traits

*You model a domain with types so invalid states do not compile, and you choose between generics and trait objects deliberately.*  
Spec: [`curriculum/stage-02-types-and-traits.md`](curriculum/stage-02-types-and-traits.md) · Window: 2026-10-30 → 2026-11-12

| ID | Task | Days | Due | Status | Done | Score |
|----|------|------|-----|--------|------|-------|
| S2-T1 | Newtypes & domain modeling | 2 | 2026-10-31 | ⬜ todo | — | — |
| S2-T2 | Traits & generic dispatch | 2 | 2026-11-03 | ⬜ todo | — | — |
| S2-T3 | Conversions & error plumbing | 2 | 2026-11-05 | ⬜ todo | — | — |
| S2-T4 | Standard traits & fixed-point math | 2 | 2026-11-07 | ⬜ todo | — | — |
| S2-T5 | Capstone: Config trait with associated types | 3 | 2026-11-11 | ⬜ todo | — | — |
| S2-GATE | Stage 2 gate: closed-book exam + catch-up | 1 | 2026-11-12 | ⬜ todo | — | — |

## Stage 3: Lifetimes & memory

*You can reason about where data lives, how long, and who may touch it, and you pick between Box, Rc, Arc and indices with a reason.*  
Spec: [`curriculum/stage-03-lifetimes-and-memory.md`](curriculum/stage-03-lifetimes-and-memory.md) · Window: 2026-11-13 → 2026-11-26

| ID | Task | Days | Due | Status | Done | Score |
|----|------|------|-----|--------|------|-------|
| S3-T1 | Lifetimes & zero-copy parsing | 2 | 2026-11-14 | ⬜ todo | — | — |
| S3-T2 | Memory layout & niches | 2 | 2026-11-17 | ⬜ todo | — | — |
| S3-T3 | Smart pointers: Box, Rc, Weak, Drop | 2 | 2026-11-19 | ⬜ todo | — | — |
| S3-T4 | Interior mutability | 2 | 2026-11-21 | ⬜ todo | — | — |
| S3-T5 | Capstone: typed arena & AST | 3 | 2026-11-25 | ⬜ todo | — | — |
| S3-GATE | Stage 3 gate: closed-book exam + catch-up | 1 | 2026-11-26 | ⬜ todo | — | — |

## Stage 4: Error design

*You design error types as part of the API, audit code for panics, and make state changes atomic.*  
Spec: [`curriculum/stage-04-error-design.md`](curriculum/stage-04-error-design.md) · Window: 2026-11-27 → 2026-12-07

| ID | Task | Days | Due | Status | Done | Score |
|----|------|------|-----|--------|------|-------|
| S4-T1 | Error taxonomy with thiserror | 2 | 2026-11-28 | ⬜ todo | — | — |
| S4-T2 | Panic audit of state_machine | 2 | 2026-12-01 | ⬜ todo | — | — |
| S4-T3 | Atomic operations & rollback | 2 | 2026-12-03 | ⬜ todo | — | — |
| S4-T4 | Capstone: error policy for execute_block | 2 | 2026-12-05 | ⬜ todo | — | — |
| S4-GATE | Stage 4 gate: closed-book exam + catch-up | 1 | 2026-12-07 | ⬜ todo | — | — |

## Stage 5: Collections & iterators

*You choose collections by ordering, determinism and complexity, and you write iterator pipelines that allocate only when they must.*  
Spec: [`curriculum/stage-05-collections-and-iterators.md`](curriculum/stage-05-collections-and-iterators.md) · Window: 2026-12-08 → 2026-12-17

| ID | Task | Days | Due | Status | Done | Score |
|----|------|------|-----|--------|------|-------|
| S5-T1 | HashMap vs BTreeMap & determinism | 2 | 2026-12-09 | ⬜ todo | — | — |
| S5-T2 | Writing iterators | 2 | 2026-12-11 | ⬜ todo | — | — |
| S5-T3 | Closures & an LRU cache | 2 | 2026-12-14 | ⬜ todo | — | — |
| S5-T4 | Capstone: event log & indexer | 2 | 2026-12-16 | ⬜ todo | — | — |
| S5-GATE | Stage 5 gate: closed-book exam + catch-up | 1 | 2026-12-17 | ⬜ todo | — | — |

## Stage 6: Testing & correctness

*You specify invariants, test them with generated inputs against a reference model, and measure performance before optimizing.*  
Spec: [`curriculum/stage-06-testing-and-correctness.md`](curriculum/stage-06-testing-and-correctness.md) · Window: 2026-12-18 → 2026-12-29

| ID | Task | Days | Due | Status | Done | Score |
|----|------|------|-----|--------|------|-------|
| S6-T1 | Test architecture & coverage | 2 | 2026-12-19 | ⬜ todo | — | — |
| S6-T2 | Property-based testing | 2 | 2026-12-22 | ⬜ todo | — | — |
| S6-T3 | Model-based & stateful testing | 2 | 2026-12-24 | ⬜ todo | — | — |
| S6-T4 | Capstone: benchmarks & a measured optimization | 3 | 2026-12-28 | ⬜ todo | — | — |
| S6-GATE | Stage 6 gate: closed-book exam + catch-up | 1 | 2026-12-29 | ⬜ todo | — | — |

## Stage 7: Concurrency

*You write data-race-free parallel code, explain why the compiler accepts it, and prove parallel execution equals sequential.*  
Spec: [`curriculum/stage-07-concurrency.md`](curriculum/stage-07-concurrency.md) · Window: 2026-12-30 → 2027-01-14

| ID | Task | Days | Due | Status | Done | Score |
|----|------|------|-----|--------|------|-------|
| S7-T1 | Threads, move & Send/Sync | 2 | 2026-12-31 | ⬜ todo | — | — |
| S7-T2 | Shared state: Mutex, RwLock, deadlocks | 2 | 2027-01-02 | ⬜ todo | — | — |
| S7-T3 | Channels & worker pools | 2 | 2027-01-05 | ⬜ todo | — | — |
| S7-T4 | Rayon & atomics | 2 | 2027-01-07 | ⬜ todo | — | — |
| S7-T5 | Async basics | 2 | 2027-01-09 | ⬜ todo | — | — |
| S7-T6 | Capstone: deterministic parallel executor | 3 | 2027-01-13 | ⬜ todo | — | — |
| S7-GATE | Stage 7 gate: closed-book exam + catch-up | 1 | 2027-01-14 | ⬜ todo | — | — |

## Stage 8: Macros

*You can read and write declarative and procedural macros, test them, and give good diagnostics.*  
Spec: [`curriculum/stage-08-macros.md`](curriculum/stage-08-macros.md) · Window: 2027-01-15 → 2027-01-28

| ID | Task | Days | Due | Status | Done | Score |
|----|------|------|-----|--------|------|-------|
| S8-T1 | macro_rules! fundamentals | 2 | 2027-01-16 | ⬜ todo | — | — |
| S8-T2 | Advanced declarative macros | 2 | 2027-01-19 | ⬜ todo | — | — |
| S8-T3 | Derive macros with syn & quote | 2 | 2027-01-21 | ⬜ todo | — | — |
| S8-T4 | Attribute & function-like macros | 2 | 2027-01-23 | ⬜ todo | — | — |
| S8-T5 | Capstone: extend the runtime macros | 3 | 2027-01-27 | ⬜ todo | — | — |
| S8-GATE | Stage 8 gate: closed-book exam + catch-up | 1 | 2027-01-28 | ⬜ todo | — | — |

## Stage 9: State machines

*You can design a deterministic state transition system where invalid states are unrepresentable and every failure is a clean rollback.*  
Spec: [`curriculum/stage-09-state-machines.md`](curriculum/stage-09-state-machines.md) · Window: 2027-01-29 → 2027-02-13

| ID | Task | Days | Due | Status | Done | Score |
|----|------|------|-----|--------|------|-------|
| S9-T1 | Typestate pattern | 2 | 2027-01-30 | ⬜ todo | — | — |
| S9-T2 | Pure transition functions | 2 | 2027-02-02 | ⬜ todo | — | — |
| S9-T3 | Overlay storage & nested transactions | 3 | 2027-02-05 | ⬜ todo | — | — |
| S9-T4 | Blocks, nonces & a mempool | 3 | 2027-02-09 | ⬜ todo | — | — |
| S9-T5 | Capstone: add a new pallet end to end | 3 | 2027-02-12 | ⬜ todo | — | — |
| S9-GATE | Stage 9 gate: closed-book exam + catch-up | 1 | 2027-02-13 | ⬜ todo | — | — |

## Stage 10: Protocol engineering

*You can build the core of a small blockchain from primitives and explain every design decision, then map it to FRAME/Substrate.*  
Spec: [`curriculum/stage-10-protocol-engineering.md`](curriculum/stage-10-protocol-engineering.md) · Window: 2027-02-15 → 2027-03-10

| ID | Task | Days | Due | Status | Done | Score |
|----|------|------|-----|--------|------|-------|
| S10-T1 | Hashing & content addressing | 2 | 2027-02-16 | ⬜ todo | — | — |
| S10-T2 | Merkle trees & proofs | 3 | 2027-02-19 | ⬜ todo | — | — |
| S10-T3 | Signatures & replay protection | 3 | 2027-02-23 | ⬜ todo | — | — |
| S10-T4 | Serialization & a SCALE-like codec | 3 | 2027-02-26 | ⬜ todo | — | — |
| S10-T5 | Consensus toy: PoA/PoW & fork choice | 3 | 2027-03-02 | ⬜ todo | — | — |
| S10-T6 | Networking-lite: gossip simulation | 3 | 2027-03-05 | ⬜ todo | — | — |
| S10-T7 | Capstone: no_std core & FRAME mapping | 3 | 2027-03-09 | ⬜ todo | — | — |
| S10-GATE | Stage 10 gate: closed-book exam + catch-up | 1 | 2027-03-10 | ⬜ todo | — | — |
