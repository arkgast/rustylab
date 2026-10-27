# Stage 14: Polkadot SDK

**Phase:** V · Chains
**Outcome:** You write, test, benchmark and upgrade a FRAME pallet, run it in a local node, and explain every macro-generated piece by pointing at the runtime you built by hand in Stage 12.

Each task: `/study` (recall, then probes: predict, run, explain the gap), solve the katas, run them and read the output, make the gate green, write the journal, then `/review <ID>`.

All Stage 14 code lives in `labs/polkadot/`, its own Cargo workspace outside the `tasks` crate (FRAME pulls a large dependency tree, a `wasm32` runtime target and its own toolchain pin). Use the `polkadot-sdk` umbrella crate at the version current when you scaffold; `/study` resolves it.

## S14-T1 · FRAME pallet anatomy
*2 days (6 h)*

**Why it matters.** FRAME is your Stage 12 runtime, industrialized. If you can map every `#[pallet::*]` attribute to the code you wrote by hand, the macros stop being magic and you can debug what they generate.

**Probes** (`labs/polkadot/lessons/t01_anatomy`, run with `cargo test -p t01_anatomy -- --nocapture`)
- `#[pallet::pallet]`, `#[pallet::config]`, `#[pallet::call]`: the `Config` trait is your S12-T3 `Config`; `#[pallet::call]` is your S9-T5 `#[module]` macro generating the `Call` enum and dispatch
- Origins: `ensure_signed`, `ensure_root`, `ensure_none`; `DispatchResult` and why a dispatchable must not panic
- `cargo expand` on a 30-line pallet: find the generated `Call<T>` enum, `Pallet<T>` struct and `Dispatchable` impl

**Reference** (read after the probes, or when stuck)
- docs.polkadot.com: "Parachains" > "Customize your runtime" > "Pallet development" (make a custom pallet)
- polkadot-sdk rustdocs (`paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs`): `reference_docs::frame_runtime`, `guides::your_first_pallet`, `frame_support::pallet_macros`

**Build.** `labs/polkadot/pallets/counter/`: a `pallet-counter` with a per-account counter, dispatchables `increment(by)`, `reset()` (signed) and `force_set(who, value)` (root), plus an `Error` enum. In `journal/S14-T1.md`, a two-column table mapping each FRAME item to its Stage 12 equivalent.

**Run.** `cargo test -p pallet-counter` in `labs/polkadot/`; `cargo expand -p pallet-counter > expanded.rs` to read the generated code.

**Acceptance criteria** (all required)
- [ ] Pallet compiles for `std` and `no_std` (`cargo check --no-default-features`)
- [ ] `increment` uses checked arithmetic and returns a pallet `Error` on overflow
- [ ] `force_set` rejects a signed origin with `BadOrigin`
- [ ] The mapping table covers `Config`, `Call`, `Pallet`, `Error`, `Event`, storage and origin

**Stretch.** Add a custom origin check via a `Config` associated type `type AdminOrigin: EnsureOrigin<Self::RuntimeOrigin>`.

**Checkpoint questions** (answer in `journal/S14-T1.md`; the reviewer asks them again)
1. Why is the pallet generic over `T: Config` instead of using concrete types, and what does the runtime supply at `impl Config for Runtime`?
2. What does `#[pallet::call]` generate, and how does it compare to your S9-T5 macro?
3. Why must a dispatchable return an error rather than panic, given what a panic does inside a block?

## S14-T2 · Storage, events, errors & weights
*3 days (9 h)*

**Why it matters.** Storage is where on-chain cost lives: every read and write is weighed and paid for. Unbounded storage is a denial-of-service vector, and wrong weights let attackers fill blocks for free.

**Probes** (`labs/polkadot/lessons/t02_storage`, run with `cargo test -p t02_storage -- --nocapture`)
- Storage types are typed keys over a flat trie: compare with your S12-T2 overlay; `ValueQuery` vs `OptionQuery`; why hashers matter (iteration order and key grinding)
- `#[pallet::event]` + `deposit_event`, `#[pallet::error]`; events vs return values
- Weights: `Weight` (ref_time, proof_size), `#[pallet::weight]`, `WeightInfo` trait; benchmarking v2 measures a worst case

**Reference** (read after the probes, or when stuck)
- polkadot-sdk rustdocs: `frame_support::storage::types` (`StorageValue`, `StorageMap`, `StorageDoubleMap`, `CountedStorageMap`), `BoundedVec`, hashers (`Blake2_128Concat`, `Twox64Concat`, `Identity`)
- `reference_docs::frame_benchmarking_weight` and `frame_benchmarking::v2` (`#[benchmarks]`); docs.polkadot.com "Benchmark the custom pallet"

**Build.** `labs/polkadot/pallets/registry/`: a name registry pallet. `Names: StorageMap<AccountId, BoundedVec<u8, MaxNameLen>>`, `Owners: StorageMap<BoundedVec, AccountId>`, `Count: StorageValue<u32>`; calls `register`, `transfer`, `release` (with a reserved deposit via `fungible::MutateHold` or `ReservableCurrency`); events for each; a `benchmarking.rs` with one benchmark per call and a generated `weights.rs`.

**Run.** `cargo test -p pallet-registry --features runtime-benchmarks` in `labs/polkadot/`.

**Acceptance criteria** (all required)
- [ ] No unbounded `Vec` in storage; `MaxNameLen` comes from `Config` as a `Get<u32>` constant
- [ ] Every call emits an event on success and returns a typed error on each failure path
- [ ] Every dispatchable's weight comes from `T::WeightInfo`, not a literal
- [ ] Benchmarks cover the worst case (longest name) and their tests pass
- [ ] Hasher choice is justified in the journal for each map

**Stretch.** Generate real weights with `frame-omni-bencher` against a runtime WASM and compare them with your guesses.

**Checkpoint questions** (answer in `journal/S14-T2.md`; the reviewer asks them again)
1. Why does FRAME demand bounded types in storage, and what attack does it prevent?
2. When is `Twox64Concat` safe and when must you use `Blake2_128Concat`?
3. A dispatchable fails halfway after one write. What happens to that write, and how does it relate to your S12-T2 nested transactions?

## S14-T3 · Mock runtime & pallet testing
*2 days (6 h)*

**Why it matters.** A pallet is only as trustworthy as its tests, and FRAME tests need a whole (tiny) runtime around them. Building the mock teaches you exactly what a runtime is made of.

**Probes** (`labs/polkadot/lessons/t03_mock`, run with `cargo test -p t03_mock -- --nocapture`)
- A mock runtime: `#[derive_impl]` defaults, `impl pallet_balances::Config`, the `Runtime` enum composing pallets (your S12-T3 composition, macro-generated)
- `new_test_ext()` seeding genesis; `run_to_block` with `on_initialize`/`on_finalize` hooks
- Assertions: `assert_ok!`, `assert_noop!` (state unchanged), `System::assert_last_event`

**Reference** (read after the probes, or when stuck)
- docs.polkadot.com: "Mock your runtime" and "Pallet unit testing"
- polkadot-sdk rustdocs: `frame_support::derive_impl`, `frame_system::config_preludes::TestDefaultConfig`, `sp_io::TestExternalities`, `frame_support::runtime` / `construct_runtime!`

**Build.** `labs/polkadot/pallets/registry/src/{mock.rs,tests.rs}`: a mock with `frame_system` + `pallet_balances` + your registry; at least 12 tests, including deposits held/released and every error.

**Run.** `cargo test -p pallet-registry` in `labs/polkadot/`.

**Acceptance criteria** (all required)
- [ ] Every `Error` variant has a test using `assert_noop!`
- [ ] At least one invariant test: `Count` equals the number of `Names` entries after a random sequence of calls
- [ ] Deposit balances are checked before and after `register` and `release`
- [ ] No `unwrap` in pallet code; tests may use it

**Stretch.** Turn the invariant test into a proptest (S6-T4) that generates call sequences.

**Checkpoint questions** (answer in `journal/S14-T3.md`; the reviewer asks them again)
1. What does `assert_noop!` check that `assert_err!` does not, and why does that matter?
2. Why do tests need `TestExternalities`; what would `StorageMap::insert` do without it?
3. What does `#[derive_impl]` save you from writing, and what risk does a default config carry?

## S14-T4 · Runtime upgrades & storage migrations
*2 days (6 h)*

**Why it matters.** Polkadot chains upgrade forklessly by swapping the runtime WASM. The code changes instantly but the stored bytes do not; a missing or wrong migration bricks a live chain.

**Probes** (`labs/polkadot/lessons/t04_migrations`, run with `cargo test -p t04_migrations -- --nocapture`)
- Why migrations exist: SCALE has no schema (S11-T2); old bytes decoded as new types = garbage or failure
- `#[pallet::storage_version]`, `VersionedMigration<FROM, TO, Inner, Pallet, DbWeight>`, `pre_upgrade`/`post_upgrade` under `try-runtime`
- Single-block weight limits; when you need multi-block migrations

**Reference** (read after the probes, or when stuck)
- docs.polkadot.com: "Runtime upgrades" and "Storage migrations"
- polkadot-sdk rustdocs: `frame_support::traits::OnRuntimeUpgrade`, `UncheckedOnRuntimeUpgrade`, `migrations::VersionedMigration`, `StorageVersion`; `pallet_migrations` (multi-block migrations)
- try-runtime-cli README (`paritytech/try-runtime-cli`)

**Build.** `labs/polkadot/pallets/registry/src/migrations/v1.rs`: change `Names` value from `BoundedVec<u8>` to a `NameRecord { name, registered_at: BlockNumber }`, bump storage version 0 to 1, write the migration with `pre_upgrade`/`post_upgrade` checks, and tests that seed v0 storage and verify v1.

**Run.** `cargo test -p pallet-registry --features try-runtime` in `labs/polkadot/`.

**Acceptance criteria** (all required)
- [ ] Migration is wrapped in `VersionedMigration` and is a no-op when run twice
- [ ] `post_upgrade` checks entry count and that every record decodes
- [ ] Migration weight accounts for one read and one write per entry
- [ ] An ADR in `docs/adr/` explains single-block vs multi-block for this migration

**Stretch.** Use `try-runtime` with a snapshot (or chopsticks) of your S14-T5 node to run the migration against real state.

**Checkpoint questions** (answer in `journal/S14-T4.md`; the reviewer asks them again)
1. Why can't the runtime just "read the old format" without a migration?
2. What goes wrong if a migration runs twice, and how does `StorageVersion` prevent it?
3. Why does a migration that is too heavy for one block halt a parachain, and what is the alternative?

## S14-T5 · Capstone: your pallet on a local node
*3 days (9 h)*

**Why it matters.** Unit tests prove the logic; a running node proves integration: genesis config, runtime API, extrinsic encoding, fees and events as a wallet sees them.

**Probes** (`labs/polkadot/README.md` walkthrough, run with `polkadot-omni-node --chain ./chain_spec.json --dev`)
- Template layout: `runtime/` (WASM + native), `pallets/`, no custom node: omni-node runs any compatible runtime
- Adding a pallet: `Cargo.toml` features (`std`, `runtime-benchmarks`, `try-runtime`), `impl Config for Runtime`, adding it to the runtime composition, genesis preset
- Submitting extrinsics from Polkadot.js Apps or a `subxt` client; reading events

**Reference** (read after the probes, or when stuck)
- docs.polkadot.com: "Set up the parachain template", "Add pallets to the runtime", "Run a parachain network"
- `paritytech/polkadot-sdk-parachain-template` README; `polkadot-omni-node` and `chain-spec-builder`; Chopsticks (`AcalaNetwork/chopsticks`) for forking and dry-running

**Build.** `labs/polkadot/node/`: the parachain template (version current when you scaffold) with `pallet-registry` (v1) integrated, a genesis preset that pre-registers one name, and `labs/polkadot/client/`: a `subxt` Rust binary that registers a name, transfers it and prints the events.

**Run.** `cargo build --release -p parachain-template-runtime`, `chain-spec-builder create ... named-preset development`, `polkadot-omni-node --chain chain_spec.json --dev`, then `cargo run -p registry-client`.

**Acceptance criteria** (all required)
- [ ] Node produces blocks locally; the client's register/transfer appear as events
- [ ] Runtime builds with `--features runtime-benchmarks` and `--features try-runtime`
- [ ] A storage-version upgrade is performed on the running chain via `system.setCode` (sudo) and state survives
- [ ] `labs/polkadot/README.md` reproduces the demo from a clean clone

**Stretch.** Fork your local chain with Chopsticks and dry-run the upgrade before applying it.

**Checkpoint questions** (answer in `journal/S14-T5.md`; the reviewer asks them again)
1. Why can omni-node run your runtime without you writing node code, and what does that say about the node/runtime boundary?
2. Trace one `register` extrinsic from the client to a stored record: encoding, signature, fee, dispatch, event.
3. What does the runtime WASM blob in storage (`:code`) have to do with forkless upgrades?

## S14-GATE · Stage 14 gate: closed-book exam + catch-up
*1 day (3 h)*

**Exam.** Closed book, 90 min, blank pallet in `labs/polkadot/pallets/gate/`: write `pallet-allowance` where an owner `approve(spender, amount)` and the spender `spend_from(owner, amount)`; `StorageDoubleMap<owner, spender, Balance>`, events, errors (`InsufficientAllowance`, `Overflow`), weights from a `WeightInfo` trait (dummy impl allowed), and a mock runtime with 5 tests including 2 `assert_noop!`. Then 5 oral questions drawn from the S14 checkpoints, including one on migrations.

**Pass criteria.** Solved without notes or AI, compiles, tests pass, no panicking path in pallet code, oral answers correct or corrected with a clear explanation. Remaining time: redo the weakest task's core from a blank file.
