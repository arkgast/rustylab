# Stage 8: Async Rust

**Phase:** II · Depth
**Outcome:** You explain what `.await` compiles to, write a future and an executor by hand, build tokio services that time out, cancel and shut down cleanly under load, and you know which code must never run on an async worker thread.

Each task: run the lesson, solve the katas, run them and read the output, make the gate green, write the journal, then `/task-review <ID>`.

## S8-T1 · Futures, Poll & Waker: a hand-written executor
*2 days (6 h)*

**Why it matters.** Async is a state machine the compiler writes for you, driven by a runtime you did not write. Until you write both once, `Pin`, `Send` bounds on futures and "my task never wakes up" are magic. Every node's networking layer sits on this.

**Read first**
- Asynchronous Programming in Rust (the Async Book): "Under the Hood" chapters (Future trait, Wakers, Executors, Pinning)
- std docs: `std::future::Future`, `std::task::{Poll, Context, Waker, Wake}`, `std::pin::Pin`
- "Pin" (without.boats blog) or the `std::pin` module docs, sections on structural pinning

**Lesson** (`scripts/run.sh S8-T1 learn`)
- `async fn` desugared: an enum state machine with one variant per `.await`; why it may be self-referential and therefore needs `Pin`
- `poll` returns `Ready` or `Pending`; the contract: return `Pending` only after arranging a wake; a busy-loop "executor" vs a waker-driven one, CPU usage printed
- `Wake` trait + `Arc<Task>` + a channel = a minimal executor; `block_on` with a thread-park waker

**Build.** `tasks/src/s08/t01_futures.rs` (std only, no tokio): `Countdown` future that is `Pending` n times and wakes itself; `TimerFuture` that a helper thread completes; `block_on<F: Future>(f) -> F::Output`; `MiniExecutor` with `spawn` and `run` over a channel of `Arc<Task>`; `join2(a, b)` combinator written by hand.

**Run.** `scripts/run.sh S8-T1` prints every `poll` and `wake` call in order.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s08_t01.rs` pass
- [ ] No busy-polling: a test asserts `poll` is called at most once per wake
- [ ] `unsafe` only for pin projection if at all, each with `// SAFETY:`; prefer `Box::pin` / `pin!`
- [ ] A `compile_fail` doctest shows a `!Unpin` future cannot be polled without pinning
- [ ] Journal: the `Countdown` state machine drawn as states and transitions

**Stretch.** Make `MiniExecutor` multi-threaded with a shared queue and show which bound (`Send`) the spawned futures now need, and why.

**Checkpoint questions** (answer in `journal/S8-T1.md`; the reviewer asks them again)
1. What does `Pin` guarantee, and why does an `async` block need that guarantee?
2. What happens if a future returns `Pending` without registering the waker?
3. Why are futures lazy, and what does that mean for code that builds a future but never awaits it?

## S8-T2 · tokio: tasks, IO, timeouts & JoinSet
*2 days (6 h)*

**Why it matters.** Polkadot, Solana and Stellar nodes and their RPC services run on tokio. Spawning, timing out and collecting tasks correctly is daily work, and one blocking call on a runtime thread stalls every peer connection on it.

**Read first**
- tokio tutorial: "Hello Tokio", "Spawning", "Shared state", "Channels", "I/O", "Framing"
- tokio docs: `task::JoinSet`, `time::timeout`, `task::spawn_blocking`, `sync::Semaphore`
- Alice Ryhl: "Async: What is blocking?"

**Lesson** (`scripts/run.sh S8-T2 learn`)
- `#[tokio::main]`, multi-thread vs current-thread runtime; `tokio::spawn` needs `Send + 'static`, and holding a `std::sync::MutexGuard` across `.await` makes a future `!Send`
- Blocking the executor live: `std::thread::sleep` vs `tokio::time::sleep` in 100 tasks, timings printed; `spawn_blocking` for CPU work
- `JoinSet` to collect results and abort the rest; `timeout` wrapping any future; `Semaphore` to cap concurrency

**Build.** `labs/async-lab/` (binary crate with tokio, tokio-util): `echo-server` on `TcpListener` with per-connection tasks and line framing; `fetch_all(addrs, max_in_flight) -> Vec<Result<Response, FetchError>>` with `JoinSet`, `Semaphore` and a per-request `timeout`; a `hash_blocks` endpoint that offloads hashing with `spawn_blocking`.

**Run.** `cd labs/async-lab && cargo run --bin echo-server` and `cargo run --bin fetch-all`; `cargo test -p async-lab`.

**Acceptance criteria** (all required)
- [ ] Tests pass under `#[tokio::test]`, including one with `start_paused = true` for timeouts
- [ ] No blocking call (`std::thread::sleep`, sync file IO, heavy CPU loop) on a runtime thread
- [ ] No `std::sync::Mutex` guard held across `.await`
- [ ] `fetch_all` never exceeds `max_in_flight` (asserted with a counter)
- [ ] Every error is typed (`thiserror`); no `unwrap` outside tests and `main`

**Stretch.** Run with `tokio-console` and capture a screenshot of a task blocked on a slow peer.

**Checkpoint questions** (answer in `journal/S8-T2.md`; the reviewer asks them again)
1. Why does `tokio::spawn` require `Send + 'static`, and how can a single `.await` make a future `!Send`?
2. What exactly goes wrong when you block inside an async task on a multi-thread runtime?
3. When do you pick `tokio::sync::Mutex` over `std::sync::Mutex`?

## S8-T3 · select!, cancellation safety & graceful shutdown
*2 days (6 h)*

**Why it matters.** In async Rust, cancellation is dropping a future at any `.await`. If that drop loses a half-read message or a popped transaction, the node silently drops data. Graceful shutdown is how a validator restarts without corrupting state.

**Read first**
- tokio tutorial: "Select", "Graceful Shutdown"
- tokio docs: `select!` (cancellation safety section), `sync::mpsc::channel` (bounded), `tokio_util::sync::CancellationToken`, `task::TaskTracker`
- "Cancelling async Rust" (sunshowers / Rain, RustConf 2024 talk or blog)

**Lesson** (`scripts/run.sh S8-T3 learn`)
- `select!` drops the losing branches: a cancel-unsafe loop (`read_exact` inside `select!`) losing bytes vs the safe version (persistent buffer / framed stream)
- `CancellationToken` + `TaskTracker`: signal, stop accepting, drain in-flight, wait with a deadline
- Backpressure: bounded `mpsc` and `send().await` vs `try_send` with an explicit `Busy` error

**Build.** `labs/async-lab/`: a `tx-gateway` binary: accepts lines over TCP, pushes into a bounded queue (capacity from config), a worker batches txs every N ms or M items (`select!` on `interval` tick, `recv`, and `token.cancelled()`), and on Ctrl-C stops accepting, flushes the final batch, and exits within a deadline.

**Run.** `cd labs/async-lab && cargo run --bin tx-gateway`, then send lines with `nc localhost 7000` and press Ctrl-C.

**Acceptance criteria** (all required)
- [ ] Tests pass, including: no accepted tx is lost on shutdown; shutdown completes within the deadline
- [ ] Each `select!` branch has a comment stating whether its future is cancel-safe and why
- [ ] Queue is bounded; overload produces a typed `Busy` response, not unbounded memory
- [ ] No `tokio::spawn` without the task being tracked (no orphaned tasks)

**Stretch.** Add a `biased;` `select!` and explain in the journal when fairness matters and when priority matters.

**Checkpoint questions** (answer in `journal/S8-T3.md`; the reviewer asks them again)
1. What does "cancellation safe" mean, and give one tokio method that is and one that is not.
2. Walk through your shutdown sequence: who signals whom, and what guarantees no tx is lost?
3. Why is backpressure a correctness feature for a node, not just a performance one?

## S8-T4 · Capstone: async JSON-RPC node client and mock server
*3 days (9 h)*

**Why it matters.** Every chain exposes JSON-RPC (Substrate `state_getStorage`, Solana `getBalance`, Stellar RPC `getLedgerEntries`). A production client must handle timeouts, retries with backoff, request IDs, batching, and nodes that lie, stall or disconnect. You will reuse this client in Stages 14-16.

**Read first**
- JSON-RPC 2.0 specification (whole thing, it is short)
- tokio tutorial "Framing"; serde docs: "Enum representations" (`untagged`, `tag`)
- AWS Architecture blog: "Exponential Backoff and Jitter"

**Lesson** (`scripts/run.sh S8-T4 learn`)
- Request/response correlation by `id`: a `HashMap<Id, oneshot::Sender>` owned by one connection task
- Retry policy: which errors are retryable, exponential backoff with jitter, and why non-idempotent calls (submit tx) must not blindly retry
- Testing against a mock server with injected faults: delay, drop, malformed JSON, wrong `id`

**Build.** `labs/rpc-lab/`: `rpc-client` lib (`RpcClient::connect`, `call<P: Serialize, R: DeserializeOwned>(method, params) -> Result<R, RpcError>`, `batch`, per-call timeout, `RetryPolicy`), `mock-node` binary (methods `chain_getBlockNumber`, `getBalance`, `submitTx`; configurable fault injection), and an example that polls block height and stops on Ctrl-C. ADR in `docs/adr/` on retry and idempotency.

**Run.** `cd labs/rpc-lab && cargo run --bin mock-node -- --faults 0.2` and in another terminal `cargo run --example watch-height`; `cargo test`.

**Acceptance criteria** (all required)
- [ ] Integration tests start the mock in-process and cover: success, timeout, retry then success, retry exhausted, malformed response, mismatched id, server disconnect
- [ ] Concurrent calls over one connection are correlated correctly (test with 100 parallel calls)
- [ ] `submitTx` is never retried after the request may have been sent, unless an idempotency key is used
- [ ] Graceful shutdown of both client and server with `CancellationToken`
- [ ] No blocking calls on runtime threads; no `unwrap` outside tests; ADR written

**Stretch.** Point the client at a real public devnet RPC (Solana devnet `getSlot`) and record latency percentiles.

**Checkpoint questions** (answer in `journal/S8-T4.md`; the reviewer asks them again)
1. How does your client match a response to the caller that is waiting for it, and what happens to waiters on disconnect?
2. Which RPC errors do you retry, which do you not, and why?
3. Where could a cancelled caller leave your client in an inconsistent state, and how did you prevent it?

## S8-GATE · Stage 8 gate: closed-book exam + catch-up
*1 day (3 h)*

**Exam.** Closed book, 90 min, blank crate `labs/s08-gate/`: write a hand-rolled `Yield` future that returns `Pending` once then `Ready`, and a tokio program that spawns 20 workers reading from a bounded channel, each job wrapped in a 200 ms `timeout`, collected with `JoinSet`, shut down on a `CancellationToken` without losing queued jobs. Identify the bug in 3 given snippets (blocking call, guard across `.await`, cancel-unsafe `select!`). Then answer 5 oral questions drawn from the S8 checkpoints.

**Pass criteria.** Solved without notes or AI, compiles, tests pass, all 3 snippet bugs identified with the fix, oral answers correct or corrected with a clear explanation. Remaining time: redo the weakest task's core from a blank file.
