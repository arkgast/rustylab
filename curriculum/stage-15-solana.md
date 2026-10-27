# Stage 15: Solana

**Phase:** V · Chains
**Outcome:** You write Solana programs natively and with Anchor, validate every account by hand before trusting a framework to do it, test them in-process with LiteSVM and Mollusk, and deploy an escrow to devnet with a client that drives it.

Each task: run the lesson, solve the katas, run them and read the output, make the gate green, write the journal, then `/task-review <ID>`.

All Stage 15 code lives in `labs/solana/`, outside the `tasks` crate: programs build for the SBF target with the Solana/Anza toolchain (`cargo build-sbf`) and Anchor has its own CLI. Use the crate and CLI versions current when you scaffold; `/start-task` resolves them.

## S15-T1 · Programming model: accounts, owners, rent & PDAs
*2 days (6 h)*

**Why it matters.** Solana inverts your Stage 12 runtime: programs are stateless and all state lives in accounts passed in by the caller. Every Solana exploit starts with a program trusting an account it should have checked.

**Read first**
- solana.com/docs: "Core concepts" > Accounts, Transactions, Programs, Program Derived Addresses, Cross Program Invocation, Fees (compute units, rent)
- Your S13-T1 journal (account models comparison)

**Lesson** (`labs/solana/lessons/t01_model`, run with `cargo test -p t01_model -- --nocapture`)
- `AccountInfo`: key, `owner`, `lamports`, `data`, `is_signer`, `is_writable`, `executable`; only the owner may mutate data or debit lamports
- Rent exemption: minimum balance for data length; why accounts are pre-sized
- PDAs: `find_program_address(seeds, program_id)`, the bump, why PDAs are off-curve and can only "sign" via their program

**Build.** `labs/solana/lessons/t01_model/src/lib.rs` katas against a LiteSVM instance: create and fund an account, transfer lamports via the System Program, derive a PDA and verify its bump, compute the rent-exempt minimum for 3 data sizes, and print each account's fields after every step.

**Run.** `cargo test -p t01_model -- --nocapture` in `labs/solana/`.

**Acceptance criteria** (all required)
- [ ] Output shows owner, lamports and data length for each account after each transaction
- [ ] A test proves a transfer from a non-signer account fails
- [ ] A test derives the same PDA twice and a different PDA for a different seed
- [ ] The journal maps Solana accounts to your S12-T2 storage keys and explains what moved where

**Stretch.** Measure compute units for a transfer and a PDA derivation from the transaction metadata.

**Checkpoint questions** (answer in `journal/S15-T1.md`; the reviewer asks them again)
1. Why must a transaction declare every account it touches up front, and what does that enable for the runtime (Sealevel)?
2. What is a PDA, why can no private key sign for it, and what is the bump for?
3. Who can change an account's data, and who can credit vs debit its lamports?

## S15-T2 · A native program without a framework
*3 days (9 h)*

**Why it matters.** Anchor hides the checks that matter most. Writing them by hand once is how you learn to audit Anchor code and to read the many production programs written natively or with Pinocchio.

**Read first**
- solana.com/docs: "Programs" > "Developing programs in Rust" (entrypoint, `process_instruction`)
- `anza-xyz/pinocchio` README (zero-copy, `no_std` entrypoint); `solana-program` rustdocs for `AccountInfo`, `invoke_signed`
- `borsh` docs (you used it in S11-T2)

**Lesson** (`labs/solana/lessons/t02_native`, run with `cargo test -p t02_native -- --nocapture`)
- Entrypoint, instruction data as bytes, a Borsh-decoded instruction enum (your S12-T3 `Call` enum again)
- Manual validation order: signer, writable, owner, PDA re-derivation with stored bump, discriminator / initialized flag
- CPI with `invoke` and `invoke_signed` (PDA signer seeds) to the System Program to create an account

**Build.** `labs/solana/programs/counter-native/`: a counter program using `solana-program` or Pinocchio (justify in the journal) with `Initialize` (creates a PDA account per user via CPI), `Increment`, `Close` (returns lamports to the user); a typed error enum mapped to `ProgramError::Custom`.

**Run.** `cargo build-sbf` then `cargo test` in `labs/solana/programs/counter-native/`.

**Acceptance criteria** (all required)
- [ ] Every account is checked for signer/writable/owner/address as needed, in one `validate` step before any mutation
- [ ] Counter uses checked arithmetic; overflow is a custom error
- [ ] `Close` zeroes data, moves all lamports and reassigns or marks the account closed
- [ ] Tests include: wrong owner, wrong PDA, missing signer, double initialize

**Stretch.** Port the program to the other library (Pinocchio or `solana-program`) and compare compute units and binary size.

**Checkpoint questions** (answer in `journal/S15-T2.md`; the reviewer asks them again)
1. Why must you re-derive the PDA (or compare against a stored bump) instead of trusting the address passed in?
2. What exactly does `invoke_signed` prove to the callee, and how?
3. Why is closing an account more than transferring its lamports out?

## S15-T3 · Anchor: accounts, constraints & CPI
*3 days (9 h)*

**Why it matters.** Anchor is the dominant framework. Its `#[derive(Accounts)]` constraints are your S15-T2 checks expressed declaratively; knowing which constraint replaces which check is what separates using Anchor from auditing it.

**Read first**
- anchor-lang.com/docs: "Program structure", "Account constraints", "Program Derived Address", "Cross Program Invocation"
- `coral-xyz/sealevel-attacks` (each insecure / secure / recommended triple)

**Lesson** (`labs/solana/lessons/t03_anchor`, run with `anchor test` in that directory)
- `#[program]`, `Context<T>`, `#[derive(Accounts)]`, `Account<'info, T>` (owner + discriminator check), `Signer`, `Program`
- Constraints: `init`, `payer`, `space`, `seeds` + `bump`, `has_one`, `constraint = ...`, `close = target`, `mut`; `InitSpace`
- CPI with `CpiContext::new` / `new_with_signer`; `#[error_code]` and `require!`

**Build.** `labs/solana/programs/counter-anchor/`: the S15-T2 counter rewritten in Anchor, plus a `transfer_authority` instruction using `has_one = authority`. In the journal, a table mapping each Anchor constraint to the manual S15-T2 check it replaces.

**Run.** `anchor build && anchor test` in `labs/solana/`.

**Acceptance criteria** (all required)
- [ ] No raw `AccountInfo`/`UncheckedAccount` without a `/// CHECK:` comment explaining why it is safe
- [ ] PDA seeds include the user key; bump is stored and reused, not recomputed with `find_program_address` per call
- [ ] Every test from S15-T2's failure list has an Anchor equivalent
- [ ] The constraint-to-check mapping table is complete

**Stretch.** Reproduce two sealevel-attacks (e.g. account substitution, missing signer) against a deliberately broken copy, then fix them.

**Checkpoint questions** (answer in `journal/S15-T3.md`; the reviewer asks them again)
1. What does `Account<'info, T>` verify that `AccountInfo` does not?
2. What does `has_one = authority` expand to, and what attack does it stop?
3. Why does Anchor write an 8-byte discriminator, and which bug class does it prevent?

## S15-T4 · Testing programs with LiteSVM / Mollusk
*2 days (6 h)*

**Why it matters.** A test validator is slow and flaky. In-process SVMs give you fast, deterministic tests where you can forge any account state, which is exactly what you need to test attacks.

**Read first**
- solana.com/docs: "Tools" > LiteSVM; anchor-lang.com/docs "Testing" > LiteSVM and Mollusk
- `anza-xyz/mollusk` README (`process_and_validate_instruction`, `Check`, compute-unit benches)

**Lesson** (`labs/solana/lessons/t04_testing`, run with `cargo test -p t04_testing -- --nocapture`)
- Mollusk: single instruction, explicit accounts in, checks out; ideal for validation unit tests
- LiteSVM: multi-instruction flows, clock/slot warping, `set_account` to forge state
- Compute-unit budgets: measure, assert an upper bound, catch regressions (your S6-T5 benchmarks)

**Build.** `labs/solana/programs/counter-native/tests/` and `counter-anchor/tests/`: Mollusk tests for every validation path and LiteSVM tests for full flows, including forged accounts (wrong owner, fake PDA data with a correct discriminator) and a CU ceiling assertion.

**Run.** `cargo test` in each program directory (Anchor: `cargo test` against the built `.so`, or `anchor test`).

**Acceptance criteria** (all required)
- [ ] At least one attack test per item in the S15-T5 security checklist below
- [ ] Each instruction has a compute-unit upper-bound assertion
- [ ] Tests are deterministic: no network, no test validator
- [ ] A property or table-driven test covers increment overflow boundaries

**Stretch.** Add a Trident or proptest-based fuzz target for the native program's instruction decoder.

**Checkpoint questions** (answer in `journal/S15-T4.md`; the reviewer asks them again)
1. When do you pick Mollusk over LiteSVM, and vice versa?
2. Why is forging account state in tests essential for security testing on Solana?
3. What do compute-unit assertions protect against that functional tests do not?

## S15-T5 · Capstone: an escrow program on devnet
*3 days (9 h)*

**Why it matters.** Escrow combines PDAs as authorities, token CPIs, and account closing: the canonical Solana interview and audit exercise. Shipping it to devnet with a client proves the whole path.

**Read first**
- solana.com/docs: "Tokens" (SPL Token / Token-2022, associated token accounts); anchor-lang.com "SPL Token" CPI helpers
- Security checklist sources: sealevel-attacks, Solana "Program security" docs, your S13-T2 vulnerability lab

**Lesson** (`labs/solana/lessons/t05_escrow`, run with `anchor test`)
- Escrow flow: `make` (deposit token A to a PDA-owned vault, record terms), `take` (atomically swap A for B), `refund`
- Security checklist: signer checks, owner checks, account substitution (`has_one`, seeds), arithmetic, closing accounts and revival, mint/ATA validation, PDA sharing
- Deploying: `solana config set --url devnet`, airdrop, `anchor deploy`, program ID and upgrade authority

**Build.** `labs/solana/programs/escrow/`: an Anchor escrow with `make`, `take`, `refund`; LiteSVM tests; `labs/solana/client/`: a TypeScript or Rust client that runs make/take on devnet; `labs/solana/SECURITY.md`: the checklist with how each item is enforced and tested.

**Run.** `anchor test`, then `anchor deploy --provider.cluster devnet` and the client (`cargo run -p escrow-client` or `npx tsx client/run.ts`).

**Acceptance criteria** (all required)
- [ ] Deployed to devnet; the journal lists program ID and two explorer transaction links (make, take)
- [ ] Vault and escrow state accounts are closed on `take` and `refund`; rent returns to the maker
- [ ] Every checklist item in `SECURITY.md` points at a test that would fail without the check
- [ ] Token amounts use checked arithmetic; mints are validated against escrow state

**Stretch.** Support Token-2022 mints via the `token_interface` and discuss transfer-fee extensions.

**Checkpoint questions** (answer in `journal/S15-T5.md`; the reviewer asks them again)
1. Why is the vault owned by a PDA, and what stops another escrow from draining it?
2. What happens if `take` passes a different mint's token account for the taker, and which constraint blocks it?
3. Who holds the upgrade authority of your program, and what is the trust implication for users?

## S15-GATE · Stage 15 gate: closed-book exam + catch-up
*1 day (3 h)*

**Exam.** Closed book, 90 min, `labs/solana/programs/gate/`: write an Anchor `vault` program where a user `deposit`s lamports into a per-user PDA and `withdraw`s up to their balance; full `#[derive(Accounts)]` constraints, a typed error, and 4 LiteSVM or Mollusk tests including wrong signer and over-withdraw. Then review a 40-line native program handed to you and find its 3 planted bugs (missing signer, missing owner check, unchecked arithmetic). Then 5 oral questions drawn from the S15 checkpoints.

**Pass criteria.** Solved without notes or AI, builds, tests pass, all 3 planted bugs found and explained, oral answers correct or corrected with a clear explanation. Remaining time: redo the weakest task's core from a blank file.
