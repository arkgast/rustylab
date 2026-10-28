# Stage 4: Traits & generics

**Phase:** I · Fluency
**Outcome:** You design small trait-based APIs: you choose between generics and trait objects for a reason, use associated types and std conversion traits idiomatically, wrap primitives in newtypes so the compiler catches unit mix-ups, and write a Substrate-style `Config` trait that a runtime is generic over.

Each task: `/study` (recall, then probes: predict, run, explain the gap), solve the katas, run them and read the output, make the gate green, write the journal, then `/review <ID>`.

## S4-T1 · Traits, default methods & std derives
*1 day (3 h)*

**Why it matters.** Traits are Rust's unit of abstraction. The std derives are contracts, not decorations: if `Ord` or `Hash` disagrees with `Eq`, a `BTreeMap` or `HashSet` silently misbehaves, and in a runtime that means corrupted state.

**Probes**
- Defining a trait, required vs default methods, implementing it for your types and for std types
- What each derive generates; derived `Ord` compares fields in declaration order (field order is API)
- Manual `PartialEq`/`Ord`/`Hash` that must agree: a broken impl next to a correct one, with a `BTreeSet` printout showing the damage

*Previews:* `other: &impl Shape` and `fn hash<H: Hasher>(..)` use generic syntax taught in S4-T2; read them as "any type that implements the trait".

**Reference** (read after the probes, or when stuck)
- The Rust Book ch. 10.2 (Traits), Appendix C (Derivable Traits)
- std docs: `PartialEq`/`Eq`, `PartialOrd`/`Ord` (read the "consistency" requirements), `Hash` (the `k1 == k2 -> hash(k1) == hash(k2)` rule), `Default`, `Debug`, `Display`

**Build.** `tasks/src/s04/t01_traits.rs`: seven katas:
- `trait Describe { fn name(&self) -> String; fn describe(&self) -> String { /* default uses name */ } }` for `Account` and `Validator`, overriding the default in one
- `trait Shape { fn area(&self) -> u64; fn perimeter(&self) -> u64; fn is_larger(&self, other: &impl Shape) -> bool }`
- `Priority` enum deriving `PartialOrd, Ord` with variants ordered so `Low < Normal < High`
- `PendingTx { fee: u64, nonce: u64, sender: String }` with a manual `Ord` (higher fee first, then lower nonce, then sender) consistent with a manual `PartialEq`
- `CaseInsensitive(String)` with manual `PartialEq`, `Eq` and `Hash` that agree (usable as a `HashSet` key)
- `impl Display for Account` and `impl Default for Config`
- `sorted_pool(txs: Vec<PendingTx>) -> Vec<PendingTx>` using your `Ord`

**Run.** `scripts/run.sh S4-T1`

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s04_t01.rs` pass
- [ ] `PendingTx`: `a == b` exactly when `a.cmp(&b) == Ordering::Equal` (a test checks this over many pairs)
- [ ] `CaseInsensitive("Alice")` and `CaseInsensitive("ALICE")` hash equally and dedupe in a `HashSet`
- [ ] Every derive in the file is needed by some test or call site; the journal says why each one is there

**Stretch.** Implement `PartialOrd` for `PendingTx` by hand inconsistently with `Ord`, run `sort` and a `BTreeSet` insert, and record what breaks.

**Checkpoint questions** (answer in `journal/S4-T1.md`; the reviewer asks them again)
1. Why must `Hash` agree with `Eq`, and what goes wrong in a `HashMap` if it does not?
2. Why can `f64` implement `PartialOrd` but not `Ord`, and what does that mean for state code?
3. When should a method be a default method on the trait versus a free function?

## S4-T2 · Generics, bounds & monomorphization
*1 day (3 h)*

**Why it matters.** Generics let one function serve many types at zero runtime cost: the compiler stamps out a copy per concrete type. Bounds are the contract that makes this safe. FRAME pallets are generic over a `Config`, so this is daily work later.

**Probes**
- `fn f<T: Bound>`, `impl Trait` in argument position, `where` clauses: same thing, different readability
- Monomorphization: one copy per type; `cargo build` then `nm`/`cargo asm` (optional) to see two instances
- Generic structs and `impl<T: Bound> Struct<T>`; conditional methods only for some `T`; blanket impls

**Reference** (read after the probes, or when stuck)
- The Rust Book ch. 10.1 (Generic Data Types), ch. 10.2 ("Traits as Parameters", "Trait Bound Syntax", `where` clauses)
- std docs: `std::cmp::Ord`, `std::ops::Add`, `std::iter::Sum`; Rust Reference "Trait and lifetime bounds"

**Build.** `tasks/src/s04/t02_generics.rs`: seven katas:
- `largest<T: PartialOrd + Copy>(xs: &[T]) -> Option<T>`
- `sum_checked<T: CheckedAdd + Default + Copy>(xs: &[T]) -> Option<T>` with your own `trait CheckedAdd` implemented for `u32`, `u64`, `u128`
- `Pair<T>` with `new` for all `T` and `largest(&self) -> &T` only when `T: PartialOrd`
- `Stack<T>` with `push`, `pop`, `peek`, `len`, `is_empty`, and `Default`
- `dedup_sorted<T: Ord + Clone>(xs: &[T]) -> Vec<T>`
- `trait Summary` with a blanket `impl<T: Display> Summary for T`
- `count_where<T, F: Fn(&T) -> bool>(xs: &[T], pred: F) -> usize`

**Run.** `scripts/run.sh S4-T2`

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s04_t02.rs` pass
- [ ] Each function uses the smallest set of bounds that compiles (no extra `Clone`, `Debug`, `'static`)
- [ ] `sum_checked` returns `None` on overflow for every implemented integer type
- [ ] The journal explains, for one function, what code the compiler generates for two different `T`

**Stretch.** Make `largest` work for non-`Copy` types by returning `Option<&T>`, and compare the bounds.

**Checkpoint questions** (answer in `journal/S4-T2.md`; the reviewer asks them again)
1. What is monomorphization, and what are its costs in compile time and binary size?
2. Why does `largest` need `PartialOrd`, and why did `Copy` (or returning `&T`) become necessary?
3. What does a blanket impl let you do, and what conflict can it cause later?

## S4-T3 · Trait objects & dynamic dispatch
*1 day (3 h)*

**Why it matters.** Sometimes you need a heterogeneous list: different transaction handlers, different hook implementations, chosen at runtime. `dyn Trait` gives you that through a vtable. Knowing the cost and the dyn-compatibility rules tells you when to reach for it.

**Probes**
- `&dyn Trait` and `Box<dyn Trait>`: fat pointers (data + vtable); print `size_of` for `&T` vs `&dyn Trait`
- Static vs dynamic dispatch side by side: `fn run<H: Handler>(h: &H)` vs `fn run(h: &dyn Handler)`
- Dyn compatibility: why generic methods or `-> Self` break it, and the `where Self: Sized` escape hatch

*Previews:* `std::mem::size_of::<T>()` appears only for the fat-pointer comparison; size, alignment and layout are S5-T5.

**Reference** (read after the probes, or when stuck)
- The Rust Book ch. 18.2 (Using Trait Objects That Allow for Values of Different Types)
- Rust Reference: "Trait objects" and "Dyn compatibility" (formerly "object safety")
- std docs: `Box`, `std::any::Any` (skim)

**Build.** `tasks/src/s04/t03_trait_objects.rs`: six katas:
- `trait Handler { fn name(&self) -> &str; fn handle(&mut self, tx: &Tx) -> Result<(), HandlerError>; }` with `FeeCollector`, `NonceChecker` and `Logger` implementations
- `Pipeline { handlers: Vec<Box<dyn Handler>> }` with `add` and `run(&mut self, tx: &Tx) -> Result<(), HandlerError>` (stops at first error, reports which handler failed)
- `total_area(shapes: &[Box<dyn Shape>]) -> u64` reusing the `Shape` idea from S4-T1
- `trait Plugin` with a generic helper method kept dyn-compatible via `where Self: Sized`
- `make_handler(kind: &str) -> Option<Box<dyn Handler>>` (a small factory)
- `describe_all(items: &[&dyn Describe]) -> Vec<String>`

**Run.** `scripts/run.sh S4-T3` prints the pipeline run and the `size_of` comparison.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s04_t03.rs` pass
- [ ] `Pipeline` never panics; an error names the failing handler
- [ ] The journal states, for `Pipeline` and for `total_area`, why `dyn` was chosen over generics
- [ ] No `Any` downcasting is used to get around the trait design

**Stretch.** Write `Pipeline` generically over a tuple of two handlers and compare ergonomics and dispatch cost in the journal.

**Checkpoint questions** (answer in `journal/S4-T3.md`; the reviewer asks them again)
1. What is in a `&dyn Trait` pointer, and what does a method call through it cost?
2. Why can't a trait with a generic method be used as `dyn Trait`, and how does `where Self: Sized` help?
3. Give one design reason to prefer generics and one to prefer `dyn` in a runtime.

## S4-T4 · Associated types, operators & conversions
*2 days (6 h)*

**Why it matters.** Associated types make traits like `Iterator` and FRAME's `Config` readable. Operator traits and std conversions (`From`, `TryFrom`, `FromStr`, `AsRef`) are how your types plug into the language: `?`, `parse`, `+`, `into()` all work through them.

**Probes**
- Associated type vs generic parameter: one impl per type (`Iterator::Item`) vs many (`From<T>`); the rule of thumb
- Operator overloading with `Add<Output = ...>`; when `+` should not exist (overflowing money)
- `From` (infallible) vs `TryFrom` (fallible) vs `FromStr` (from text); implement `From` and get `Into` free; `AsRef<str>` to accept many string-like inputs

**Reference** (read after the probes, or when stuck)
- The Rust Book ch. 20.2 (Advanced Traits: associated types, default type parameters, operator overloading)
- std docs: `std::ops` (`Add`, `Sub`, `AddAssign`, `Neg`, `Index`), `From`/`Into`, `TryFrom`/`TryInto`, `FromStr`, `AsRef`, `Borrow` (skim)
- Rust API Guidelines: "Conversions" (C-CONV-TRAITS, C-CONV)

**Build.** `tasks/src/s04/t04_assoc_ops_conversions.rs`:
- Day 1: `trait Container { type Item; fn get(&self, i: usize) -> Option<&Self::Item>; fn first(&self) -> Option<&Self::Item>; fn len(&self) -> usize; }` for `MyVec<T>` and `Bytes`; `trait Storage { type Key: Ord; type Value; ... }` with an in-memory `BTreeMap` impl; `Vec2 { x: i64, y: i64 }` with `Add`, `Sub`, `Neg`, `AddAssign` and `Mul<i64>`
- Day 2: `Hex([u8; 4])` with `FromStr` and `Display` round-trip; `Percent(u8)` with `TryFrom<u32>` (rejects > 100); `Address` with `From<[u8; 32]>`, `TryFrom<&[u8]>` and `AsRef<[u8]>`; `greet<S: AsRef<str>>(name: S) -> String`; `Rgb` with `FromStr` for `"#rrggbb"` returning a typed error

**Run.** `scripts/run.sh S4-T4`

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s04_t04.rs` pass
- [ ] Every fallible conversion uses `TryFrom` or `FromStr` with a typed error; no `From` impl can panic
- [ ] `"…".parse::<Hex>()` and `Hex::to_string()` round-trip for every tested value
- [ ] `Vec2` operators are implemented on values and the journal explains whether `&Vec2 + &Vec2` is needed
- [ ] The journal states, for `Container` and `Storage`, why an associated type beats a generic parameter

**Stretch.** Implement `Index<usize>` for `MyVec<T>` and write in the journal why it must panic on out-of-bounds and why that is why runtime code prefers `get`.

**Checkpoint questions** (answer in `journal/S4-T4.md`; the reviewer asks them again)
1. When do you choose an associated type over a generic type parameter? Use `Iterator` and `From` as examples.
2. Why implement `From` and not `Into`, and what does the blanket impl give you?
3. Why might a money type deliberately not implement `Add`?

## S4-T5 · Newtypes & fixed-point money
*1 day (3 h)*

**Why it matters.** `u64` balances, `u64` nonces and `u64` block numbers are interchangeable to the compiler; newtypes make mixing them a type error. Floats are nondeterministic across platforms in edge cases and lose precision, so chains use fixed-point integers with checked math.

**Probes**
- Newtypes: zero cost, new trait impls, hiding invariants behind a constructor
- Why `0.1 + 0.2 != 0.3`, printed; fixed-point as an integer scaled by `10^18`
- Rounding direction is a policy: round down for what the user receives, up for what they pay

**Reference** (read after the probes, or when stuck)
- The Rust Book ch. 20.3 ("Using the Newtype Pattern"), Rust API Guidelines C-NEWTYPE
- std docs: `u128::checked_mul`, `checked_div`, `checked_sub`, `saturating_sub`
- Skim: `sp_arithmetic::FixedU128` docs (how Substrate represents 18-decimal fixed point)

**Build.** `tasks/src/s04/t05_newtypes_fixed.rs`:
- `AccountId(u32)`, `Nonce(u64)`, `BlockNumber(u64)` with only the operations that make sense (`Nonce::next`, `BlockNumber::checked_add`)
- `Balance(u128)` with `checked_add`, `checked_sub`, `saturating_sub`, `zero`, `Display` (no `Add` impl)
- `Fixed18(u128)` with `const ONE`, `from_integer`, `from_rational(n, d) -> Option<Self>`, `checked_add`, `checked_mul`, `checked_div`, `floor`, `Display` printing `"1.500000000000000000"`
- `Balance::apply_rate(self, rate: Fixed18) -> Option<Balance>` (rounds down)
- `split_evenly(total: Balance, n: u32) -> Option<(Balance, Balance)>` returning `(share, remainder)`

**Run.** `scripts/run.sh S4-T5`

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s04_t05.rs` pass
- [ ] No `f32`/`f64` anywhere in the file, and no arithmetic that can overflow or panic (including divide by zero)
- [ ] `Nonce`, `BlockNumber` and `Balance` have no conversions between each other; inner fields are private behind constructors
- [ ] `split_evenly`: `share * n + remainder == total` for every tested input

**Stretch.** Add `checked_pow(self, exp: u32)` to `Fixed18` by repeated squaring and test it against hand-computed values.

**Checkpoint questions** (answer in `journal/S4-T5.md`; the reviewer asks them again)
1. What does a newtype cost at runtime, and what does it buy you at compile time?
2. Why are floats unacceptable in state-transition code, beyond precision?
3. In `apply_rate`, which way do you round and who would exploit the other choice?

## S4-T6 · Capstone: pluggable runtime Config trait
*2 days (6 h)*

**Why it matters.** Substrate's `frame_system::Config` makes a whole runtime generic over its account, balance and block-number types. Building a miniature version shows how associated types, bounds, newtypes and checked math combine into an API where swapping a type is one line.

**Probes**
- `trait Config { type AccountId: Ord + Clone + Debug; type Balance: ...; type BlockNumber: ...; }`: bounds as the contract
- `struct System<T: Config>` and `struct Balances<T: Config>`: `T::AccountId` everywhere, no concrete types
- Supertraits and helper traits (`trait Zero`, `trait CheckedMath`) instead of depending on external crates

**Reference** (read after the probes, or when stuck)
- Re-read your S4-T1..T5 journals
- Polkadot SDK docs: `frame_system::Config` (skim the associated types list), "Pallet coupling" overview
- Optional, after you finish: the archived mini runtime on branch `archive/legacy-projects` (`git show archive/legacy-projects:state_machine/src/system.rs`), to compare with your design

**Build.** `tasks/src/s04/t06_config_runtime.rs`:
- `trait Config` with `AccountId`, `Balance`, `BlockNumber` (and a `const EXISTENTIAL_DEPOSIT`)
- Helper traits `Zero`, `One`, `CheckedAdd`, `CheckedSub` implemented for the integer types you use
- `System<T>`: `block_number`, `inc_block_number -> Result<(), RuntimeError>`, `nonce(&who)`, `inc_nonce`
- `Balances<T>`: `set_balance`, `balance`, `transfer(from, to, amount) -> Result<(), RuntimeError>` with checked math and the existential-deposit rule
- `Runtime<T>` that owns both and exposes `execute_block(block: Block<T>) -> Result<(), RuntimeError>`
- Two configs: `TestConfig` (`AccountId = String`, `Balance = u64`, `BlockNumber = u32`) and `ProdConfig` (`AccountId = AccountId([u8; 32])`-style newtype, `Balance = u128`, `BlockNumber = u64`)

**Run.** `scripts/run.sh S4-T6` runs the same scripted block on both configs and prints final state for each.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s04_t06.rs` pass for both configs
- [ ] No concrete account, balance or block-number type appears outside the two `Config` impls
- [ ] No `unwrap`, `expect`, `panic!`, indexing or floats in non-test code; failed transfers leave state unchanged
- [ ] State uses `BTreeMap`; the journal explains why
- [ ] At least 6 `#[test]`s of your own, run against both configs (a generic test helper is fine)

**Stretch.** Add a `type Hooks: OnTransfer<Self>` associated type with a no-op and a logging implementation, chosen per config.

**Checkpoint questions** (answer in `journal/S4-T6.md`; the reviewer asks them again)
1. Why does `Config` use associated types rather than `System<A, B, N>` with three generic parameters?
2. Which bounds did `Balance` need, and how did you decide each was necessary?
3. What happens at compile time when the same `Runtime` is used with `TestConfig` and `ProdConfig`?

## S4-GATE · Stage 4 gate: closed-book exam + catch-up
*1 day (3 h)*

**Exam.** Closed book, 75 min, blank file `tasks/src/s04_gate.rs`: write a newtype `Amount(u64)` with `FromStr` (rejects empty, non-numeric, zero) and `Display`; a `trait Fee { fn fee(&self, amount: Amount) -> Option<Amount>; }` with `Flat` and `Percent(u8)` implementations; a function `total_fees(rules: &[Box<dyn Fee>], amount: Amount) -> Option<Amount>` and a generic `cheapest<F: Fee>(rules: &[F], amount: Amount) -> Option<Amount>`. Checked math throughout. Include 4 tests. Then answer 5 oral questions drawn from the S4 checkpoints.

**Pass criteria.** Solved without notes or AI, compiles, tests pass, no floats or panicking arithmetic, oral answers correct or corrected with a clear explanation (including why one function is `dyn` and the other generic). Remaining time: redo the weakest S4 kata from a blank file.
