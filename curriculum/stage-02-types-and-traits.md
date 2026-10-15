# Stage 2: Types & traits

**Outcome:** You model a domain with types so invalid states do not compile, and you choose between generics and trait objects deliberately.

**Window:** 2026-10-30 → 2026-11-12 · tracker: [`ROADMAP.md`](../ROADMAP.md)

Each task: read first, build in `tasks/` (or the crate named), write the journal entry, then run `/task-review <ID>`.

## S2-T1 · Newtypes & domain modeling
*2 days (6 h) · due 2026-10-31*

**Why it matters.** A `u128` can be a balance, a block number or a price. Types are the cheapest unit tests you will ever write.

**Read first**
- Rust API Guidelines: Type safety
- Rust by Example: Newtype idiom

**Build.** `AccountId([u8; 32])`, `Balance(u128)`, `BlockNumber(u64)` with `Display`, `FromStr` and checked/saturating math on `Balance`.

**Acceptance criteria** (all required)
- [ ] Round-trip test for `Display` then `FromStr` on `AccountId`
- [ ] 2 `compile_fail` doctests proving `Balance` cannot be used as `BlockNumber`
- [ ] `Balance` has a private inner field; construction only via named constructors
- [ ] `size_of::<Balance>() == 16` asserted in a test

**Stretch.** Compare with the type aliases in `state_machine/src/types.rs` and write an ADR on switching.

**Checkpoint questions** (answer in `journal/S2-T1.md`; the reviewer will ask them again)
1. Newtype vs type alias: what does each buy you?
2. Is a newtype zero-cost? How do you verify it?
3. Why should the inner field of `Balance` be private?

## S2-T2 · Traits & generic dispatch
*2 days (6 h) · due 2026-11-03*

**Why it matters.** Traits are Rust's abstraction mechanism. Knowing what the compiler generates for `T: Trait` vs `dyn Trait` is a senior-level skill.

**Read first**
- The Rust Book: Traits; Generics; Trait Objects
- Rust for Rustaceans: ch. 2 Types

**Build.** A `Storage` trait (`get/put/delete`), `MemStorage` (BTreeMap), a `LoggingStorage<S>` decorator, `Ledger<S: Storage>` and `DynLedger(Box<dyn Storage>)`.

**Acceptance criteria** (all required)
- [ ] One shared test suite runs against all four combinations
- [ ] A journal table: binary size / call overhead notes for generic vs dyn (use `cargo bloat` or `size_of`)
- [ ] A compile-fail doctest showing a non-object-safe trait used as `dyn`

**Stretch.** Add a blanket `impl<S: Storage> Storage for &mut S`.

**Checkpoint questions** (answer in `journal/S2-T2.md`; the reviewer will ask them again)
1. What is monomorphization and what does it cost?
2. What makes a trait object-safe?
3. When would you choose `dyn` despite the vtable cost?

## S2-T3 · Conversions & error plumbing
*2 days (6 h) · due 2026-11-05*

**Why it matters.** `From`/`TryFrom`/`AsRef`/`FromStr` are the glue of every Rust API. They also make `?` work.

**Read first**
- std docs: `From`, `TryFrom`, `AsRef`, `FromStr`
- The Rust Book: `?` and `From`

**Build.** Extend S2-T1 types with `From<[u8;32]>`, `TryFrom<&[u8]>`, `AsRef<[u8]>`, a hand-rolled `Hex` type, and an `AppError` that wraps three error sources via `From`.

**Acceptance criteria** (all required)
- [ ] `TryFrom<&[u8]>` rejects wrong lengths with a typed error
- [ ] `?` converts 3 different error types into `AppError` in one function
- [ ] A function taking `impl AsRef<[u8]>` accepts `Vec<u8>`, `&[u8]`, `[u8; N]` and `AccountId`
- [ ] No `unwrap` in non-test code

**Stretch.** Implement `Deref` incorrectly on purpose, then explain why `AsRef` is the better tool.

**Checkpoint questions** (answer in `journal/S2-T3.md`; the reviewer will ask them again)
1. From vs Into: which do you implement and why?
2. When is `TryFrom` the right trait instead of `From`?
3. Why does `?` need `From`?

## S2-T4 · Standard traits & fixed-point math
*2 days (6 h) · due 2026-11-07*

**Why it matters.** Consensus code cannot use floats. Fixed-point plus the right std traits is how you do money.

**Read first**
- std docs: `Ord`, `Hash`, `Default`, `Sum`, `Add`
- Why floats are non-deterministic across platforms

**Build.** `Fixed18(u128)` with 18 decimals: `Display`/`FromStr` (`"1.5"`), `Ord`, `Hash`, `Default`, `checked_add/sub/mul/div`, `Sum`.

**Acceptance criteria** (all required)
- [ ] Table-driven tests for parse/format incl. `0`, `1e-18`, max value
- [ ] `checked_mul` and `checked_div` define and document a rounding mode
- [ ] The words `f32` and `f64` appear nowhere in the source
- [ ] `impl Sum` and a test summing an empty iterator

**Stretch.** Make `Fixed<const DECIMALS: u32>` generic using const generics.

**Checkpoint questions** (answer in `journal/S2-T4.md`; the reviewer will ask them again)
1. Why are floats banned in consensus code?
2. Should `Add` panic, saturate, or be absent here? Defend your choice.
3. How do `Ord` and `Hash` have to agree with `Eq`?

## S2-T5 · Capstone: Config trait with associated types
*3 days (9 h) · due 2026-11-11*

**Why it matters.** This is the exact mechanism FRAME uses to make pallets reusable across chains.

**Read first**
- The Rust Book: Associated Types; Advanced Traits
- Skim `balances.rs` and `system.rs` in `state_machine`

**Build.** `trait Config { type AccountId; type Balance; }` and a generic `Balances<T: Config>` with `transfer`, `mint`, `balance_of`.

**Acceptance criteria** (all required)
- [ ] The same test suite instantiated for two Configs: (`String`, `u64`) and (`AccountId`, `Balance`)
- [ ] Trait bounds are the minimum required (no gratuitous `Copy`/`Debug`)
- [ ] A journal ADR: associated type vs generic parameter for `Balance`
- [ ] All arithmetic is checked; failed transfer leaves state untouched

**Stretch.** Add a `type Event` and an event log.

**Checkpoint questions** (answer in `journal/S2-T5.md`; the reviewer will ask them again)
1. Associated type vs generic parameter: when does each make sense?
2. How does your `Config` compare to the one in `state_machine`?
3. Which bounds did you have to add and why?

## S2-GATE · Stage gate
*1 day (3 h) · due 2026-11-12*

**Exam.** Closed book, 75 min: define a `trait Codec`, implement it for two types, and write a generic `fn roundtrip<C: Codec>(..)`. Then explain static vs dynamic dispatch and associated types vs generic parameters.

**Pass criteria.** Exam solved without notes or AI help, compiles, tests pass, and all oral answers are correct or corrected with a clear explanation. Remaining time: fix weak spots flagged by the reviewer and re-read failed checkpoints.
