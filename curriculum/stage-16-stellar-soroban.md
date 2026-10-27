# Stage 16: Stellar & Soroban

**Phase:** V · Chains
**Outcome:** You write Soroban contracts that use the right storage tier, manage TTL deliberately, enforce authorization with `require_auth`, implement the SEP-41 token interface, and deploy and invoke a contract on testnet with `stellar-cli`.

Each task: run the lesson, solve the katas, run them and read the output, make the gate green, write the journal, then `/task-review <ID>`.

All Stage 16 code lives in `labs/soroban/`, its own Cargo workspace outside the `tasks` crate: contracts build to `wasm32v1-none` (or the target current when you scaffold) with `stellar contract build`. Use the `soroban-sdk` and `stellar-cli` versions current when you scaffold; `/start-task` resolves them.

## S16-T1 · Soroban model: Env, storage tiers, TTL & auth
*2 days (6 h)*

**Why it matters.** Soroban is the closest of the three chains to your Stage 10 metered WASM sandbox: a host (`soroban-env-host`) exposes typed host functions to a `no_std` guest. Its twist is state archival: every entry has a TTL and rent, so storage design is a cost and liveness decision.

**Read first**
- developers.stellar.org: "Smart contracts" > "Getting started" (setup, hello world), "Learn" > "Fundamentals": Contract storage, State archival, Authorization, Fees and metering
- soroban-sdk rustdocs: `Env`, `contract`, `contractimpl`, `contracttype`, `storage::{Instance, Persistent, Temporary}`, `Address::require_auth`

**Lesson** (`labs/soroban/lessons/t01_model`, run with `cargo test -p t01_model -- --nocapture`)
- `#[contract]`, `#[contractimpl]`, `Env` as the handle to the host; host types (`Symbol`, `Vec`, `Map`, `Bytes`) vs Rust std; `#![no_std]` (your S10-T3)
- Storage tiers: instance (lives with the contract), persistent (archivable, restorable), temporary (deleted on expiry); `extend_ttl(threshold, extend_to)`
- `require_auth` / `require_auth_for_args` and the auth tree; `env.mock_all_auths()` and `env.auths()` in tests

**Build.** `labs/soroban/contracts/notes/`: a contract where an `Address` writes a note (persistent), a global note counter (instance) and a rate-limit marker (temporary); each write calls `require_auth` and extends TTL; a `#[contracterror]` enum.

**Run.** `cargo test -p notes` in `labs/soroban/`; `stellar contract build` to produce the WASM.

**Acceptance criteria** (all required)
- [ ] Each piece of data uses a tier whose choice is justified in the journal
- [ ] Tests assert the exact auth tree with `env.auths()`, not only `mock_all_auths`
- [ ] A test advances the ledger past a temporary entry's TTL and shows it is gone
- [ ] Counter uses checked arithmetic and returns a contract error on overflow

**Stretch.** Print the budget (`env.cost_estimate()` or the budget API current at scaffold time) for each call and compare tiers.

**Checkpoint questions** (answer in `journal/S16-T1.md`; the reviewer asks them again)
1. When does data belong in instance vs persistent vs temporary storage, and what happens to each at TTL expiry?
2. How does `require_auth` differ from Solana's `is_signer` and FRAME's `ensure_signed`?
3. Why do Soroban contracts use host types like `soroban_sdk::Vec` instead of `std::vec::Vec`?

## S16-T2 · A token contract with tests
*2 days (6 h)*

**Why it matters.** SEP-41 is the token interface every Stellar wallet and DEX speaks, and Stellar Asset Contracts implement it for classic assets. Writing one makes you handle allowances with expiry, auth on every mutating call and events, the same concerns as your S14 registry and S15 vault.

**Read first**
- Stellar SEP-41 (Soroban Token Interface) on `stellar/stellar-protocol`
- developers.stellar.org: "Tokens" (Stellar Asset Contract, token interface); soroban-sdk `token::TokenInterface`, `testutils`
- `stellar/soroban-examples`: `token`

**Lesson** (`labs/soroban/lessons/t02_token`, run with `cargo test -p t02_token -- --nocapture`)
- `TokenInterface`: `balance`, `transfer`, `approve(from, spender, amount, expiration_ledger)`, `transfer_from`, `burn`; `i128` amounts
- Events with `env.events().publish(...)` (or the `#[contractevent]` form current at scaffold time) and asserting them in tests
- Testutils: `Address::generate`, `register` a contract, ledger manipulation, cross-contract calls to the Stellar Asset Contract

**Build.** `labs/soroban/contracts/token/`: a SEP-41 token with admin `mint`, allowances with expiry stored in temporary storage, balances in persistent storage with TTL extension, and at least 12 tests.

**Run.** `cargo test -p token` in `labs/soroban/`; `stellar contract build`.

**Acceptance criteria** (all required)
- [ ] Negative amounts are rejected everywhere; arithmetic is checked
- [ ] `transfer_from` requires the spender's auth and decrements the allowance; expired allowances read as zero
- [ ] Every mutating call emits an event and a test asserts it
- [ ] Tests assert auth trees for `transfer`, `approve` and `transfer_from`

**Stretch.** Write an invariant test: total supply equals the sum of balances after a random call sequence (S6-T4 proptest).

**Checkpoint questions** (answer in `journal/S16-T2.md`; the reviewer asks them again)
1. Why does SEP-41 use `i128` and how do you defend against negative amounts?
2. Why do allowances live in temporary storage with an expiration ledger?
3. What is the Stellar Asset Contract, and why would you use it instead of deploying your own token?

## S16-T3 · Capstone: a contract on testnet
*2 days (6 h)*

**Why it matters.** Testnet deployment exercises what unit tests cannot: real fees, footprints, simulation, TTL in wall-clock ledgers, and authorization signed by real keys.

**Read first**
- developers.stellar.org: "Getting started" > "Deploy to Testnet"; `stellar-cli` reference (`keys`, `contract build|deploy|invoke|bindings`)
- `stellar/soroban-examples`: `timelock`, `atomic_swap`

**Lesson** (`labs/soroban/README.md` walkthrough, run with `stellar contract invoke --network testnet ...`)
- `stellar keys generate <name> --network testnet --fund`; `stellar contract deploy --wasm ... --alias ...`
- Simulation, footprints and resource fees; why every invoke is simulated first
- Invoking with `-- <fn> --help` (the CLI reads the interface embedded in the WASM); generating TypeScript or Rust bindings

**Build.** `labs/soroban/contracts/vesting/`: a vesting escrow: `create(funder, beneficiary, token, total, start, cliff, duration)` pulls tokens via the token client; `claim(beneficiary)` releases the vested amount (integer math only, no floats); `revoke(admin)` returns unvested funds. Tests with ledger-time advancement; a `labs/soroban/scripts/demo.sh` that deploys to testnet and runs create and claim.

**Run.** `cargo test -p vesting`, `stellar contract build`, then `labs/soroban/scripts/demo.sh`.

**Acceptance criteria** (all required)
- [ ] Deployed to testnet; the journal lists the contract ID and the transaction hashes of create and claim
- [ ] Vesting math is integer-only, rounds down in the contract's favor, and is tested at start, cliff, mid and end
- [ ] Claiming twice in the same ledger releases nothing the second time
- [ ] TTL is extended for every persistent entry touched; the journal explains the chosen thresholds

**Stretch.** Generate TypeScript bindings with `stellar contract bindings typescript` and call `claim` from a small script.

**Checkpoint questions** (answer in `journal/S16-T3.md`; the reviewer asks them again)
1. What is a transaction footprint and why must it be known before execution?
2. What happens to an unclaimed vesting entry whose TTL expires, and how is it recovered?
3. Compare authorizing `claim` here with the escrow `take` you wrote on Solana: who signs what?

## S16-GATE · Stage 16 gate: closed-book exam + catch-up
*1 day (3 h)*

**Exam.** Closed book, 60 min, `labs/soroban/contracts/gate/`: write a `piggy_bank` contract where an owner `deposit`s a SEP-41 token and can `withdraw` only after an unlock ledger timestamp; correct storage tiers, TTL extension, `require_auth`, a `#[contracterror]` enum, and 4 tests including an auth-tree assertion and an early-withdraw rejection. Then 5 oral questions drawn from the S16 checkpoints, including one comparing Soroban, Solana and FRAME authorization.

**Pass criteria.** Solved without notes or AI, compiles, tests pass, oral answers correct or corrected with a clear explanation. Remaining time: redo the weakest task's core from a blank file.
