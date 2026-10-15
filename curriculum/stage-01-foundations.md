# Stage 1: Foundations

**Outcome:** You can predict, before compiling, whether code passes the borrow checker, and you model failure with Option/Result instead of panics.

**Window:** 2026-10-16 → 2026-10-29 · tracker: [`ROADMAP.md`](../ROADMAP.md)

Each task: read first, build in `tasks/` (or the crate named), write the journal entry, then run `/task-review <ID>`.

## S1-T1 · Ownership & borrowing lab
*2 days (6 h) · due 2026-10-17*

**Why it matters.** Every compile error you will meet this year traces back to these rules. Learn them as rules about aliasing, not as 'compiler pain'.

**Read first**
- The Rust Book: Understanding Ownership
- Rust by Example: Scoping rules

**Build.** `tasks/src/s01/t01_ownership.rs`: implement `Inventory` (add, take, peek, total) and write 6 `compile_fail` doctests, each demonstrating a distinct borrow-checker rule.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s01_t01.rs` pass
- [ ] At least 6 `compile_fail,EXXXX` doctests covering at least 5 distinct error codes (use E0382, E0499, E0502, E0505, E0506, E0597/E0515)
- [ ] Each doctest has a one-sentence comment: which rule it breaks and how to fix it
- [ ] No `.clone()` in `Inventory` methods

**Stretch.** Draw an ASCII stack/heap diagram for each failing example.

**Checkpoint questions** (answer in `journal/S1-T1.md`; the reviewer will ask them again)
1. Why does `let b = a;` move a `String` but copy an `i32`?
2. State the rule `&mut T` enforces in terms of aliasing and mutation.
3. Why can't `peek` return a reference that outlives the next `take`?

## S1-T2 · Slices, strings & UTF-8
*2 days (6 h) · due 2026-10-20*

**Why it matters.** Zero-copy views over data are the backbone of parsers and codecs. Strings are the place most people first get bitten by bytes vs chars.

**Read first**
- The Rust Book: Slices; Storing UTF-8 Encoded Text
- std docs: `str::char_indices`, `str::is_char_boundary`

**Build.** `tasks/src/s01/t02_slices.rs`: `first_word`, `longest_line`, `split_header_body`, `truncate_chars`, `count_words`, all returning borrowed `&str`.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s01_t02.rs` pass, including emoji and multi-byte cases
- [ ] The source file contains no `String`, `Vec`, `format!` or `to_string` (zero allocation)
- [ ] `truncate_chars` never panics on any input and never splits a code point

**Stretch.** Explain in the journal why `char` is not a user-perceived character (grapheme clusters).

**Checkpoint questions** (answer in `journal/S1-T2.md`; the reviewer will ask them again)
1. Why is `&s[0..1]` allowed to panic?
2. What is the difference between `s.len()` and `s.chars().count()`, and the cost of each?
3. Which lifetime does your returned `&str` carry, and why did elision accept it?

## S1-T3 · Option & Result without unwrap
*2 days (6 h) · due 2026-10-22*

**Why it matters.** Protocol code must not panic on bad input. `Result` makes failure a value you are forced to handle.

**Read first**
- The Rust Book: Error Handling; `Option` and `Result` method lists in std

**Build.** `tasks/src/s01/t03_option_result.rs`: `parse_config(&str) -> Result<Config, ConfigError>` for `key=value` lines (`host`, `port: u16`, optional `debug: bool`). Implement `Display` and `std::error::Error` for `ConfigError` by hand.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s01_t03.rs` pass
- [ ] No `unwrap`, `expect`, `panic!`, `unreachable!` or `todo!` left in the source file
- [ ] Uses `?` and at least one of `ok_or` / `map_err` / `and_then`
- [ ] Error messages include the offending line number

**Stretch.** Return all errors in the file at once instead of the first one.

**Checkpoint questions** (answer in `journal/S1-T3.md`; the reviewer will ask them again)
1. When does `?` convert the error type and what trait does it use?
2. Option vs Result: when is `None` the right way to say 'failed'?
3. Name one situation where a panic is the correct choice.

## S1-T4 · Enums & exhaustive matching
*2 days (6 h) · due 2026-10-24*

**Why it matters.** Enums + exhaustive `match` are how Rust makes state transitions explicit. Adding a variant must break the build everywhere it matters.

**Read first**
- The Rust Book: Enums and Pattern Matching; Patterns and Matching

**Build.** `tasks/src/s01/t04_patterns.rs`: an RPN evaluator `eval_rpn(&str) -> Result<i64, RpnError>` supporting `+ - * /`.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s01_t04.rs` pass
- [ ] Overflow and division by zero return errors (use `checked_*`)
- [ ] No `_ =>` wildcard arm on your own enums
- [ ] `Token` and `RpnError` are enums; evaluation is one `match` on `Token`

**Stretch.** Add a `dup` and a `swap` token without touching existing match arms' structure.

**Checkpoint questions** (answer in `journal/S1-T4.md`; the reviewer will ask them again)
1. Why is the absence of `_ =>` a safety feature when you later add a variant?
2. What does `#[non_exhaustive]` do and who is it for?
3. How is `Option<T>` itself just an enum?

## S1-T5 · Capstone: in-memory key-value store
*3 days (9 h) · due 2026-10-28*

**Why it matters.** Combine parsing, enums, ownership and errors into a small but real command interpreter, the same shape as a runtime's dispatch.

**Read first**
- Review your S1-T1..T4 journals first

**Build.** `tasks/src/s01/t05_kvstore.rs`: `Command::parse` and `Store::execute` for `SET k v`, `GET k`, `DEL k`, `INCR k` (i64 stored as text).

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s01_t05.rs` pass
- [ ] Parsing and execution are separate steps with separate error enums
- [ ] `INCR` on non-integer and on overflow returns a typed error, store unchanged
- [ ] At least 5 additional `#[test]`s of your own in the source file
- [ ] `cargo clippy -p tasks --all-targets -- -D warnings` is clean

**Stretch.** Wire it to a REPL in `tasks/src/main.rs` reading stdin.

**Checkpoint questions** (answer in `journal/S1-T5.md`; the reviewer will ask them again)
1. Where does ownership of keys and values live, and why did you pick `String` vs `&str`?
2. How does your `Command` enum compare to `Call` in `state_machine`?
3. What invariant does your store guarantee after a failed command?

## S1-GATE · Stage gate
*1 day (3 h) · due 2026-10-29*

**Exam.** Closed book, 60 min: write `fn parse_pair(&str) -> Result<(&str, i64), PairError>` for input like `"age=42"` from a blank file, with 4 tests. Then answer 5 oral questions drawn from your S1 checkpoints.

**Pass criteria.** Exam solved without notes or AI help, compiles, tests pass, and all oral answers are correct or corrected with a clear explanation. Remaining time: fix weak spots flagged by the reviewer and re-read failed checkpoints.
