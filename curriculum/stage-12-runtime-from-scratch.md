# Stage 12: A blockchain runtime from scratch

**Phase:** IV · Protocol
**Outcome:** You design and build a deterministic blockchain runtime in `labs/runtime/` from an empty crate: typed block lifecycle, transactional storage, pluggable modules with dispatch and events, nonces, fees, a mempool, a toy fork choice, and block import that recomputes and checks a state root. Every later chain (FRAME, Solana, Soroban) will look like a variation of what you built. The runtime is one crate, `labs/runtime/`, grown task by task (the old `state_machine/` is archived; do not copy from it). It is `no_std + alloc` with a `std` feature (S10-T3), uses your S11 codec, hashes and signatures, and has a binary `runtime-demo` that narrates what happens. Fixed gate tests live in `labs/runtime/tests/s12_tMM.rs`.

Each task: `/study` (recall, then probes: predict, run, explain the gap), solve the katas, run them and read the output, make the gate green, write the journal, then `/review <ID>`.

## S12-T1 · Typestate & pure state transitions
*2 days (6 h)*

**Why it matters.** A runtime is a function `(State, Block) -> Result<State, Error>`. Making that function pure and making illegal lifecycle orders unrepresentable removes whole classes of consensus bugs before a test is written.

**Probes**
- STF as a pure function; why determinism means no clocks, randomness, floats or `HashMap` iteration
- Typestate with zero-sized marker types: `Block<Open>` → `Block<Executing>` → `Block<Sealed>`
- Naive `&mut self` mutation with a `status` field vs consuming `self` transitions, side by side

**Reference** (read after the probes, or when stuck)
- Polkadot SDK docs: "State transition function", "Runtime development" overview, `Executive` (initialize, apply extrinsics, finalize)
- Cliff Biffle, "The Typestate Pattern in Rust"
- Re-read your S4-T6 and S11-T6 journals

**Build.** `labs/runtime/src/stf.rs` and `labs/runtime/src/block.rs`: `State` (balances only for now), `Tx`, `fn apply_tx(state: State, tx: &Tx) -> Result<State, TxError>` (pure, by value), `fn apply_block(state: State, txs: &[Tx]) -> Result<State, BlockError>`, and `BlockBuilder<S>` with `new(parent) -> BlockBuilder<Open>`, `start() -> BlockBuilder<Executing>`, `push(tx)`, `seal() -> Block<Sealed>`.

**Run.** `cargo run -p runtime --bin runtime-demo -- t01` prints each transition and the final state.

**Acceptance criteria** (all required)
- [ ] All tests in `labs/runtime/tests/s12_t01.rs` pass
- [ ] `seal` before `start` and `push` after `seal` do not compile (`compile_fail` doctests)
- [ ] Applying the same block to the same state twice gives identical states (test)
- [ ] Builds with `--no-default-features --target wasm32-unknown-unknown`

**Stretch.** Benchmark `apply_tx` by value vs `&mut State` for 10,000 txs and explain the result.

**Checkpoint questions** (answer in `journal/S12-T1.md`; the reviewer asks them again)
1. What makes a state transition function deterministic, and name three ways Rust code can silently break that?
2. What does typestate buy you over a runtime `status` enum, and what does it cost?
3. If a block's 5th tx fails, what should happen to txs 1-4, and how does your signature express it?

## S12-T2 · Storage, overlays & nested transactions
*3 days (9 h)*

**Why it matters.** Real runtimes never mutate the backing database during execution. They write to an overlay that can be committed or discarded per extrinsic, and nested for calls inside calls. Getting rollback wrong means a failed tx still moves money.

**Probes**
- A `Storage` trait over bytes: `get`, `set`, `remove`, `iter_prefix`
- Overlay as a stack of changesets: `start_transaction`, `commit_transaction`, `rollback_transaction`
- Typed storage on top: `StorageValue<T>` and `StorageMap<K, V>` using your S11 codec and `Blake2_128Concat` keys

**Reference** (read after the probes, or when stuck)
- Polkadot SDK docs: `sp_state_machine::OverlayedChanges`, `frame_support::storage::with_transaction`, `#[transactional]` history
- Solana docs: "Transactions" (atomicity: all instructions or none)

**Build.** `labs/runtime/src/storage/`: `trait Storage`, `MemoryBackend` (`BTreeMap<Vec<u8>, Vec<u8>>`), `Overlay<B: Storage>` with a layered changeset stack (writes and deletes as `Option<Vec<u8>>`), `with_transaction(|ov| -> Result<T, E>)` that commits on `Ok` and rolls back on `Err`, typed `StorageValue`/`StorageMap`, and `drain() -> ChangeSet` for commit to the backend. Migrate S12-T1 balances onto it.

**Run.** `cargo run -p runtime --bin runtime-demo -- t02` prints the overlay layers during a nested call that fails at depth 2.

**Acceptance criteria** (all required)
- [ ] All tests in `labs/runtime/tests/s12_t02.rs` pass
- [ ] Nested rollback discards only the inner layer; outer writes survive (test at depth 3)
- [ ] `iter_prefix` merges overlay and backend in sorted order and hides deleted keys (test)
- [ ] Proptest: a random sequence of set/remove/start/commit/rollback matches a naive clone-the-whole-map model
- [ ] Commit/rollback without an open transaction is a typed error, not a panic

**Stretch.** Add a depth limit and storage-read counting so S12-T4 can charge weight per read.

**Checkpoint questions** (answer in `journal/S12-T2.md`; the reviewer asks them again)
1. Why is "clone the map, mutate, swap on success" a valid model but a bad implementation?
2. How does a delete in an inner layer interact with a value in the backend during `get` and `iter_prefix`?
3. Why must a failing extrinsic still be included in the block (and pay fees) even though its writes are discarded?

## S12-T3 · Modules: Config trait, dispatch & events
*3 days (9 h)*

**Why it matters.** FRAME pallets, Solana programs and Soroban contracts are all modules: a typed call enum, a dispatcher, errors and events, configured by the runtime that hosts them. This is the shape you will meet in S14-S16.

**Probes**
- `trait Config { type AccountId: Ord + Encode + ...; type Balance: ...; type BlockNumber: ...; }`
- `Call` enums per module, an outer `RuntimeCall` and `RuntimeEvent`, dispatch by exhaustive `match`
- Origins: signed vs root; events deposited to storage and cleared per block

**Reference** (read after the probes, or when stuck)
- Polkadot SDK docs: "FRAME pallets", `Config` trait, `#[pallet::call]`, `Event`, `DispatchResult`
- Re-read your S4-T6 (Config trait) and S9-T5 (`#[module]` macro) journals

**Build.** `labs/runtime/src/modules/`: `system` (block number, account nonces, events), `balances` (transfer, mint by root, total issuance, existential deposit), `registry` (claim/revoke a name, proof-of-existence style); each with `Config`, `Call`, `Error`, `Event`. A `Runtime` struct implementing every `Config`, `RuntimeCall`, `dispatch(origin, call) -> DispatchResult` wrapping each call in `with_transaction`. Optionally reuse your S9-T5 macro to generate dispatch.

**Run.** `cargo run -p runtime --bin runtime-demo -- t03` runs a scripted block and prints emitted events.

**Acceptance criteria** (all required)
- [ ] All tests in `labs/runtime/tests/s12_t03.rs` pass
- [ ] Modules depend only on their `Config`, never on `Runtime` concretely; a test runtime with `u32` accounts compiles
- [ ] A failing call emits no events and writes nothing
- [ ] Total issuance invariant holds across a random proptest sequence of calls
- [ ] ADR in `docs/adr/` on how modules depend on each other (`balances` needs `system`)

**Stretch.** Add a `utility::batch_all(Vec<RuntimeCall>)` call that is atomic across inner calls.

**Checkpoint questions** (answer in `journal/S12-T3.md`; the reviewer asks them again)
1. Why associated types on `Config` instead of generic parameters on every function?
2. How does one module call another without a circular dependency?
3. Where in your design would an "origin check forgotten" bug live, and how do the types help?

## S12-T4 · Blocks, nonces, fees & a mempool
*3 days (9 h)*

**Why it matters.** Without fees and weights, any user can fill a block with expensive work for free. Without mempool rules, an attacker can evict honest transactions or stall an account. These are economic safety properties, enforced in code.

**Probes**
- Signed extrinsics from S11-T6 entering the runtime; `validate_transaction` vs `apply_extrinsic`
- Weight per call, block weight limit, fee = base + weight × multiplier + tip, charged before dispatch
- Mempool: ready vs future queues, priority ordering, replacement by same (sender, nonce) with a bump

**Reference** (read after the probes, or when stuck)
- Polkadot SDK docs: "Transaction weights and fees", `TransactionPool` validity (`provides`/`requires`, priority, longevity)
- Ethereum: replace-by-fee rules (geth `txpool` price bump); Solana docs: "Fees" (base + prioritization fee)

**Build.** `labs/runtime/src/executive.rs` and `labs/runtime/src/mempool.rs`: `Executive::validate(tx) -> Result<ValidTx, InvalidTx>`, `Executive::apply_extrinsic` (check nonce, withdraw fee, dispatch, refund unused weight, bump nonce even on dispatch failure), `Executive::build_block(mempool, weight_limit)`; `Mempool` with `submit`, `ready()` ordered by (priority, insertion), future-nonce queue promotion, replacement requiring ≥ 10% higher tip, and a size cap with lowest-priority eviction.

**Run.** `cargo run -p runtime --bin runtime-demo -- t04` submits 20 txs (gaps, replacements, overweight) and prints the built block and leftover pool.

**Acceptance criteria** (all required)
- [ ] All tests in `labs/runtime/tests/s12_t04.rs` pass
- [ ] Fees use checked/saturating arithmetic; no floats anywhere
- [ ] A tx whose dispatch fails still pays its fee and bumps the nonce (test)
- [ ] Block building never exceeds the weight limit (proptest)
- [ ] Mempool ordering is deterministic for equal priorities (test)

**Stretch.** Add mortality (`valid_until` from S11-T5) and purge expired txs on each new block.

**Checkpoint questions** (answer in `journal/S12-T4.md`; the reviewer asks them again)
1. Why is the fee charged before dispatch and not after?
2. Why require a minimum bump for replacement instead of accepting any higher tip?
3. What attack does the future-nonce queue size limit prevent?

## S12-T5 · Consensus toy & fork choice
*3 days (9 h)*

**Why it matters.** The runtime decides whether a block is valid; consensus decides which valid chain is canonical. Reorgs force you to keep state per block and revert cleanly, which is why runtimes are pure functions over immutable snapshots.

**Probes**
- Proof of authority: slot = height % authorities, header signed by the slot author
- Block tree, longest-chain fork choice with a deterministic tie-break (lowest hash)
- Reorg: find common ancestor, revert to its state, re-apply the new branch; finality as "never revert below"

**Reference** (read after the probes, or when stuck)
- Bitcoin whitepaper sections 4-5 and 11 (longest chain, probability of reorg)
- Polkadot SDK docs: Aura (round-robin PoA) and GRANDPA (finality) overviews
- Solana docs: "Fork choice" / Tower BFT overview (skim)

**Build.** `labs/runtime/src/consensus.rs`: `Header { parent_hash, number, state_root, extrinsics_root, author, slot }` signed with Ed25519, `Authorities`, `BlockTree` (`BTreeMap<Hash, Node>` with state snapshot per node), `import(header, body) -> Result<ImportOutcome, ConsensusError>` returning `NewBest`, `Fork`, or `Reorg { retracted, enacted }`, and `finalize(hash)`.

**Run.** `cargo run -p runtime --bin runtime-demo -- t05` simulates 3 authorities, a partition and a reorg, printing the tree as ASCII.

**Acceptance criteria** (all required)
- [ ] All tests in `labs/runtime/tests/s12_t05.rs` pass
- [ ] Wrong-slot author, bad signature, unknown parent and below-finalized blocks are rejected with typed errors
- [ ] After a reorg, state equals replaying the new branch from genesis (test)
- [ ] Fork choice is identical regardless of block arrival order (proptest over permutations)

**Stretch.** Prune non-finalized forks below the finalized block and assert memory is bounded.

**Checkpoint questions** (answer in `journal/S12-T5.md`; the reviewer asks them again)
1. Why does fork choice need a deterministic tie-break?
2. What does finality buy you compared with "longest chain"?
3. During a reorg, which transactions return to the mempool, and which must not?

## S12-T6 · Capstone: block import with a state root
*3 days (9 h)*

**Why it matters.** A node never trusts a block's claimed result. It re-executes and compares the state root. That single 32-byte check is what makes a blockchain verifiable and what turns non-determinism into a chain split.

**Probes**
- State root = Merkle root over sorted `(key, value)` leaves of the committed `BTreeMap`
- Extrinsics root; header commits to both; author computes, importer verifies
- Debugging a root mismatch: dump both changesets and diff

**Reference** (read after the probes, or when stuck)
- Polkadot SDK docs: "Block import pipeline", state root and storage trie overview
- Re-read your S11-T4 journal (Merkle trees)

**Build.** `labs/runtime/src/import.rs`: `state_root(&MemoryBackend) -> Hash` via your S11-T4 tree with domain-separated leaves; `author_block(parent, mempool) -> (Header, Body)`; `import_block(parent_state, header, body) -> Result<NewState, ImportError>` that re-executes, recomputes both roots and rejects mismatches; storage inclusion proof `prove_storage(key) -> Proof`. Final ADR on the runtime architecture.

**Run.** `cargo run -p runtime --bin runtime-demo -- t06`: two in-process nodes, one authors 10 blocks, the other imports them; then a tampered block is rejected and its diff printed.

**Acceptance criteria** (all required)
- [ ] All tests in `labs/runtime/tests/s12_t06.rs` pass
- [ ] Any tampering (one tx byte, state root, extrinsics root, order of txs) is rejected (one test each)
- [ ] Two independent runs produce byte-identical roots for all 10 blocks (test)
- [ ] A storage proof verifies against the header's state root
- [ ] No `unwrap`/`expect`/indexing in `labs/runtime/src`; clippy clean; ADR written

**Stretch.** Replace the flat Merkle tree with a sparse Merkle tree so proofs of absence work.

**Checkpoint questions** (answer in `journal/S12-T6.md`; the reviewer asks them again)
1. Why must importers re-execute instead of trusting the author's state root?
2. Why does `BTreeMap` matter here, and what happens to the root if one node used `HashMap` iteration?
3. What is the cost of recomputing a flat Merkle root every block, and how do real chains avoid it?

## S12-GATE · Stage 12 gate: closed-book exam + catch-up
*1 day (3 h)*

**Exam.** Closed book, 90 min, blank file `tasks/src/s12_gate.rs`: implement an `Overlay` over `BTreeMap<Vec<u8>, Vec<u8>>` with nested `start`/`commit`/`rollback`, a `transfer` call that uses it with checked arithmetic and a fee charged even on failure, and `state_root` as a domain-separated hash over sorted entries, with 5 tests (including nested rollback and root determinism). Then answer 5 oral questions drawn from the S12 checkpoints, including one on reorgs.

**Pass criteria.** Solved without notes or AI, compiles, tests pass, no panicking paths, oral answers correct or corrected with a clear explanation. Remaining time: rebuild the weakest S12 module from a blank file.
