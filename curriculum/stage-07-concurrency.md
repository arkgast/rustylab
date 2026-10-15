# Stage 7: Concurrency

**Outcome:** You write data-race-free parallel code, explain why the compiler accepts it, and prove parallel execution equals sequential.

**Window:** 2026-12-30 → 2027-01-14 · tracker: [`ROADMAP.md`](../ROADMAP.md)

Each task: read first, build in `tasks/` (or the crate named), write the journal entry, then run `/task-review <ID>`.

## S7-T1 · Threads, move & Send/Sync
*2 days (6 h) · due 2026-12-31*

**Why it matters.** `Send` and `Sync` are marker traits that make data races a compile error.

**Read first**
- The Rust Book: Fearless Concurrency
- Rustonomicon: Send and Sync

**Build.** Parallel sum/word-count with `thread::spawn` + `move`, then with `thread::scope`; a demo that `Rc` is rejected across threads.

**Acceptance criteria** (all required)
- [ ] Results match sequential implementation on randomized inputs
- [ ] 3 `compile_fail` doctests: Rc across threads, borrowed local in `spawn`, `Cell` shared by reference across threads
- [ ] Journal explains each in terms of `Send` / `Sync` / `'static`

**Stretch.** Implement `unsafe impl Send` on a wrapper and explain why it would be unsound.

**Checkpoint questions** (answer in `journal/S7-T1.md`; the reviewer will ask them again)
1. Define `Send` and `Sync` precisely and give a type that is `Send` but not `Sync`.
2. Why does `thread::spawn` require `'static` and scoped threads don't?
3. What happens to a panic in a spawned thread?

## S7-T2 · Shared state: Mutex, RwLock, deadlocks
*2 days (6 h) · due 2027-01-02*

**Why it matters.** Shared mutable state is how most real bugs happen. Make the failure modes visible.

**Read first**
- The Rust Book: Shared-State Concurrency
- Rust Atomics and Locks: ch. 1 Mutexes

**Build.** A concurrent `Bank` with `Arc<Mutex<..>>` per account; a transfer that deadlocks with naive locking; the fix with lock ordering; an `RwLock` read-heavy variant; contention measurement.

**Acceptance criteria** (all required)
- [ ] A test that reliably demonstrates the deadlock (with timeout) and one that passes after the fix
- [ ] Total money conserved under 8 threads and 100k random transfers
- [ ] Handles lock poisoning explicitly
- [ ] Journal: coarse vs fine-grained locking numbers

**Stretch.** Replace per-account Mutex with sharded locks and compare.

**Checkpoint questions** (answer in `journal/S7-T2.md`; the reviewer will ask them again)
1. What is lock poisoning and what should you do about it?
2. Why does consistent lock ordering prevent deadlock?
3. When is `RwLock` slower than `Mutex`?

## S7-T3 · Channels & worker pools
*2 days (6 h) · due 2027-01-05*

**Why it matters.** Message passing moves ownership across threads and is the natural fit for pipelines.

**Read first**
- std docs: `mpsc`
- `crossbeam-channel` docs

**Build.** A pipeline: producer → N workers → collector, with bounded channels for back-pressure and graceful shutdown on drop.

**Acceptance criteria** (all required)
- [ ] No busy-waiting; shutdown by closing channels
- [ ] Order of results documented (unordered or re-sequenced)
- [ ] A test that a slow consumer blocks the producer (back-pressure)
- [ ] All threads joined in `Drop`

**Stretch.** Add a `crossbeam` select with timeout.

**Checkpoint questions** (answer in `journal/S7-T3.md`; the reviewer will ask them again)
1. Why does closing the sender end the receiver loop?
2. Bounded vs unbounded: what failure does each choose?
3. How does sending a value relate to moving it?

## S7-T4 · Rayon & atomics
*2 days (6 h) · due 2027-01-07*

**Why it matters.** Data parallelism is the easiest speedup; atomics are the primitives under every lock.

**Read first**
- `rayon` docs
- Rust Atomics and Locks: ch. 2-3

**Build.** Parallel hash verification of 100k items with `par_iter`; an `AtomicU64` counter; a spinlock built from `AtomicBool` with correct orderings.

**Acceptance criteria** (all required)
- [ ] Speedup measured at 1/2/4/8 threads, with Amdahl's-law commentary
- [ ] Spinlock tested with 8 threads for mutual exclusion
- [ ] Journal justifies each `Ordering` used (Relaxed/Acquire/Release/SeqCst)

**Stretch.** Run the spinlock under `loom`.

**Checkpoint questions** (answer in `journal/S7-T4.md`; the reviewer will ask them again)
1. What does `Acquire`/`Release` establish that `Relaxed` does not?
2. Why is `par_iter().sum()` deterministic for integers but not floats?
3. What is false sharing?

## S7-T5 · Async basics
*2 days (6 h) · due 2027-01-09*

**Why it matters.** Async is cooperative concurrency for IO-bound work. You need the vocabulary even when a runtime hides it.

**Read first**
- The Async Book (intro chapters)
- `tokio` tutorial

**Build.** A tokio TCP line-echo server with a timeout and a shared counter, plus a hand-written `Future` (a countdown that wakes itself).

**Acceptance criteria** (all required)
- [ ] Server handles 100 concurrent clients in a test
- [ ] Hand-written `Future` implements `poll` correctly with a waker
- [ ] Journal: async vs threads, where each wins
- [ ] No blocking calls inside async fns (justify)

**Stretch.** Use `tokio::select!` with cancellation and explain cancel-safety.

**Checkpoint questions** (answer in `journal/S7-T5.md`; the reviewer will ask them again)
1. What does `.await` compile to?
2. Why must async code not block the executor thread?
3. What is `Pin` protecting?

## S7-T6 · Capstone: deterministic parallel executor
*3 days (9 h) · due 2027-01-13*

**Why it matters.** Blockchains need parallelism without nondeterminism. The result must equal sequential execution bit for bit.

**Read first**
- Read about Block-STM and Solana's Sealevel at overview level

**Build.** Partition transactions into conflict-free groups by read/write account sets and execute groups in parallel with Rayon; fall back to sequential for conflicts.

**Acceptance criteria** (all required)
- [ ] Property test: parallel result equals sequential for 1000 random blocks
- [ ] Conflict detection documented and tested (same account, read-after-write)
- [ ] Speedup benchmark on non-conflicting workloads
- [ ] Journal: why the result is deterministic despite scheduling

**Stretch.** Implement optimistic execution with re-execution on conflict.

**Checkpoint questions** (answer in `journal/S7-T6.md`; the reviewer will ask them again)
1. What makes the outcome independent of thread scheduling?
2. What is the cost of getting conflict detection wrong?
3. Where could nondeterminism still leak in (logs, maps, time)?

## S7-GATE · Stage gate
*1 day (3 h) · due 2027-01-14*

**Exam.** Closed book, 90 min: implement a worker pool with channels and graceful shutdown; explain `Send`/`Sync` and a deadlock you can cause and fix.

**Pass criteria.** Exam solved without notes or AI help, compiles, tests pass, and all oral answers are correct or corrected with a clear explanation. Remaining time: fix weak spots flagged by the reviewer and re-read failed checkpoints.
