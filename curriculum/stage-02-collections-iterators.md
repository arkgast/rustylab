# Stage 2: Collections, closures & iterators

**Phase:** I · Fluency
**Outcome:** You pick the right collection without thinking, write closures that capture exactly what they need, and turn any loop into an iterator pipeline (or back) while predicting what it prints and what it allocates.

Each task: `/study` (recall, then probes: predict, run, explain the gap), solve the katas, run them and read the output, make the gate green, write the journal, then `/review <ID>`.

## S2-T1 · Vec, HashMap, HashSet & BTreeMap
*1 day (3 h)*

**Why it matters.** Every piece of state is a collection. `HashMap` iterates in a random order that changes between runs; if that order leaks into a hash, a state root or an event list, two honest nodes disagree and consensus breaks. That is why runtimes use ordered maps.

**Probes**
- `Vec` growth, capacity and `with_capacity`; `get` vs `[i]`; `retain`, `dedup`, `sort_unstable`, `binary_search`
- The entry API: one lookup instead of `contains_key` + `insert` + `get_mut`; naive version side by side
- `HashMap` vs `BTreeMap`: run the same insertions twice and print both iteration orders; `HashSet` set algebra (`union`, `intersection`, `difference`)

**Reference** (read after the probes, or when stuck)
- The Rust Book ch. 8 (Common Collections)
- std docs: `std::collections` module overview (the "When should you use which collection?" section), `HashMap::entry`, `Entry::or_insert`, `Entry::and_modify`, `BTreeMap::range`, `Vec::retain`, `Vec::dedup`

**Build.** `tasks/src/s02/t01_collections.rs`: eight katas:
- `word_counts(text: &str) -> BTreeMap<String, u32>` (entry API)
- `group_by_len(words: &[&str]) -> BTreeMap<usize, Vec<String>>`
- `apply_deltas(balances: &mut BTreeMap<String, u64>, deltas: &[(&str, i64)]) -> Result<(), DeltaError>` (checked, all or nothing)
- `unique_in_order(xs: &[u32]) -> Vec<u32>` (first occurrence kept, uses a `HashSet`)
- `common_accounts(a: &[&str], b: &[&str]) -> BTreeSet<String>`
- `top_n(counts: &BTreeMap<String, u32>, n: usize) -> Vec<(String, u32)>` (ties broken by name)
- `range_sum(m: &BTreeMap<u64, u64>, from: u64, to: u64) -> u64` (uses `range`)
- `state_digest(m: &BTreeMap<String, u64>) -> String` (concatenates `key=value;` in key order)

**Run.** `scripts/run.sh S2-T1` prints each kata's result, plus a `HashMap` vs `BTreeMap` iteration order demo over two runs.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s02_t01.rs` pass
- [ ] `word_counts` and `group_by_len` use the entry API, never `contains_key` followed by `insert`
- [ ] `apply_deltas` leaves the map unchanged when any delta fails
- [ ] No indexing (`v[i]`, `m[&k]`) in non-test code
- [ ] The journal explains, with the printed output, why `state_digest` must not iterate a `HashMap`

**Stretch.** Write `state_digest_hashmap` over a `HashMap`, run the binary five times and record whether the digest changes.

**Checkpoint questions** (answer in `journal/S2-T1.md`; the reviewer asks them again)
1. What does the entry API save you compared with `contains_key` + `insert`, in lookups and in borrow-checker friction?
2. Why is `HashMap` iteration order nondeterministic, and why does that matter for two nodes computing the same state?
3. When would you still choose `HashMap` over `BTreeMap` in a runtime?

## S2-T2 · Closures: Fn, FnMut, FnOnce
*1 day (3 h)*

**Why it matters.** Closures are the glue of iterator pipelines, callbacks and hooks. Knowing which trait a closure implements tells you what it captures and how often it can run, which is the same reasoning you apply to who may mutate state.

**Probes**
- Capture by shared borrow, mutable borrow, or move; how the compiler picks; `move` keyword
- `Fn` ⊂ `FnMut` ⊂ `FnOnce`: which bound to accept and why the loosest bound you can call wins
- Passing closures (`impl Fn`, generic `F: Fn`) and returning them (`impl Fn`, `Box<dyn Fn>`)

**Reference** (read after the probes, or when stuck)
- The Rust Book ch. 13.1 (Closures)
- std docs: `Fn`, `FnMut`, `FnOnce`; Rust Reference "Closure expressions" (capture modes)

**Build.** `tasks/src/s02/t02_closures.rs`: seven katas:
- `apply_twice<F: Fn(i64) -> i64>(f: F, x: i64) -> i64`
- `make_adder(n: i64) -> impl Fn(i64) -> i64`
- `make_counter() -> impl FnMut() -> u32`
- `compose<A, B, C>(f: impl Fn(A) -> B, g: impl Fn(B) -> C) -> impl Fn(A) -> C`
- `call_once_with(name: String) -> impl FnOnce() -> String`
- `retry<T, E>(attempts: u32, op: impl FnMut() -> Result<T, E>) -> Result<T, E>`
- `Validator` holding `Vec<Box<dyn Fn(&Tx) -> bool>>` with `add_rule` and `check(&self, tx: &Tx) -> bool`

**Run.** `scripts/run.sh S2-T2`

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s02_t02.rs` pass
- [ ] Each generic function uses the loosest bound that compiles (`FnOnce` > `FnMut` > `Fn`), justified in the journal
- [ ] `retry` with `attempts == 0` returns without calling `op` and without panicking (pick and document the error)

**Stretch.** Rewrite `Validator` with generics instead of `Box<dyn Fn>` and note in the journal what you lose (heterogeneous rules).

**Checkpoint questions** (answer in `journal/S2-T2.md`; the reviewer asks them again)
1. What decides whether a closure is `Fn`, `FnMut` or `FnOnce`: how it is declared or what its body does?
2. Why does `make_adder` need `move`, and what would go wrong without it?
3. Why accept `FnMut` in `retry` rather than `Fn`?

## S2-T3 · Iterator adapters: loops vs pipelines
*2 days (6 h)*

**Why it matters.** Iterators are how idiomatic Rust processes data: lazy, bounds-checked away, and as fast as a hand loop. Reading and writing pipelines fluently is the single biggest jump from "writes Rust" to "reads Rust codebases".

**Probes**
- Laziness: nothing runs until a consumer (`collect`, `sum`, `for`); `iter` vs `iter_mut` vs `into_iter`
- Each adapter shown as a `for` loop and as a pipeline side by side, with printed output from both
- `collect` is driven by the target type: `Vec`, `String`, `HashMap`, `BTreeMap`, `Result<Vec<_>, E>` (short-circuits on the first `Err`)

**Reference** (read after the probes, or when stuck)
- The Rust Book ch. 13.2-13.4 (Iterators, performance)
- std docs: `Iterator` trait page (read every provided method's one-line summary), `FromIterator`, `Peekable`, `slice::windows`, `slice::chunks`
- Rust by Example: "Iterator::any", "Iterator::find", "Iterating over Results"

**Build.** `tasks/src/s02/t03_iterators.rs`: every kata has a `_loop` and an `_iter` version with the same signature (e.g. `sum_squares_loop`, `sum_squares_iter`); the tests run both.
- Day 1: `sum_squares` (map, sum), `evens` (filter, collect), `parse_all_valid(&[&str]) -> Vec<i64>` (filter_map), `product_nonzero` (filter, product), `running_total` (scan), `dot` (zip, sum), `indexed_labels` (enumerate, String), `concat_all` (chain), `flatten_words` (flat_map), `leading_positives` (take_while), `after_header(lines: &[&str]) -> Vec<&str>` (skip_while)
- Day 2: `max_rise(prices: &[u64]) -> Option<u64>` (windows), `batch_sums` (chunks), `reverse_words` (rev, join), `merge_digits` (peekable: group adjacent digits into numbers), `run_lengths(&str) -> Vec<(char, usize)>` (group-by via fold), `parse_strict(&[&str]) -> Result<Vec<i64>, ParseIntError>` (collect into `Result`), `letter_index(&str) -> HashMap<char, Vec<usize>>` (fold into a map), `richest(&[(&str, u64)]) -> Option<&str>` (max_by_key), `shortest_word` (min_by_key), `closest_to(target: i64, xs: &[i64])` (min_by with ties)
- Spaced repetition: `count_words_iter` and `largest_iter` from S1-T4 rewritten as pipelines

**Run.** `scripts/run.sh S2-T3` prints each kata's loop and iterator result next to each other.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s02_t03.rs` pass
- [ ] Every `_iter` function contains no `for`, `while` or `loop` and no `mut` binding
- [ ] `parse_strict` stops at the first bad input (prove it with a test that counts calls via `inspect`)
- [ ] No `.clone()` or `.collect()` used only to make a borrow compile
- [ ] The journal has a table: kata, loop lines, iterator lines, which one you find clearer, and why

**Stretch.** Pick three katas and compare `cargo build --release` assembly or a quick timing loop; note whether the iterator version was slower.

**Checkpoint questions** (answer in `journal/S2-T3.md`; the reviewer asks them again)
1. Why does `v.iter().map(f)` do nothing on its own, and what makes it run?
2. How does `collect::<Result<Vec<_>, _>>()` know when to stop, and why is that useful for validating a batch of transactions?
3. Name one case where a plain `for` loop is clearer than the pipeline, and say why.

## S2-T4 · Writing your own iterators
*1 day (3 h)*

**Why it matters.** Implementing `Iterator` once gives you all 70+ adapters for free. Custom iterators let a block, a storage prefix or a byte stream be consumed lazily, without materializing everything in memory.

**Probes**
- `impl Iterator for X { type Item; fn next }`: state lives in the struct
- `IntoIterator` for owned, `&` and `&mut` forms so `for x in &coll` works
- Quick iterators without a struct: `from_fn`, `successors`, `repeat_with`

**Reference** (read after the probes, or when stuck)
- The Rust Book ch. 13.2 ("Creating Our Own Iterators" via the `Iterator` trait)
- std docs: `Iterator::next`, `IntoIterator`, `std::iter::from_fn`, `std::iter::successors`, `DoubleEndedIterator`, `ExactSizeIterator`

**Build.** `tasks/src/s02/t04_custom_iterators.rs`: six katas:
- `Fibonacci` (stops at `u64` overflow with `checked_add`, no panic)
- `Countdown { from: u32 }` that also implements `DoubleEndedIterator`
- `Pairs<'a, T>` over a slice yielding consecutive non-overlapping pairs
- `Ledger` holding `Vec<Entry>` with `IntoIterator` for `Ledger`, `&Ledger` and `&mut Ledger`
- `collatz(n: u64) -> impl Iterator<Item = u64>` via `successors`
- `BlockHeights { start, step, end }` that implements `ExactSizeIterator` correctly

**Run.** `scripts/run.sh S2-T4`

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s02_t04.rs` pass
- [ ] `Fibonacci` yields every `u64` Fibonacci number and then `None`, never wrapping or panicking
- [ ] `for e in &ledger` and `for e in &mut ledger` both compile and are exercised in a test
- [ ] At least one test chains a std adapter (`take`, `filter`, `sum`) on each custom iterator

**Stretch.** Implement `size_hint` for `Pairs` and show with a test that `collect` into a `Vec` allocates once (`capacity()` equals `len()`).

**Checkpoint questions** (answer in `journal/S2-T4.md`; the reviewer asks them again)
1. Why does `Iterator` use an associated type `Item` instead of a generic parameter?
2. What is the difference between implementing `Iterator` and implementing `IntoIterator`?
3. After `next` returns `None`, what may happen on the next call, and what does `fuse` guarantee?

## S2-T5 · Capstone: transaction analytics CLI
*2 days (6 h)*

**Why it matters.** Real tooling reads a log of transactions and answers questions: who is busiest, what is the volume per block, which accounts went negative. This combines collections, closures and pipelines on data with a fixed, deterministic output.

**Probes**
- Parse a line-based format into a `Vec<Tx>`; keep parsing and analysis separate
- One pass vs many passes: when a single `fold` beats five pipelines, and when it does not
- Deterministic report output: sort keys, fixed formatting, no `HashMap` iteration in output

**Reference** (read after the probes, or when stuck)
- Re-read your S2-T1..T4 journals
- std docs: `str::lines`, `str::split`, `std::env::args`, `std::fs::read_to_string`

**Build.** `tasks/src/s02/t05_analytics.rs`: `Tx { block: u64, from: String, to: String, amount: u64 }`, `parse_txs(&str) -> Result<Vec<Tx>, ParseError>` (format: `block,from,to,amount`), and an `Analytics` set: `volume_per_block`, `top_senders(n)`, `net_flows -> BTreeMap<String, i128>`, `largest_tx`, `accounts_seen -> BTreeSet<String>`, `blocks_with_volume_over(threshold)`, `report() -> String`.

**Run.** `scripts/run.sh S2-T5` analyzes the bundled sample; `scripts/run.sh S2-T5 -- <path>` analyzes your own file.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s02_t05.rs` pass
- [ ] No `unwrap`, `expect`, `panic!` or indexing in non-test code; sums use checked or widened arithmetic
- [ ] `report()` is byte-for-byte identical across runs (test it twice in a row)
- [ ] Analysis functions are iterator pipelines; any loop left is justified in a comment
- [ ] At least 5 `#[test]`s of your own, at least 2 on malformed input

**Stretch.** Add `--since <block>` filtering, implemented as a closure passed into the analysis functions.

**Checkpoint questions** (answer in `journal/S2-T5.md`; the reviewer asks them again)
1. Which collections did you pick for each analysis and why: ordering, lookup cost, or both?
2. Why `i128` (or another approach) for `net_flows`, and what overflow did it rule out?
3. Where did you choose a loop or a single `fold` over several pipelines, and what did it buy you?

## S2-GATE · Stage 2 gate: closed-book exam + catch-up
*1 day (3 h)*

**Exam.** Closed book, 75 min, blank file `tasks/src/s02_gate.rs`: given `&[(&str, &str, u64)]` transfers `(from, to, amount)`, write `fn balances(initial: &BTreeMap<String, u64>, txs: &[(&str, &str, u64)]) -> Result<BTreeMap<String, u64>, TransferError>` (unknown account, insufficient funds, overflow; all or nothing), `fn busiest(txs) -> Option<&str>` (most transfers sent, ties by name) as an iterator pipeline, and a custom iterator `EveryNth<I>` that yields every n-th item of any iterator. Include 4 tests. Then answer 5 oral questions drawn from the S2 checkpoints.

**Pass criteria.** Solved without notes or AI, compiles, tests pass, no loops in `busiest`, oral answers correct or corrected with a clear explanation. Remaining time: redo the weakest S2-T3 kata from a blank file, both versions.
