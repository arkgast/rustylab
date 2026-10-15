# Stage 3: Lifetimes & memory

**Outcome:** You can reason about where data lives, how long, and who may touch it, and you pick between Box, Rc, Arc and indices with a reason.

**Window:** 2026-11-13 → 2026-11-26 · tracker: [`ROADMAP.md`](../ROADMAP.md)

Each task: read first, build in `tasks/` (or the crate named), write the journal entry, then run `/task-review <ID>`.

## S3-T1 · Lifetimes & zero-copy parsing
*2 days (6 h) · due 2026-11-14*

**Why it matters.** Lifetimes are the compiler's proof that no reference outlives its data. Zero-copy decoding depends on them.

**Read first**
- The Rust Book: Validating References with Lifetimes
- Rust for Rustaceans: ch. 1 Foundations (lifetimes)

**Build.** `Parser<'a>` over `&'a [u8]`: `take(n)`, `take_u32_le()`, `take_len_prefixed()`; a `Frame<'a>` struct that borrows from the input.

**Acceptance criteria** (all required)
- [ ] All tests pass incl. truncated input returning `Err` (never panicking)
- [ ] Zero allocations in the parse path
- [ ] 5 `compile_fail` doctests where a returned slice outlives its buffer
- [ ] A journal note explaining each elided lifetime in your signatures (write out the explicit form)

**Stretch.** Add a `Cursor` with `fn peek<'b>(&'b self)` and explain why the returned lifetime differs.

**Checkpoint questions** (answer in `journal/S3-T1.md`; the reviewer will ask them again)
1. Write the explicit lifetimes for `fn first<'a>(&'a self, x: &str) -> &str` and say which rule applied.
2. What does `'static` actually promise?
3. Why can't a struct hold a reference to its own field?

## S3-T2 · Memory layout & niches
*2 days (6 h) · due 2026-11-17*

**Why it matters.** Size and alignment decide cache behavior and wire formats. Niche optimization explains why `Option<Box<T>>` is free.

**Read first**
- The Rust Reference: Type layout
- Rustonomicon: Data Layout (repr(Rust), repr(C))

**Build.** A `layout_report` binary or test table printing `size_of`/`align_of` for 12 types of your choosing, incl. `repr(C)` vs default struct, `Option<Box<T>>`, `Option<NonZeroU64>`, `Option<u64>`, `[u8; 32]`, `Vec<u8>`, `&str`, `&dyn Trait`.

**Acceptance criteria** (all required)
- [ ] Every row has a test asserting its size on 64-bit targets
- [ ] One struct with fields reordered to remove padding, shown with before/after sizes
- [ ] Journal explains each surprising number

**Stretch.** Use `std::mem::offset_of!` to print field offsets.

**Checkpoint questions** (answer in `journal/S3-T2.md`; the reviewer will ask them again)
1. Why is `Option<&T>` the same size as `&T`?
2. What does `repr(C)` guarantee that default repr does not?
3. Why does a wire format need explicit layout rather than `repr(Rust)`?

## S3-T3 · Smart pointers: Box, Rc, Weak, Drop
*2 days (6 h) · due 2026-11-19*

**Why it matters.** Heap allocation and shared ownership are tools with costs. Drop order is deterministic and you can rely on it.

**Read first**
- The Rust Book: Smart Pointers
- std docs: `Rc`, `Weak`, `Drop`

**Build.** A binary tree with `Box`, then a parent-pointer tree with `Rc<RefCell<Node>>` and `Weak`. A `Noisy` type that logs on `Drop`.

**Acceptance criteria** (all required)
- [ ] A test proving a cycle leaks (using strong/weak counts) and its `Weak` fix does not
- [ ] A test asserting drop order for locals, struct fields and `Vec` elements
- [ ] Recursive `Box` tree supports insert, contains and in-order iteration

**Stretch.** Implement `Drop` for the tree iteratively to avoid stack overflow on 1M-deep lists.

**Checkpoint questions** (answer in `journal/S3-T3.md`; the reviewer will ask them again)
1. When does `Rc` leak memory and is that 'unsafe'?
2. What is the drop order of struct fields and why does it matter?
3. Why is `Rc` not `Send`?

## S3-T4 · Interior mutability
*2 days (6 h) · due 2026-11-21*

**Why it matters.** `&T` does not always mean immutable. Knowing exactly where the rules move from compile time to runtime is essential for shared state.

**Read first**
- The Rust Book: `RefCell<T>` and the Interior Mutability Pattern
- std docs: `Cell`, `RefCell`, `OnceCell`

**Build.** A memoizing `Cache<K, V>` whose `get_or_compute(&self, ..)` takes `&self`, built on `RefCell`; a call counter with `Cell`; lazy init with `OnceCell`.

**Acceptance criteria** (all required)
- [ ] A test where a double `borrow_mut` panics and the same code using `try_borrow_mut` returns an error
- [ ] `get_or_compute` never holds a `RefCell` borrow across the user closure (document why)
- [ ] Journal compares `Cell` vs `RefCell` vs `Mutex` in a table

**Stretch.** Re-enter `get_or_compute` from inside the closure and make it safe.

**Checkpoint questions** (answer in `journal/S3-T4.md`; the reviewer will ask them again)
1. Which rule does `RefCell` enforce and when?
2. Why is `Cell<T>` free of borrow tracking?
3. How does `Mutex` differ from `RefCell` at the type level?

## S3-T5 · Capstone: typed arena & AST
*3 days (9 h) · due 2026-11-25*

**Why it matters.** Indices instead of references is how serious Rust code builds graphs, ASTs and state tries without fighting the borrow checker.

**Read first**
- Rust for Rustaceans: ch. on ownership patterns
- Skim the `slotmap` and `typed-arena` crate docs

**Build.** `Arena<T>` with typed `Id<T>` handles (`PhantomData`), an expression AST stored in it, and an evaluator.

**Acceptance criteria** (all required)
- [ ] `Id<A>` and `Id<B>` are not interchangeable (compile_fail doctest)
- [ ] Out-of-range ids return `None`, never panic
- [ ] AST evaluator for `+ * let` expressions with tests
- [ ] Journal: arena + index vs `Rc<RefCell>` vs `Box` tradeoffs

**Stretch.** Add generation counters so stale ids are detected.

**Checkpoint questions** (answer in `journal/S3-T5.md`; the reviewer will ask them again)
1. Why does `Id<T>` need `PhantomData<T>`?
2. What bugs does the generation counter catch?
3. Why is this pattern friendlier to `serde` and to `Send`?

## S3-GATE · Stage gate
*1 day (3 h) · due 2026-11-26*

**Exam.** Closed book, 75 min: write a zero-copy `Parser<'a>` returning `&'a [u8]` fields, then explain why `Rc<RefCell<T>>` can leak and how `Weak` fixes it.

**Pass criteria.** Exam solved without notes or AI help, compiles, tests pass, and all oral answers are correct or corrected with a clear explanation. Remaining time: fix weak spots flagged by the reviewer and re-read failed checkpoints.
