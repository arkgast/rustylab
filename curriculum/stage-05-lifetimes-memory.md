# Stage 5: Lifetimes & memory

**Phase:** II · Depth
**Outcome:** You read a borrow-checker error and know which rule it enforces, design structs that borrow instead of copy, choose the right smart pointer or cell on purpose, predict a type's size and layout, and prove what must *not* compile with `compile_fail` doctests.

Each task: run the lesson, solve the katas, run them and read the output, make the gate green, write the journal, then `/task-review <ID>`.

## S5-T1 · Lifetime annotations & compile_fail doctests
*2 days (6 h)*

**Why it matters.** A lifetime is a promise the compiler checks: "this reference never outlives the data it points to". Runtimes hand out views into storage and calldata all the time; getting the relation between those views right is what keeps a parser from reading freed memory. From this task on, the things Rust *rejects* are part of your API, and `compile_fail` doctests are how you prove them.

**Read first**
- The Rust Book ch. 10.3 (Validating References with Lifetimes)
- The Rustonomicon: "Lifetimes", "Lifetime Elision", "Subtyping and Variance" (overview only)
- rustdoc book: "Documentation tests" (`compile_fail`, `should_panic`, `ignore`, error codes)
- "Common Rust Lifetime Misconceptions" (pretzelhammer), misconceptions 1-6

**Lesson** (`scripts/run.sh S5-T1 learn`)
- The three elision rules; when you must write `'a`; `fn longest<'a>(a: &'a str, b: &'a str) -> &'a str` vs the naive "return an owned `String`" version and what each costs
- Structs holding references (`struct Parser<'a> { input: &'a str }`); `T: 'static` means "owns its data or borrows forever", not "lives for the whole program"
- ```` ```compile_fail,E0597 ```` doctests: one-line comment naming the broken rule and the fix; variance in one table (`&'a T` covariant, `&'a mut T` invariant in `T`, `fn(T)` contravariant) and why `&mut` must be invariant

**Build.** `tasks/src/s05/t01_lifetimes.rs`: `longest`, `first_sentence`, `split_pair<'a>(&'a str, char) -> Option<(&'a str, &'a str)>`, `struct Highlighter<'t> { text: &'t str }` with `fn words(&self) -> impl Iterator<Item = &'t str>`, `struct Pair<'a, 'b>` that returns only the `'a` half, plus at least 4 `compile_fail` doctests.

**Run.** `scripts/run.sh S5-T1` prints each kata; `cargo test -p tasks --doc s05` runs the doctests.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s05_t01.rs` pass
- [ ] At least 4 `compile_fail` doctests, each with an error code (E0597, E0499, E0502, E0505 or E0515) and a `// rule: ... fix: ...` comment
- [ ] `Highlighter::words` returns `&'t str`, not `&'_ str` tied to `&self` (a test proves the words outlive the highlighter)
- [ ] No `'static`, `.clone()` or `String` added just to satisfy the borrow checker
- [ ] Journal: the three elision rules in your own words, with one function from your file per rule

**Stretch.** Write a `compile_fail` doctest that shows why `&'a mut T` is invariant: try to store a short-lived `&str` through a `&mut &'static str`.

**Checkpoint questions** (answer in `journal/S5-T1.md`; the reviewer asks them again)
1. What does `T: 'static` actually guarantee, and why does `String` satisfy it?
2. Why does `Pair<'a, 'b>` need two lifetimes instead of one, and what breaks for callers if you collapse them?
3. Why is a `compile_fail` doctest without an error code weaker than one with it?

## S5-T2 · Zero-copy parsing
*2 days (6 h)*

**Why it matters.** Nodes parse blocks, transactions and RPC payloads millions of times. Copying every field into a `String` costs allocations; borrowing slices from the input buffer costs nothing. Solana's account data and SCALE decoding both live on this idea.

**Read first**
- The Rust Book ch. 8.2 (Strings), std docs: `str::split_once`, `slice::split_at_checked`, `u32::from_le_bytes`
- `nom` README (skim only, to see the `&[u8] -> (rest, value)` shape)

**Lesson** (`scripts/run.sh S5-T2 learn`)
- Owned parse (`Vec<String>`) vs borrowed parse (`Vec<&str>`) side by side, with allocation counts printed
- The cursor pattern: `fn take<'a>(input: &mut &'a [u8], n: usize) -> Result<&'a [u8], ParseError>`
- Returning views: the output lifetime is tied to the input, never to the parser

**Build.** `tasks/src/s05/t02_zero_copy.rs`: a `Cursor<'a>` over `&'a [u8]` with `u8`, `u32_le`, `bytes(n)`, `str_prefixed` (length-prefixed UTF-8); `Header<'a>` and `Record<'a>` types; `parse_frame(&[u8]) -> Result<Frame<'_>, ParseError>`; `parse_kv_lines(&str) -> Result<Vec<(&str, &str)>, ParseError>`.

**Run.** `scripts/run.sh S5-T2` parses a sample frame and a malformed one.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s05_t02.rs` pass, including truncated and invalid-UTF-8 inputs
- [ ] No allocation in `Cursor` or `parse_frame` (no `String`, `Vec`, `to_vec`, `to_owned`)
- [ ] No indexing or slicing that can panic; all bounds go through checked methods
- [ ] A `compile_fail` doctest shows a `Frame` cannot outlive its buffer

**Stretch.** Add `Frame::into_owned(self) -> OwnedFrame` and measure the difference in a quick loop of 1 M parses.

**Checkpoint questions** (answer in `journal/S5-T2.md`; the reviewer asks them again)
1. Why does `take` take `&mut &'a [u8]` and what would go wrong with `&'a mut [u8]`?
2. When is zero-copy the wrong choice?
3. How does a length-prefixed field protect a parser from a malicious input, and what checks are still on you?

## S5-T3 · Smart pointers: Box, Rc, Weak & Drop
*2 days (6 h)*

**Why it matters.** Ownership is a tree by default. When you need a heap value, recursion, shared ownership or back-pointers, you pick a pointer type, and each choice has a cost and a failure mode (Rc cycles leak).

**Read first**
- The Rust Book ch. 15.1-15.4 and 15.6 (Reference Cycles Can Leak Memory)
- std docs: `Rc::strong_count`, `Rc::downgrade`, `Weak::upgrade`, `Rc::try_unwrap`

**Lesson** (`scripts/run.sh S5-T3 learn`)
- `Box<T>` for recursive types and trait objects; `Deref` and auto-deref
- `Rc<T>` shared ownership vs cloning the data; counts printed as you go
- `Weak<T>` for parent links; a cycle that leaks vs the same tree with `Weak`; `Drop` order printed

**Build.** `tasks/src/s05/t03_smart_pointers.rs`: `enum List { Cons(i64, Box<List>), Nil }` with `sum` and `push_front`; a `Tree` of `Rc<Node>` children with `Weak` parent links (`add_child`, `parent_value`, `depth`); a `Tracked` type whose `Drop` logs into a shared log so tests can assert drop order.

**Run.** `scripts/run.sh S5-T3` prints ref counts and the drop log.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s05_t03.rs` pass
- [ ] A test asserts that dropping the root frees every node (no leak: strong counts reach zero)
- [ ] No `Rc` used where a plain owner or `&` would do; justify each `Rc` in a `//` comment
- [ ] `sum` on a 100 000-element `List` does not overflow the stack (iterative, not recursive)

**Stretch.** Implement `Drop` for `List` iteratively so dropping a very long list cannot overflow the stack.

**Checkpoint questions** (answer in `journal/S5-T3.md`; the reviewer asks them again)
1. Why does a recursive enum need `Box`, and what does the compiler need to know that it cannot otherwise?
2. How exactly does a parent/child `Rc` cycle leak, and how does `Weak` break it?
3. What does `Rc::try_unwrap` tell you, and when is it useful?

## S5-T4 · Interior mutability: Cell, RefCell, OnceCell
*1 day (3 h)*

**Why it matters.** Sometimes `&self` has to change something: a cache, a counter, lazy init. Interior mutability moves the aliasing check from compile time to runtime (or forbids references entirely). Know the price before you pay it.

**Read first**
- The Rust Book ch. 15.5 (RefCell and the Interior Mutability Pattern)
- std docs: `Cell`, `RefCell::try_borrow_mut`, `OnceCell`, `std::sync::LazyLock`

**Lesson** (`scripts/run.sh S5-T4 learn`)
- `Cell<T>` for `Copy` values: get/set, never a reference
- `RefCell<T>`: borrow flags; a double `borrow_mut` that panics at runtime vs the `try_borrow_mut` version
- `OnceCell` / `LazyLock` for compute-once values; `UnsafeCell` as the primitive under all of them (one line)

**Build.** `tasks/src/s05/t04_interior.rs`: `CallCounter` (`Cell<u64>`), `MemoFib` (`RefCell<BTreeMap<u64, u128>>` cache behind `&self`), `Config` with a `OnceCell<String>` lazily derived field, and `SharedLedger = Rc<RefCell<Ledger>>` with `transfer` that returns `Err(Busy)` instead of panicking on a re-entrant borrow.

**Run.** `scripts/run.sh S5-T4`

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s05_t04.rs` pass
- [ ] No `borrow()`/`borrow_mut()` in library paths that can be re-entered; use `try_borrow*`
- [ ] No `RefCell` guard held across a call into user code (closure or trait method)
- [ ] Checked arithmetic in `MemoFib`; overflow is a typed error

**Stretch.** Show a re-entrancy bug: a callback that calls back into `SharedLedger` while a guard is held. Fix it by narrowing the borrow scope.

**Checkpoint questions** (answer in `journal/S5-T4.md`; the reviewer asks them again)
1. Why can `Cell<T>` never hand out a `&T`, and what does that buy?
2. What runtime cost and failure mode does `RefCell` add, compared to `&mut`?
3. Why is re-entrancy the same bug class as a smart-contract reentrancy attack?

## S5-T5 · Memory layout, niches & Cow
*1 day (3 h)*

**Why it matters.** Validators keep millions of accounts in memory. Knowing a struct's size, padding and whether `Option` costs a byte turns guesses into numbers. `Cow` lets an API borrow in the common case and allocate only when it must.

**Read first**
- The Rustonomicon: "Data Layout" (repr(Rust), repr(C), exotically sized types)
- Rust Performance Book: "Type Sizes"
- std docs: `std::mem::{size_of, align_of}`, `std::borrow::Cow`, `std::num::NonZeroU64`

**Lesson** (`scripts/run.sh S5-T5 learn`)
- Size, alignment and padding; field reordering under `repr(Rust)` vs fixed order under `repr(C)`
- Niche optimization: `size_of::<Option<Box<T>>>() == size_of::<Box<T>>()`, `Option<NonZeroU64>`, `Option<&T>`
- `Cow<'a, str>`: an `escape` that clones every time vs one that returns `Cow::Borrowed` when nothing changes

**Build.** `tasks/src/s05/t05_layout.rs`: `layout_report::<T>() -> Layout { size, align }`; three struct pairs (same fields, different order or repr) whose sizes you predict in doc comments; `AccountId(NonZeroU64)` with `Option<AccountId>` as 8 bytes; `normalize(&str) -> Cow<'_, str>` and `escape_memo(&str) -> Cow<'_, str>`.

**Run.** `scripts/run.sh S5-T5` prints a size/align table next to your predictions.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s05_t05.rs` pass (they assert sizes and `Cow` variants)
- [ ] Predictions written in doc comments *before* running; misses explained in the journal
- [ ] `normalize` allocates only when the input changes (a test checks `Cow::Borrowed`)

**Stretch.** Use `#[cfg(test)] const _: () = assert!(size_of::<Account>() <= 48);` as a compile-time size budget.

**Checkpoint questions** (answer in `journal/S5-T5.md`; the reviewer asks them again)
1. Why is `Option<Box<T>>` the same size as `Box<T>`, but `Option<u64>` is not the same size as `u64`?
2. Why does `repr(Rust)` reorder fields, and when must you use `repr(C)` instead?
3. When does returning `Cow` complicate an API more than it helps?

## S5-T6 · Capstone: arena-allocated AST interpreter
*3 days (9 h)*

**Why it matters.** Trees with shared nodes and back-references written as `Rc<RefCell<Node>>` become spaghetti: runtime borrow panics, cycles, no `Send`. The arena pattern stores nodes in one `Vec` and links them by index. It is how compilers (rustc), ECS engines and many VMs do it, and it is deterministic and cheap to clone or hash.

**Read first**
- "Arenas in Rust" (Manish Goregaokar), "Flattening ASTs" (Adrian Sampson)
- Re-read your S5-T2, S5-T3, S5-T4 journals
- `docs/adr/` template; you will write one

**Lesson** (`scripts/run.sh S5-T6 learn`)
- The same expression tree as `Box<Expr>`, as `Rc<RefCell<Node>>`, and as `Vec<Node>` + `NodeId(u32)`; print size and count allocations for each
- Typed indices (newtype `NodeId`) and why a stale or foreign index must be a typed error, not a panic
- Tokenizer borrowing the source (`Token<'src>`), parser producing an arena, evaluator walking it

**Build.** `tasks/src/s05/t06_arena_interp.rs`: `tokenize(&str) -> Result<Vec<Token<'_>>, LexError>`; `Ast { nodes: Vec<Node> }` with `NodeId`; `parse(&[Token]) -> Result<(Ast, NodeId), ParseError>` for integers, `+ - * /`, parentheses, `let x = e in e` and variables; `eval(&Ast, NodeId) -> Result<i64, EvalError>` with checked arithmetic. ADR in `docs/adr/` on arena vs `Rc` tree.

**Run.** `scripts/run.sh S5-T6` evaluates sample programs; `scripts/run.sh S5-T6 -- repl` reads expressions.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s05_t06.rs` pass
- [ ] No `Rc`, `RefCell`, `Box<Node>`, `unwrap` or panicking index; node access goes through `Ast::get(NodeId) -> Result<&Node, _>`
- [ ] Overflow and division by zero are typed `EvalError`s
- [ ] Tokens borrow the source (no `String` in `Token`)
- [ ] ADR written; at least 8 own tests, 4 on error paths

**Stretch.** Add constant folding as a pass that builds a new arena, and a `NodeId` generation check that rejects indices from another `Ast`.

**Checkpoint questions** (answer in `journal/S5-T6.md`; the reviewer asks them again)
1. What does the arena give up compared to an `Rc` tree, and why is it still the better default here?
2. Where do lifetimes appear in your pipeline, and why does the `Ast` not carry one?
3. Why is evaluating this AST deterministic, and what single change would break that?

## S5-GATE · Stage 5 gate: closed-book exam + catch-up
*1 day (3 h)*

**Exam.** Closed book, 75 min, blank file `tasks/src/s05_gate.rs`: write `struct Tokens<'a>` that iterates `&'a str` words of an input (implement `Iterator`), a `fn pick<'a, 'b>(a: &'a str, _b: &'b str) -> &'a str`, two `compile_fail` doctests with error codes and rule/fix comments, and a tiny arena `Graph { nodes: Vec<Node> }` with `add_edge(NodeId, NodeId) -> Result<(), GraphError>` and `neighbors(NodeId)`. Predict and then assert `size_of` for `Option<Box<u8>>` and `Option<u32>`. Then answer 5 oral questions drawn from the S5 checkpoints.

**Pass criteria.** Solved without notes or AI, compiles, tests and doctests pass, oral answers correct or corrected with a clear explanation. Remaining time: redo the weakest task's core from a blank file.
