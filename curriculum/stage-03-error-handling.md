# Stage 3: Error handling

**Phase:** I · Fluency
**Outcome:** You model every failure as a typed value, chain fallible steps with combinators and `?`, design error enums that carry context and a source chain, and know exactly when a panic is acceptable and when it is a bug.

Each task: run the lesson, solve the katas, run them and read the output, make the gate green, write the journal, then `/task-review <ID>`.

## S3-T1 · Option & Result combinators
*1 day (3 h)*

**Why it matters.** Most error-handling code is plumbing: turn a missing value into an error, convert one error into another, chain steps that may fail. Combinators make that plumbing short and explicit, and a runtime that never panics is built from them.

**Read first**
- The Rust Book ch. 9 (Error Handling)
- std docs: `Option` and `Result` method lists, especially `ok_or`, `ok_or_else`, `map`, `map_err`, `and_then`, `unwrap_or`, `unwrap_or_default`, `transpose`, `ok`, `filter`, `zip`
- Rust by Example: "Error handling" (`?`, "Iterating over Results")

**Lesson** (`scripts/run.sh S3-T1 learn`)
- `match` version vs combinator version vs `?` version of the same function, printed side by side
- `?` on `Option` and on `Result`; why you cannot mix them in one function without `ok_or`
- `transpose`: `Option<Result<T, E>>` to `Result<Option<T>, E>` and when it appears (optional fields)

**Build.** `tasks/src/s03/t01_combinators.rs`: eight katas:
- `parse_port(s: &str) -> Option<u16>` (`ok`, `filter` to reject 0)
- `lookup_balance(m: &BTreeMap<String, u64>, who: &str) -> Result<u64, LookupError>` (`ok_or_else`)
- `parse_amount(s: &str) -> Result<u64, AmountError>` (`map_err` from `ParseIntError`)
- `double_then_add(a: &str, b: &str) -> Result<u64, AmountError>` (`and_then`, checked math)
- `optional_fee(field: Option<&str>) -> Result<Option<u64>, AmountError>` (`map` + `transpose`)
- `first_even_square(xs: &[&str]) -> Option<u64>` (`?` on `Option` inside a closure or helper)
- `both(a: Option<u32>, b: Option<u32>) -> Option<u64>` (`zip`, checked mul)
- `config_value(env: &BTreeMap<String, String>, key: &str) -> Result<u32, ConfigError>` (missing vs malformed are different variants)

**Run.** `scripts/run.sh S3-T1`

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s03_t01.rs` pass
- [ ] No `match` on `Option`/`Result` in non-test code where a single combinator or `?` does the job
- [ ] No `unwrap`, `expect` or `unwrap_or(0)` hiding an error the caller should see
- [ ] The journal shows one kata written three ways (`match`, combinators, `?`) and says which you keep

**Stretch.** Write `sum_all(xs: &[&str]) -> Result<u64, AmountError>` with `try_fold` and checked addition.

**Checkpoint questions** (answer in `journal/S3-T1.md`; the reviewer asks them again)
1. What is the difference between `map` and `and_then`, in terms of the closure's return type?
2. When is `unwrap_or_default()` correct, and when does it silently hide a bug?
3. Why does `transpose` exist? Describe a real input where you need it.

## S3-T2 · Custom error types, From & source chains
*1 day (3 h)*

**Why it matters.** A caller can only react to an error it can match on. Hand-writing `Display`, `Error` and `From` once shows you exactly what derive crates generate later, and source chains let a failure deep in decoding still explain itself at the top.

**Read first**
- The Rust Book ch. 9.2 ("Propagating Errors", "The `?` operator")
- std docs: `std::error::Error` (`source`), `std::fmt::Display`, `From`, `std::num::ParseIntError`
- Rust by Example: "Defining an error type", "Wrapping errors"

**Lesson** (`scripts/run.sh S3-T2 learn`)
- An error is just an enum: variants carry the data needed to act (account, line, needed vs available)
- `impl Display` for humans, `impl Error` with `source()` for the chain, `impl From<Inner>` so `?` converts
- Walking a source chain with a `while let Some(src)` loop and printing each level

**Build.** `tasks/src/s03/t02_custom_errors.rs`, with no external crates:
- `ParseTxError { MissingField(&'static str), BadAmount(ParseIntError), Empty }` with `Display`, `Error` (`source` returns the inner `ParseIntError`) and `From<ParseIntError>`
- `ExecError { UnknownAccount(String), Insufficient { needed: u64, available: u64 }, Overflow }` with `Display` and `Error`
- `TxError { Parse { line: usize, source: ParseTxError }, Exec(ExecError) }` wrapping both, with `From<ExecError>`
- `parse_tx(line: &str) -> Result<Tx, ParseTxError>` and `apply(state: &mut BTreeMap<String, u64>, tx: &Tx) -> Result<(), ExecError>`
- `process(lines: &[&str], state: &mut BTreeMap<String, u64>) -> Result<usize, TxError>` using `?` throughout
- `error_chain(e: &dyn Error) -> Vec<String>` that collects every message in the source chain

**Run.** `scripts/run.sh S3-T2` prints a few failing inputs with their full error chains.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s03_t02.rs` pass
- [ ] No `thiserror`, `anyhow` or `Box<dyn Error>` in this file
- [ ] `Display` messages are lowercase, without trailing punctuation, and do not repeat the source's message
- [ ] `process` contains no `map_err` where a `From` impl could do the conversion
- [ ] `apply` leaves state unchanged on any error

**Stretch.** Add a `line` to `ExecError` failures as well without duplicating the field in every variant (hint: wrap, don't widen).

**Checkpoint questions** (answer in `journal/S3-T2.md`; the reviewer asks them again)
1. What exactly does `?` do with the error value, step by step, and which trait makes the conversion happen?
2. Why should `Display` not include the source's message when `source()` also returns it?
3. Why carry `needed` and `available` in `Insufficient` instead of only a message string?

## S3-T3 · thiserror, anyhow & when to panic
*1 day (3 h)*

**Why it matters.** Libraries owe callers matchable, typed errors; binaries owe users a clear message and an exit code. Choosing the right tool for each side, and having a written panic policy, is what separates a robust node from one an attacker can crash with a malformed input.

**Read first**
- `thiserror` README and docs (`#[error]`, `#[from]`, `#[source]`, `#[error(transparent)]`)
- `anyhow` docs (`anyhow::Result`, `Context`, `bail!`, `ensure!`)
- The Rust Book ch. 9.3 ("To `panic!` or Not to `panic!`")

**Lesson** (`scripts/run.sh S3-T3 learn`)
- Your S3-T2 hand-written errors next to the `thiserror` version: same behavior, fewer lines
- `anyhow` in `main`: `.context("reading config")`, printing the chain with `{:#}` and `{:?}`
- Panic policy: panics are for broken invariants (bugs), never for bad input; `debug_assert!` vs `assert!`; `std::process::ExitCode`

**Build.** `tasks/src/s03/t03_thiserror_anyhow.rs` and the runner `tasks/examples/s03_t03.rs`:
- Port `ParseTxError`, `ExecError`, `TxError` from S3-T2 to `thiserror` (`#[from]`, `#[source]`)
- `load_state(text: &str) -> Result<BTreeMap<String, u64>, StateError>` (library side, typed)
- `run(args: &[String]) -> anyhow::Result<Summary>` (binary side) that adds `.context(...)` at each I/O and parse step
- `exit_code_for(err: &anyhow::Error) -> ExitCode` that downcasts to pick 2 for usage errors, 3 for invalid data, 1 otherwise
- `checked_index(xs: &[u64], i: usize) -> Result<u64, IndexError>` and a written `PANIC_POLICY` doc comment at the top of the module

**Run.** `scripts/run.sh S3-T3` runs good and bad inputs and prints each exit code.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s03_t03.rs` pass
- [ ] `anyhow` appears only in the binary-side functions, never in a library function's signature
- [ ] Each `.context(...)` message says what was being attempted, not what failed
- [ ] The module doc states the panic policy in at most 5 lines; no `unwrap`/`expect` in non-test code
- [ ] The journal shows one error printed with `{}`, `{:#}` and `{:?}` and explains the difference

**Stretch.** Return `ExitCode` from the example's `main` and verify with `echo $?` after each run.

**Checkpoint questions** (answer in `journal/S3-T3.md`; the reviewer asks them again)
1. Why should a library not return `anyhow::Error`?
2. Give one situation where `expect` is acceptable in production code and one where it is a bug.
3. What does `#[error(transparent)]` do, and when would you use it?

## S3-T4 · Capstone: transaction file validator
*2 days (6 h)*

**Why it matters.** A block importer must reject bad input without crashing and say exactly what was wrong. Collecting every error instead of stopping at the first is how good validators, linters and compilers behave, and it is what you want when someone submits a batch.

**Read first**
- Re-read your S3-T1..T3 journals
- std docs: `str::lines` with `enumerate`, `Iterator::partition`, `std::process::ExitCode`

**Lesson** (`scripts/run.sh S3-T4 learn`)
- Fail-fast (`?`, `collect::<Result<_, _>>`) vs collect-all (`partition`, a `Vec<LineError>`): when each is right
- Two phases: syntactic validation per line, then semantic validation against state (unknown accounts, balances, nonces)
- Mapping error categories to exit codes and a stable human report

**Build.** `tasks/src/s03/t04_validator.rs`: input lines `nonce,from,to,amount` (blank lines and `#` comments allowed).
- `LineError { line: usize, kind: LineErrorKind }` via `thiserror`; kinds: field count, bad number, zero amount, self-transfer, nonce gap, unknown account, insufficient funds, overflow
- `parse_line(line_no: usize, s: &str) -> Result<Option<Tx>, LineError>` (`None` for blank/comment)
- `validate(text: &str, genesis: &BTreeMap<String, u64>) -> Report` that collects **all** errors with 1-based line numbers and keeps going after a bad line
- `Report { valid: Vec<Tx>, errors: Vec<LineError>, final_state: BTreeMap<String, u64> }` with `render() -> String` and `exit_code() -> u8` (0 clean, 3 any invalid line, 2 usage)
- Runner: `scripts/run.sh S3-T4 -- <path>` uses `anyhow` for file I/O and exits nonzero on failure

**Run.** `scripts/run.sh S3-T4` validates the bundled good and bad samples; `scripts/run.sh S3-T4 -- <path>` validates your own file.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s03_t04.rs` pass
- [ ] A file with 5 independent bad lines reports all 5 with correct line numbers
- [ ] A rejected transaction never changes `final_state`; later lines see state as if it never happened
- [ ] No `unwrap`, `expect`, `panic!` or indexing in non-test code; checked arithmetic for balances
- [ ] The binary exits 0, 2 or 3 as specified; verified in the journal with `echo $?`
- [ ] At least 6 `#[test]`s of your own, at least 4 on failure paths

**Stretch.** Add a `--fail-fast` flag that stops at the first error, reusing the same `parse_line` and apply functions.

**Checkpoint questions** (answer in `journal/S3-T4.md`; the reviewer asks them again)
1. Why does the validator collect all errors, while a block importer usually stops at the first invalid transaction?
2. How did you guarantee that a failed line leaves `final_state` unchanged?
3. Where is the boundary between your typed errors and `anyhow`, and why there?

## S3-GATE · Stage 3 gate: closed-book exam + catch-up
*1 day (3 h)*

**Exam.** Closed book, 60 min, blank file `tasks/src/s03_gate.rs`: write `enum ConfigError` by hand (no crates) with `Display`, `Error` and `From<ParseIntError>`, covering missing key, bad integer (with source) and out-of-range value. Then write `fn parse_config(text: &str) -> Result<Config, ConfigError>` for `key=value` lines setting `max_block_size: u32` (1..=10_000) and `min_fee: u64`, using `?` and combinators with no `match` on `Option`. Include 4 tests, one asserting the source chain. Then answer 5 oral questions drawn from the S3 checkpoints.

**Pass criteria.** Solved without notes or AI, compiles, tests pass, no `unwrap`/`expect` outside tests, oral answers correct or corrected with a clear explanation. Remaining time: redo the weakest S3 kata from a blank file.
