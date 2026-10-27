# Stage 1: Rust refresh

**Phase:** I · Fluency
**Outcome:** You write small Rust programs from a blank file without looking things up: bindings, types, control flow, structs, enums, ownership and borrowing, and you predict what each program prints before running it.

Each task: run the lesson, solve the katas, run them and read the output, make the gate green, write the journal, then `/task-review <ID>`.

## S1-T1 · Cargo, bindings & control flow
*1 day (3 h)*

**Why it matters.** Fluency starts with the boring parts. Integer types and their overflow behavior are where protocol bugs start: the same `u8 + u8` panics in debug and silently wraps in release.

**Read first**
- The Rust Book ch. 1-3 (skim what you know; read 3.2 "Data Types" carefully)
- std docs: `u8::checked_add`, `wrapping_add`, `saturating_add`, `overflowing_add`

**Lesson** (`scripts/run.sh S1-T1 learn`)
- `cargo new`, `cargo run`, `cargo check`, debug vs `--release` overflow behavior
- `let`, `mut`, shadowing, `const`; scalar and compound types; `as` casts and why they are dangerous
- Expressions vs statements, `if` as an expression, `loop` with `break value`, labeled loops, `for` over ranges

**Build.** `tasks/src/s01/t01_basics.rs`: seven katas: `fizzbuzz`, `grade`, `sum_of_digits`, `is_prime`, `next_power_of_two`, `overflow_modes`, `min_max3`.

**Run.** `scripts/run.sh S1-T1` prints each kata's result; unfinished ones show ⏳.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s01_t01.rs` pass
- [ ] No `as` casts in the source file (use `From`/`u64::from` or change types)
- [ ] `next_power_of_two` uses `loop` with `break <value>`
- [ ] `grade` is a single `match` with range patterns

**Stretch.** Run `overflow_modes`' inputs with plain `+` in a debug build and in `--release`; write down what happens in the journal.

**Checkpoint questions** (answer in `journal/S1-T1.md`; the reviewer asks them again)
1. What does `200u8 + 100` do in a debug build, in a release build, and why does Rust differ between them?
2. Why is `if` an expression in Rust, and what does that require of both branches?
3. When would you shadow a binding instead of making it `mut`?

## S1-T2 · Structs, methods & enums
*1 day (3 h)*

**Why it matters.** Structs and enums are how you model a domain. An enum with data plus an exhaustive `match` is the shape of every transaction type you will write later.

**Read first**
- The Rust Book ch. 5 (Structs) and ch. 6 (Enums and Pattern Matching)

**Lesson** (`scripts/run.sh S1-T2 learn`)
- Named, tuple and unit structs; `impl` blocks; `&self` vs `&mut self` vs `self`; associated functions (`new`)
- Enums with data; `match` exhaustiveness; `if let`; `Option<T>` as a plain enum
- `#[derive(Debug, Clone, Copy, PartialEq, Eq)]` and what each derive gives you

**Build.** `tasks/src/s01/t02_structs_enums.rs`: `Rect` (constructors, `area`, `can_hold`), `Direction` (`turn_right`, `opposite`), `Position::step`, `Coin` (`value_in_cents`, `from_cents`) and `Command::describe`.

**Run.** `scripts/run.sh S1-T2`

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s01_t02.rs` pass
- [ ] No `_ =>` wildcard arm when matching on your own enums
- [ ] `area` cannot overflow (think about the return type)

**Stretch.** Add a `Command::Teleport { x, y }` variant and let the compiler show you every place you must update.

**Checkpoint questions** (answer in `journal/S1-T2.md`; the reviewer asks them again)
1. When does a method take `self`, `&self`, or `&mut self`? Give one example of each from your code.
2. Why is a missing `match` arm a compile error instead of a runtime one, and why is that valuable?
3. Why is `Option<T>` better than a null value or a sentinel like `-1`?

## S1-T3 · Ownership, moves & Copy
*1 day (3 h)*

**Why it matters.** Ownership is Rust's core idea: every value has exactly one owner, and the owner decides when it is freed. Once you can see moves, the borrow checker stops being a wall.

**Read first**
- The Rust Book ch. 4.1 (What is Ownership?)
- std docs: `std::mem::take`, `std::mem::replace`, `Vec::append`

**Lesson** (`scripts/run.sh S1-T3 learn`)
- Stack vs heap; move semantics; `Copy` vs `Clone`; `Drop` and scope end
- Passing ownership into and out of functions
- `std::mem::take` / `replace`: moving out of a `&mut` without cloning

**Build.** `tasks/src/s01/t03_ownership.rs`: `shout`, `longer`, `Wallet` (`new`, `deposit`, `into_owner`, `take_largest`, `drain_into`), `swap_owners`.

**Run.** `scripts/run.sh S1-T3`

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s01_t03.rs` pass
- [ ] No `.clone()`, `.to_owned()`, `.to_string()` or `String::from` anywhere in the file: every value moves
- [ ] The journal shows, for `drain_into`, who owns each coin before and after

**Stretch.** Add a `Drop` impl to a local type in the lesson file and predict the drop order of three values before running it.

**Checkpoint questions** (answer in `journal/S1-T3.md`; the reviewer asks them again)
1. Why does `let b = a;` move a `String` but copy an `i32`?
2. What does `std::mem::take` do, and why does it need `Default`?
3. When a function takes `String` instead of `&str`, what is it telling the caller?

## S1-T4 · Borrowing, slices & strings
*2 days (6 h)*

**Why it matters.** Borrowing lets code read and change data without owning it. Slices are zero-copy views, the backbone of parsers and codecs. Strings are where bytes vs chars first bites you.

**Read first**
- The Rust Book ch. 4.2-4.3 (References and Borrowing, The Slice Type), ch. 8.2 (Strings)
- std docs: `str::char_indices`, `str::split_once`, `str::lines`, `slice::swap`

**Lesson** (`scripts/run.sh S1-T4 learn`)
- `&T` vs `&mut T`: many readers XOR one writer; non-lexical lifetimes
- `&[T]` and `&str` as views into owned data; why `&s[0..1]` can panic on UTF-8
- Returning borrowed data: lifetime elision in one sentence

**Build.** `tasks/src/s01/t04_borrowing.rs`: slices (`largest`, `sum_evens`, `rotate_left`, `dedup_sorted_len`) and strings (`first_word`, `longest_line`, `split_header_body`, `truncate_chars`, `count_words`, `capitalize_words`).

**Run.** `scripts/run.sh S1-T4`

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s01_t04.rs` pass, including emoji and multi-byte cases
- [ ] No `.clone()` in the file; functions returning `&str` never allocate
- [ ] `truncate_chars` never panics on any input and never splits a code point
- [ ] `rotate_left` works in place (no new `Vec`)

**Stretch.** Re-implement `count_words` and `largest` without any loop (peek at Stage 2: `split_whitespace().count()`, `iter().max()`).

**Checkpoint questions** (answer in `journal/S1-T4.md`; the reviewer asks them again)
1. State the rule `&mut T` enforces in terms of aliasing and mutation, and the bug class it prevents.
2. What is the difference between `s.len()` and `s.chars().count()`, and the cost of each?
3. Your `first_word` returns `&str`. Whose data does it point to, and why did the compiler accept it without a lifetime annotation?

## S1-T5 · Capstone: ledger CLI
*2 days (6 h)*

**Why it matters.** Parsing, enums, ownership and errors combined into a small command interpreter. This is the exact shape of a blockchain runtime: decode a call, validate it, apply a state transition, or reject it with state unchanged.

**Read first**
- Re-read your S1-T1..T4 journals
- The Rust Book ch. 9.2 (Recoverable Errors with `Result`), just enough to use `Result` and `?`

**Lesson** (`scripts/run.sh S1-T5 learn`)
- Parse, then execute: why the two steps get separate error types
- `Result<T, E>` with an enum error, `?` for early return, `match` on results
- "Validate everything, then mutate": keeping state unchanged on failure

**Build.** `tasks/src/s01/t05_ledger.rs`: `Command::parse(&str) -> Result<Command, ParseError>` and `Ledger::execute(&mut self, Command) -> Result<Outcome, LedgerError>` for `open`, `deposit`, `withdraw`, `transfer`, `balance`.

**Run.** `scripts/run.sh S1-T5` runs a scripted session; `scripts/run.sh S1-T5 -- repl` reads commands from your keyboard.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s01_t05.rs` pass
- [ ] No `unwrap`, `expect`, `panic!` or indexing (`v[i]`) in non-test code
- [ ] Balances use checked arithmetic; overflow is a typed error
- [ ] Any failed command leaves the ledger exactly as it was
- [ ] At least 5 `#[test]`s of your own in the source file, at least 3 on failure paths

**Stretch.** Add a `history` command that lists every successful command, newest first.

**Checkpoint questions** (answer in `journal/S1-T5.md`; the reviewer asks them again)
1. Why do parsing and execution have separate error enums?
2. Walk through a failed `transfer`: which checks run, in what order, and why does no balance change?
3. Where do the account names live, who owns them, and why `String` and not `&str` inside `Ledger`?

## S1-GATE · Stage 1 gate: closed-book exam + catch-up
*1 day (3 h)*

**Exam.** Closed book, 60 min, blank file `tasks/src/s01_gate.rs`: write `fn parse_pair(input: &str) -> Result<(&str, i64), PairError>` for input like `"age=42"` (trimmed key, rejects empty key, missing `=`, bad number), with 4 tests, and a `struct Counter` with `incr(&mut self, key: &str)` and `get(&self, key: &str) -> u32` backed by a `Vec`. Then answer 5 oral questions drawn from the S1 checkpoints.

**Pass criteria.** Solved without notes or AI, compiles, tests pass, oral answers correct or corrected with a clear explanation. Remaining time: redo the weakest kata from a blank file.
