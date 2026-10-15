# Stage 5: Collections & iterators

**Outcome:** You choose collections by ordering, determinism and complexity, and you write iterator pipelines that allocate only when they must.

**Window:** 2026-12-08 → 2026-12-17 · tracker: [`ROADMAP.md`](../ROADMAP.md)

Each task: read first, build in `tasks/` (or the crate named), write the journal entry, then run `/task-review <ID>`.

## S5-T1 · HashMap vs BTreeMap & determinism
*2 days (6 h) · due 2026-12-09*

**Why it matters.** Iteration order can silently fork a blockchain. Know exactly which collections are deterministic.

**Read first**
- std docs: `HashMap` (RandomState), `BTreeMap`
- The Rust Book: Common Collections

**Build.** A demo that shows two runs of the same program producing different `HashMap` orders, and a `state_root` function over a `BTreeMap` that is stable. Benchmark both for 1e5 inserts and lookups.

**Acceptance criteria** (all required)
- [ ] Test proving `state_root` is order-independent of insertion order
- [ ] A one-page journal with benchmark numbers and conclusion
- [ ] A `BTreeMap::range` query used and tested
- [ ] Explain `Entry` API with an example that avoids a double lookup

**Stretch.** Implement the same with a sorted `Vec<(K,V)>` and binary search; compare.

**Checkpoint questions** (answer in `journal/S5-T1.md`; the reviewer will ask them again)
1. Why does `HashMap` use `RandomState`?
2. When is `HashMap` still fine in a runtime?
3. What is the complexity trade-off of `BTreeMap`?

## S5-T2 · Writing iterators
*2 days (6 h) · due 2026-12-11*

**Why it matters.** Iterators are zero-cost abstractions that compile down to loops. Implementing one teaches you lazy evaluation and ownership of state.

**Read first**
- The Rust Book: Iterators
- std docs: `Iterator`, `IntoIterator`, `DoubleEndedIterator`

**Build.** Your own `Chunks<'a, T>`, `Windows<'a, T>` and `Pairwise` adaptor, with `impl Iterator` return types and an `IntoIterator for &Collection`.

**Acceptance criteria** (all required)
- [ ] Behavior matches `slice::chunks`/`windows` on property-style table tests
- [ ] `size_hint` implemented and tested
- [ ] A pipeline of 4+ adaptors with no intermediate `collect`
- [ ] Implements `DoubleEndedIterator` for one type

**Stretch.** Implement `ExactSizeIterator` and `FusedIterator`.

**Checkpoint questions** (answer in `journal/S5-T2.md`; the reviewer will ask them again)
1. Why are iterator chains lazy?
2. `iter()` vs `iter_mut()` vs `into_iter()`: what ownership is involved?
3. What does `impl Iterator<Item = T>` hide from the caller?

## S5-T3 · Closures & an LRU cache
*2 days (6 h) · due 2026-12-14*

**Why it matters.** Closures capture by borrow or move and implement `Fn`, `FnMut` or `FnOnce`. Misunderstanding this is the #1 source of lifetime confusion in callbacks.

**Read first**
- The Rust Book: Closures
- Rust by Example: Closures as input parameters

**Build.** An `LruCache<K, V>` (HashMap + VecDeque or intrusive indices), plus `group_by<K, T>(items, key_fn)` and `retry<F: FnMut() -> Result<T,E>>`.

**Acceptance criteria** (all required)
- [ ] Eviction order tests
- [ ] `get` updates recency, O(1) or documented O(n)
- [ ] `group_by` returns a `BTreeMap` for determinism
- [ ] A table in the journal mapping each closure in your code to Fn / FnMut / FnOnce and why

**Stretch.** Make `LruCache` generic over a `Weigher` closure for byte-size capacity.

**Checkpoint questions** (answer in `journal/S5-T3.md`; the reviewer will ask them again)
1. When does a closure implement `FnOnce` only?
2. What does `move` change and what does it not?
3. Why can't you store a closure borrowing local data in a `'static` context?

## S5-T4 · Capstone: event log & indexer
*2 days (6 h) · due 2026-12-16*

**Why it matters.** Chains are append-only logs plus indices. Designing the index API teaches ownership of shared data and range queries.

**Read first**
- Review S5-T1 and S5-T2 journals

**Build.** An append-only `EventLog` with `append`, `iter_by_account(&AccountId)`, `range(BlockNumber..BlockNumber)`, `since(index)` returning iterators that borrow the log.

**Acceptance criteria** (all required)
- [ ] Queries return iterators, not `Vec`s
- [ ] Indices stay consistent under 10k random appends (assert against a naive scan)
- [ ] Memory: one copy of each event (justify with `size_of` math)
- [ ] Public API has doc tests

**Stretch.** Support pruning old blocks without invalidating indices.

**Checkpoint questions** (answer in `journal/S5-T4.md`; the reviewer will ask them again)
1. How do you keep the secondary index in sync?
2. What lifetime do your returned iterators carry and what does that forbid while iterating?
3. Why use `BTreeMap::range` instead of filtering?

## S5-GATE · Stage gate
*1 day (3 h) · due 2026-12-17*

**Exam.** Closed book, 60 min: implement a custom iterator adaptor and a deterministic group-by with `BTreeMap`; explain why `HashMap` iteration order must not reach consensus state.

**Pass criteria.** Exam solved without notes or AI help, compiles, tests pass, and all oral answers are correct or corrected with a clear explanation. Remaining time: fix weak spots flagged by the reviewer and re-read failed checkpoints.
