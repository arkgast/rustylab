# Stage 7: Concurrency

**Phase:** II · Depth
**Outcome:** You explain why a type is or is not `Send`/`Sync`, share state without deadlocks, build pipelines and worker pools that shut down cleanly, choose memory orderings deliberately, and parallelize work while keeping the result bit-for-bit deterministic.

Each task: `/study` (recall, then probes: predict, run, explain the gap), solve the katas, run them and read the output, make the gate green, write the journal, then `/review <ID>`.

## S7-T1 · Threads, scoped threads, Send & Sync
*2 days (6 h)*

**Why it matters.** "Fearless concurrency" is two marker traits plus ownership. Once you see that a data race is a compile error, not a heisenbug, you can parallelize validators and executors with confidence.

**Probes**
- `thread::spawn` needs `'static + Send` and `move`; `JoinHandle<T>` returns results and panics
- `thread::scope`: borrow stack data from threads without `Arc`; naive "Arc + clone everything" vs scoped borrow side by side
- `Send` vs `Sync`; why `Rc` is `!Send` (non-atomic count), `RefCell` is `!Sync`, `MutexGuard` is `!Send`; auto traits and `PhantomData`

**Reference** (read after the probes, or when stuck)
- The Rust Book ch. 16.1 (Using Threads) and 16.4 (Extensible Concurrency with Send and Sync)
- Rust Atomics and Locks (Mara Bos) ch. 1 (Basics of Rust Concurrency)
- The Rustonomicon: "Send and Sync"

**Build.** `tasks/src/s07/t01_threads.rs`: `parallel_sum(&[u64], threads) -> Result<u128, SumError>` with `thread::scope`; `parallel_map<T, U>(Vec<T>, f) -> Vec<U>` preserving order; `chunked_hash_count(&[Vec<u8>]) -> usize`; plus `compile_fail` doctests showing `Rc` cannot cross `thread::spawn` (E0277) and a borrowed slice cannot go into `thread::spawn` (E0373).

**Run.** `scripts/run.sh S7-T1` prints per-thread chunks and timings.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s07_t01.rs` pass
- [ ] No `Arc` where `thread::scope` suffices; no `unsafe impl Send`
- [ ] A panicking worker yields a typed error, not a process abort
- [ ] Two `compile_fail` doctests with error codes and rule/fix comments
- [ ] `parallel_sum` result equals the sequential sum for empty, 1-element and odd-length inputs

**Stretch.** Write a type with a raw pointer field, show it is `!Send`, and explain in the journal what you would need to prove to `unsafe impl Send` (do not ship it).

**Checkpoint questions** (answer in `journal/S7-T1.md`; the reviewer asks them again)
1. Why is `Rc<T>` `!Send` while `Arc<T>` is `Send` (given `T: Send + Sync`)?
2. Why does `thread::spawn` require `'static`, and how does `thread::scope` remove that requirement soundly?
3. `T: Sync` is defined as `&T: Send`. Explain that definition with an example.

## S7-T2 · Shared state: Arc, Mutex, RwLock & deadlocks
*2 days (6 h)*

**Why it matters.** Mempools, caches and peer tables are shared across threads. Locks are easy to use and easy to misuse: hold one too long and throughput dies; take two in different orders and the node hangs.

**Probes**
- `Arc<Mutex<T>>`: the guard as RAII; holding a guard across slow work vs copying out and releasing
- Deadlock live: two threads, two locks, opposite order (with a timeout in the demo); fix with a global lock order
- Poisoning: what it means, recovering with `into_inner` vs propagating as an error; `RwLock` and writer starvation

**Reference** (read after the probes, or when stuck)
- The Rust Book ch. 16.3 (Shared-State Concurrency)
- Rust Atomics and Locks ch. 1 (sections on Mutex, RwLock, poisoning, condvars)
- std docs: `Mutex::lock`, `PoisonError::into_inner`, `RwLock`, `Condvar`

**Build.** `tasks/src/s07/t02_shared_state.rs`: `Bank { accounts: Vec<Mutex<Account>> }` with `transfer(from, to, amount)` that locks in ascending id order and handles `from == to`; `Mempool` behind `RwLock` (`insert`, `contains`, `drain_by_fee`); `Counter` with poisoning mapped to `Err(Poisoned)`; a stress test with 8 threads x 10 000 random transfers.

**Run.** `scripts/run.sh S7-T2` runs the stress test and prints total supply before and after.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s07_t02.rs` pass, including the stress test under `--release`
- [ ] Locks always acquired in a documented global order; journal draws the order
- [ ] No `lock().unwrap()` in library code; poisoning is a typed error
- [ ] No guard held across I/O, sleep or a user callback
- [ ] Total supply is conserved after the stress test

**Stretch.** Replace `Vec<Mutex<Account>>` with one `Mutex<Vec<Account>>`, benchmark both under contention, and explain the result.

**Checkpoint questions** (answer in `journal/S7-T2.md`; the reviewer asks them again)
1. State the four Coffman conditions and which one your lock ordering breaks.
2. Why does Rust poison a mutex, and when is `into_inner` the right recovery?
3. When is `RwLock` slower than `Mutex`?

## S7-T3 · Channels & worker pools
*2 days (6 h)*

**Why it matters.** Message passing replaces shared state with ownership transfer. Nodes are pipelines: network -> decode -> verify signatures -> execute -> persist. A worker pool that cannot shut down cleanly loses transactions on restart.

**Probes**
- `mpsc` unbounded vs `sync_channel(n)` bounded: backpressure, and memory growth when the consumer is slow
- Disconnection as the shutdown signal: drop all senders, `recv` returns `Err`, workers exit
- crossbeam `select!` across a work channel and a shutdown channel; fan-out / fan-in

**Reference** (read after the probes, or when stuck)
- The Rust Book ch. 16.2 (Using Message Passing) and ch. 21.2-21.3 (thread pool and graceful shutdown)
- crossbeam-channel docs: `bounded`, `select!`, `never`, disconnection semantics
- Rust Atomics and Locks ch. 5 (Building Our Own Channels), read for ideas

**Build.** `tasks/src/s07/t03_channels.rs`: `ThreadPool::new(n) -> Result<Self, PoolError>`, `execute(job) -> Result<(), PoolError>`, `shutdown(self) -> Vec<JobReport>` and a `Drop` that joins all workers; a 3-stage pipeline `parse -> verify -> apply` over bounded channels that preserves input order in the final output.

**Run.** `scripts/run.sh S7-T3` prints jobs per worker and the shutdown sequence.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s07_t03.rs` pass
- [ ] Every job submitted before `shutdown` runs exactly once; jobs after shutdown return `Err`
- [ ] A panicking job does not kill its worker or the pool
- [ ] Pipeline channels are bounded; journal states the chosen capacity and why
- [ ] No detached threads: every `JoinHandle` is joined

**Stretch.** Swap `std::sync::mpsc` for crossbeam and add a `shutdown_timeout(Duration)` using `select!` on a deadline.

**Checkpoint questions** (answer in `journal/S7-T3.md`; the reviewer asks them again)
1. Why is an unbounded channel a memory leak waiting for a slow consumer?
2. How does dropping senders shut the pool down, and what must happen to the receiver for that to work?
3. When would you choose channels over `Arc<Mutex<_>>`, and vice versa?

## S7-T4 · Atomics, memory ordering & rayon
*2 days (6 h)*

**Why it matters.** Atomics are the floor every lock and channel is built on. Get `Ordering` wrong and code passes tests on x86 and fails on ARM. Rayon is the safe, high-level path that most parallel chain code actually uses.

**Probes**
- `fetch_add`, `compare_exchange` loops; `Relaxed` for counters, `Release`/`Acquire` for publishing data, `SeqCst` and when it is really needed
- A spinlock: `AtomicBool` + `UnsafeCell<T>` + guard; why `lock` needs `Acquire` and `unlock` needs `Release`
- rayon: sequential loop vs `par_iter().map().sum()` vs manual threads; work stealing; `reduce` needs associativity

**Reference** (read after the probes, or when stuck)
- Rust Atomics and Locks ch. 2 (Atomics), ch. 3 (Memory Ordering), ch. 4 (Building Our Own Spin Lock)
- rayon docs: `par_iter`, `join`, `ThreadPoolBuilder`; "Rayon: data parallelism in Rust" (Niko Matsakis)

**Build.** `tasks/src/s07/t04_atomics.rs`: `StatsCounter` (Relaxed counters, snapshot), `OnceFlag` publishing a value with Release/Acquire, `SpinLock<T>` with an RAII guard (the one place `unsafe` is allowed this stage, every block with a `// SAFETY:` comment), and `par_verify(&[Tx]) -> Vec<Result<(), VerifyError>>` with rayon preserving order.

**Run.** `scripts/run.sh S7-T4`; `cargo +nightly miri test -p tasks --test s07_t04` if Miri is installed.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s07_t04.rs` pass, including an 8-thread spinlock stress test
- [ ] Every `Ordering` choice has a one-line justification comment
- [ ] Every `unsafe` block has a `// SAFETY:` comment stating the invariant
- [ ] `SpinLock<T>` is `Sync` only when `T: Send`
- [ ] `par_verify` output is identical to sequential output

**Stretch.** Use `loom` to model-check `OnceFlag` and report what it finds if you weaken Release to Relaxed.

**Checkpoint questions** (answer in `journal/S7-T4.md`; the reviewer asks them again)
1. What does a Release store followed by an Acquire load of the same value guarantee about other memory?
2. Why is `Relaxed` fine for a statistics counter but wrong for a "data ready" flag?
3. Why must the operation in a rayon `reduce` be associative, and what breaks if it is not?

## S7-T5 · Capstone: deterministic parallel transaction executor
*3 days (9 h)*

**Why it matters.** Every validator must reach the same state from the same block. Solana's Sealevel parallelizes by requiring transactions to declare their read/write account sets up front; Aptos/Sui's Block-STM executes optimistically and re-runs on conflict. Both must produce exactly the state sequential execution would. That is your test oracle.

**Probes**
- Read/write sets and conflicts: W-W and R-W on the same key; building non-conflicting batches greedily in block order
- Determinism traps: `HashMap` iteration order, thread-completion order, floats, wall-clock time
- State root: hash a `BTreeMap` canonically; compare sequential vs parallel roots as the core test

**Reference** (read after the probes, or when stuck)
- Solana docs: "Sealevel: parallel processing of smart contracts" (blog) and the account-locking section of the runtime docs
- Block-STM paper (Gelashvili et al., 2022): sections 1-3
- Re-read your S7-T2, S7-T3, S7-T4 journals

**Build.** `tasks/src/s07/t05_parallel_exec.rs`: `Tx { id, reads: BTreeSet<Key>, writes: BTreeSet<Key>, op }`, `schedule(&[Tx]) -> Vec<Batch>` (conflict-free batches preserving relative order of conflicting txs), `execute_sequential(&State, &[Tx]) -> Result<(State, Receipts), ExecError>`, `execute_parallel(&State, &[Tx], threads) -> Result<(State, Receipts), ExecError>` using rayon or scoped threads, and `state_root(&State) -> [u8; 32]`. ADR in `docs/adr/` on scheduling strategy.

**Run.** `scripts/run.sh S7-T5` prints batches, speedup and both state roots.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s07_t05.rs` pass
- [ ] A proptest over random tx sets asserts parallel root == sequential root and equal receipts
- [ ] A tx touching a key outside its declared sets is rejected with a typed error, not silently run
- [ ] No `HashMap`/`HashSet` in state or scheduling; checked arithmetic everywhere
- [ ] Speedup measured on a low-conflict and a high-conflict workload; numbers in the journal
- [ ] ADR written

**Stretch.** Implement a minimal optimistic executor (Block-STM-lite): run all txs in parallel against a snapshot, detect read-set invalidation, re-execute in order, and compare speedup with your scheduler.

**Checkpoint questions** (answer in `journal/S7-T5.md`; the reviewer asks them again)
1. Why is "same state root as sequential execution" the right correctness definition?
2. Sealevel vs Block-STM: what does each require from transactions, and what does each do on conflict?
3. Name two places nondeterminism could leak into your executor and how you closed each.

## S7-GATE · Stage 7 gate: closed-book exam + catch-up
*1 day (3 h)*

**Exam.** Closed book, 90 min, blank file `tasks/src/s07_gate.rs`: write `parallel_word_count(&[&str], threads) -> BTreeMap<String, u64>` with `thread::scope` and a channel fan-in, equal to the sequential result; a `Bank::transfer` over `Vec<Mutex<u64>>` that cannot deadlock (with a short justification); and an `AtomicU64` ID generator with a justified `Ordering`. Explain in writing why a given snippet using `Rc` across threads fails to compile. Then answer 5 oral questions drawn from the S7 checkpoints.

**Pass criteria.** Solved without notes or AI, compiles, tests pass, no deadlock in a 4-thread stress run, oral answers correct or corrected with a clear explanation. Remaining time: redo the weakest task's core from a blank file.
