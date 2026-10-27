# Stage 13: Execution models & smart-contract security

**Phase:** IV · Protocol
**Outcome:** You can explain how UTXO, Substrate, Solana and Soroban model state and authorization, recognize the classic on-chain vulnerability classes in Rust code, prove each with an exploit test before fixing it, fuzz a runtime for invariant violations, and write a professional audit report of your own code.

Each task: run the lesson, solve the katas, run them and read the output, make the gate green, write the journal, then `/task-review <ID>`.

## S13-T1 · Account models: UTXO, Substrate, Solana & Soroban
*1 day (3 h)*

**Why it matters.** Most chain-specific bugs come from carrying one chain's mental model into another. On Solana the program must check every account it is handed; on Substrate the runtime reads storage itself; on Soroban auth is a host call. Knowing where each model puts the trust boundary tells you what you must check.

**Read first**
- Bitcoin whitepaper sections 2 (Transactions) and 9 (Combining and Splitting Value)
- Solana docs: "Accounts", "Programs", "Transactions" (account lists, `is_signer`, `is_writable`, owner); Sealevel blog post ("Sealevel: Parallel Processing Thousands of Smart Contracts")
- Soroban docs: "Storage types" (instance, persistent, temporary, TTL) and "Authorization" (`require_auth`)
- Polkadot SDK docs: "Accounts, addresses and keys", storage in FRAME

**Lesson** (`scripts/run.sh S13-T1 learn`)
- UTXO: state is a set of unspent outputs; a tx consumes and creates; double-spend = spending a consumed input
- Accounts passed explicitly (Solana) vs storage read implicitly (Substrate, Soroban); why explicit lists enable parallelism (Sealevel)
- Where authorization lives: signature on inputs, `ensure_signed(origin)`, `is_signer` + owner check, `require_auth`

**Build.** `tasks/src/s13/t01_models.rs`: three tiny models of the same "transfer 10 from A to B": `utxo::apply(&mut UtxoSet, Tx)`, `solana_like::process(program_id, accounts: &[AccountInfo], data)` with owner and signer checks, and `soroban_like::transfer(&mut Env, from, to, amount)` with `env.require_auth(from)` and storage tiers. Plus `docs/execution-models.md`: a comparison table (state location, auth, parallelism, rent/TTL, replay protection) and one page of prose.

**Run.** `scripts/run.sh S13-T1` runs the transfer in each model and prints what each one had to check.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s13_t01.rs` pass
- [ ] Each model has a test for its double-spend / unauthorized path
- [ ] Comparison doc covers all five columns for all four models, with a source link per row

**Stretch.** Add an Ethereum-style row (global account state, `msg.sender`, nonce) to the comparison.

**Checkpoint questions** (answer in `journal/S13-T1.md`; the reviewer asks them again)
1. Why can Solana execute non-overlapping transactions in parallel, and what does it demand from the transaction in return?
2. Where does replay protection come from in UTXO, in Substrate, and in Solana?
3. Why does Soroban charge rent (TTL) for storage, and what failure mode does that create for a contract?

## S13-T2 · Vulnerability lab: exploit, then fix
*3 days (9 h)*

**Why it matters.** Auditors find bugs by knowing the classes. Writing the exploit first proves the bug is real, gives you a regression test, and trains the attacker's eye you need when reviewing your own code.

**Read first**
- Wormhole exploit write-up (Feb 2022: unchecked sysvar account let a forged signature verification pass)
- Cashio exploit write-up (Mar 2022: missing validation of collateral account chain)
- Neodyme "Solana common pitfalls" / Sealevel attacks repo (coral-xyz/sealevel-attacks)
- Polkadot SDK docs: "Safe arithmetic" and weight/unbounded-iteration guidance; The DAO post-mortem (reentrancy) summary

**Lesson** (`scripts/run.sh S13-T2 learn`)
- Red → green: an exploit test that passes against the vulnerable code (asserts the attacker profits), then flips to expecting an error after the fix
- The classes: missing signer, missing owner / account substitution, overflow, unchecked fee math, reentrancy-like callback ordering, unbounded iteration, replay
- Fix patterns: checks-effects-interactions, typed validated accounts, checked math, bounded loops with weight, nonces + domain

**Build.** `tasks/src/s13/t02_vulns/`: seven intentionally vulnerable modules (provided as stubs by `/start-task`): `missing_signer.rs`, `account_substitution.rs`, `overflow.rs`, `fee_math.rs`, `callback_order.rs`, `unbounded_iter.rs`, `replay.rs`. For each: write `exploit_*` test in the module showing the attack succeeds, commit the red state in the journal (paste the output), then fix the code so the test asserts the attack fails. Each fix gets a two-line `//` comment naming the class.

**Run.** `scripts/run.sh S13-T2` prints, per module, the exploit outcome before and after (`--vulnerable` flag keeps the original behind a feature).

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s13_t02.rs` pass against the fixed code
- [ ] Seven exploit tests exist, written before the fix; the journal shows each red output
- [ ] The `vulnerable` feature still builds and every exploit succeeds under it (`cargo test --features vulnerable`)
- [ ] Fixes are minimal and typed (e.g. `Signer<'a>` / `Owned<'a, T>` wrappers), not ad-hoc `if` checks scattered across call sites

**Stretch.** Add an eighth module: a PDA-style derived-address collision (seeds without separators) and its exploit.

**Checkpoint questions** (answer in `journal/S13-T2.md`; the reviewer asks them again)
1. Explain account substitution and how a typed "validated account" wrapper prevents it.
2. Why is reordering "transfer then update balance" to "update balance then transfer" the fix for the callback bug?
3. Unbounded iteration does not corrupt state. Why is it still a critical bug on a blockchain?

## S13-T3 · Fuzzing & invariant testing
*2 days (6 h)*

**Why it matters.** Unit tests check what you thought of. Fuzzers and invariant tests check what you did not. Every serious protocol team fuzzes its decoder and its state transition function.

**Read first**
- Rust Fuzz Book (rust-fuzz.github.io/book): cargo-fuzz, `fuzz_target!`, `arbitrary`
- `arbitrary` crate docs; proptest book: state machine testing (`proptest-state-machine`)
- Trail of Bits blog: "Fuzzing blockchain state machines" / invariant testing overview

**Lesson** (`scripts/run.sh S13-T3 learn`)
- Coverage-guided fuzzing: corpus, crashes, minimization (`cargo fuzz tmin`)
- Structured fuzzing with `#[derive(Arbitrary)]` sequences of calls
- Invariants for a runtime: total issuance conservation, no negative/overflowed balance, nonce monotonicity, root determinism

**Build.** `labs/runtime/fuzz/` with targets `decode_extrinsic` (raw bytes → decode → must not panic) and `call_sequence` (arbitrary `Vec<(Origin, RuntimeCall)>` applied to a genesis state, asserting all invariants after every call); plus `labs/runtime/tests/s13_invariants.rs`, a proptest state-machine test comparing the runtime with a simple reference model.

**Run.** `cargo +nightly fuzz run call_sequence -- -max_total_time=600` from `labs/runtime/`, and `cargo test -p runtime --test s13_invariants`.

**Acceptance criteria** (all required)
- [ ] Both fuzz targets run 10 minutes each with no crash (or every crash fixed, minimized and turned into a regression test)
- [ ] At least 4 invariants are asserted after every call
- [ ] The reference-model proptest passes 10,000 cases
- [ ] The journal lists each bug found (or argues why none were) with the minimized input

**Stretch.** Reintroduce one S13-T2 bug into the runtime on a branch and measure how long the fuzzer takes to find it.

**Checkpoint questions** (answer in `journal/S13-T3.md`; the reviewer asks them again)
1. Why does structured fuzzing reach deeper runtime states than raw-byte fuzzing?
2. What makes a good invariant, and why is "total issuance is conserved except by mint/burn" stronger than any single unit test?
3. Why must a fuzz harness itself be deterministic?

## S13-T4 · Capstone: audit report of your runtime
*2 days (6 h)*

**Why it matters.** Writing the report is the deliverable auditors are paid for: precise severity, reproducible proof, and a fix a team can merge. Auditing your own code also exposes the gap between what you meant and what you wrote.

**Read first**
- Two public audit reports for Rust chains (e.g. an SR Labs Polkadot SDK audit and an OtterSec or Neodyme Solana program audit): note structure and severity language
- Immunefi or Code4rena severity classification

**Lesson** (`scripts/run.sh S13-T4 learn`)
- Threat model first: actors, assets, trust boundaries, entry points
- Finding format: title, severity (impact × likelihood), affected code, description, PoC test, recommendation, status
- Distinguishing a bug, a design weakness and an informational note

**Build.** `docs/audits/runtime-v1.md`: scope (commit hash, files), threat model, methodology (manual review, S13-T2 checklist, S13-T3 fuzzing), at least 6 findings across at least 3 severities, each with a PoC test in `labs/runtime/tests/audit_*.rs`, the fix commit and a retest result. Fix every Critical and High.

**Run.** `cargo test -p runtime --test 'audit_*'` runs all PoCs (green = fixed and guarded).

**Acceptance criteria** (all required)
- [ ] Report contains scope, threat model, methodology, findings table, and per-finding detail
- [ ] Every finding of Medium or higher has a PoC test that failed before the fix and passes after
- [ ] All Critical and High findings are fixed; others have a documented decision
- [ ] Severity ratings use a stated rubric and are defensible in review

**Stretch.** Swap reports with a peer (or ask the reviewer to plant one bug) and audit blind.

**Checkpoint questions** (answer in `journal/S13-T4.md`; the reviewer asks them again)
1. Pick your highest-severity finding: why that severity, and what would lower it?
2. What part of your runtime did you trust without checking, and why?
3. How would your threat model change if the runtime ran inside your S10-T5 sandbox as untrusted contracts?

## S13-GATE · Stage 13 gate: closed-book exam + catch-up
*1 day (3 h)*

**Exam.** Closed book, 75 min. You get an unseen 120-line vulnerable module (`tasks/src/s13_gate.rs`, provided by the reviewer) modelling a Solana-style vault with `deposit`, `withdraw` and `set_authority`. Find at least 3 of its 4 planted bugs, write an exploit test for each, fix them, and write a 1-page finding summary with severities. Then answer 5 oral questions drawn from the S13 checkpoints, including one comparing account models.

**Pass criteria.** Without notes or AI, at least 3 bugs found with red→green exploit tests, fixes compile and tests pass, severities justified, oral answers correct or corrected with a clear explanation. Remaining time: re-run S13-T3 fuzzing on the gate module.
