# Stage 9: Macros

**Phase:** II · Depth
**Outcome:** You read and write `macro_rules!` with repetitions and internal rules, build derive, attribute and function-like proc macros with `syn` and `quote` that fail with precise error spans, test them with `trybuild`, and read any framework macro's expansion with `cargo expand`.

Each task: `/study` (recall, then probes: predict, run, explain the gap), solve the katas, run them and read the output, make the gate green, write the journal, then `/review <ID>`.

## S9-T1 · macro_rules! fundamentals
*2 days (6 h)*

**Why it matters.** Macros remove boilerplate that functions and generics cannot: variadic arguments, generated items, compile-time DSLs. FRAME, Anchor and the Soroban SDK are mostly macros; you cannot debug them until you can write one.

**Probes**
- Matchers and transcribers; fragment specifiers (`expr`, `ident`, `ty`, `tt`, `literal`, `pat`, `vis`) and why `expr` cannot be followed by just anything
- Repetitions `$(...),*` / `$(...);+` and nested repetitions; a copy-pasted `impl` block per type vs one macro that generates all of them
- Hygiene: a local `let` inside a macro does not clash with the caller's; `cargo expand` to see the output; `#[macro_export]` and textual scope

**Reference** (read after the probes, or when stuck)
- The Rust Book ch. 20.5 (Macros)
- The Little Book of Rust Macros: "Macros, A Methodical Introduction" (all of it)
- The Rust Reference: "Macros By Example" (fragment specifiers, follow-set rules, hygiene)

**Build.** `tasks/src/s09/t01_macro_rules.rs`: `btreemap!{ k => v, ... }` (trailing comma ok), `max!(a, b, c, ...)` with checked comparison, `newtype!(pub AccountId(u64))` generating the struct + `From`/`Display`, `impl_checked_ops!(Balance, u128)` generating `checked_add`/`checked_sub`, and `ensure!(cond, err)` returning `Err(err.into())`.

**Run.** `scripts/run.sh S9-T1`; `cargo expand -p tasks s09::t01_macro_rules` to read the expansion.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s09_t01.rs` pass
- [ ] A `compile_fail` doctest shows `max!()` with zero args is rejected at compile time
- [ ] A test proves hygiene: a caller variable with the same name as a macro-internal binding is unaffected
- [ ] Generated code contains no `unwrap`/`expect`
- [ ] Journal includes one `cargo expand` output, annotated

**Stretch.** Make `newtype!` accept optional derives: `newtype!(#[derive(Hash)] pub Nonce(u64))`.

**Checkpoint questions** (answer in `journal/S9-T1.md`; the reviewer asks them again)
1. When is a macro the right tool, and when is a generic function or trait better?
2. What does macro hygiene protect against, and what is *not* hygienic in `macro_rules!`?
3. Why can a `$e:expr` fragment not be followed by an arbitrary token?

## S9-T2 · Advanced declarative macros
*2 days (6 h)*

**Why it matters.** Real-world declarative macros parse small languages: `vec!`, `matches!`, `tokio::select!`, `frame_support::construct_runtime!` (historically). TT-munchers, internal rules and `$crate` are how they stay correct across crates.

**Probes**
- Incremental TT munchers: consume tokens one rule at a time; recursion limit
- Internal rules (`(@step ...)`) and push-down accumulation; counting tokens at compile time
- `$crate::path` so the macro works from any crate; `compile_error!` for helpful failures

**Reference** (read after the probes, or when stuck)
- The Little Book of Rust Macros: "Patterns" (Callbacks, Incremental TT Munchers, Internal Rules, Push-down Accumulation, Counting)
- The Rust Reference: "Macros By Example", `$crate` and "Hygiene" sections

**Build.** `tasks/src/s09/t02_advanced_macros.rs`: `count!(a b c) == 3` as a `const`; `bitflags_lite!{ struct Perms: u8 { READ = 1, WRITE = 2 } }` generating consts, `contains`, `union`; `state_machine!{ Idle -> Running on Start; Running -> Idle on Stop; }` generating a `State` enum, `Event` enum and `fn next(self, Event) -> Result<State, InvalidTransition>`; `storage_map!` using `$crate` paths only.

**Run.** `scripts/run.sh S9-T2`

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s09_t02.rs` pass
- [ ] `count!` evaluates in a `const` context
- [ ] Every path in exported macros starts with `$crate::` or `::core::`/`::std::`
- [ ] Invalid `state_machine!` input yields a `compile_error!` with a readable message (shown in a `compile_fail` doctest)
- [ ] Generated `next` uses an exhaustive `match` with no `_ =>` arm on states

**Stretch.** Rewrite `state_machine!` to reject duplicate transitions at compile time.

**Checkpoint questions** (answer in `journal/S9-T2.md`; the reviewer asks them again)
1. How does a TT muncher make progress, and why can it hit the recursion limit?
2. Why does an exported macro need `$crate`, and what breaks without it?
3. At what point does a declarative macro become a proc macro, and why?

## S9-T3 · Derive macros with syn & quote
*2 days (6 h)*

**Why it matters.** `#[derive(Encode, Decode, TypeInfo)]` in Substrate, `#[derive(AnchorSerialize)]` and `#[contracttype]` in Soroban are proc macros. Writing one teaches you how they see your code, why they reject some inputs, and how to read their errors.

**Probes**
- A proc-macro crate (`proc-macro = true`): `TokenStream` in, `TokenStream` out; `syn` parses, `quote` generates
- Errors: `panic!` in a macro (ugly, no span) vs `syn::Error::new_spanned(field, "...")` pointing at the exact field
- Generics: `split_for_impl` and adding where-clause bounds; testing with `trybuild` pass/fail UI tests and `.stderr` snapshots

**Reference** (read after the probes, or when stuck)
- The Rust Reference: "Procedural Macros"
- dtolnay's proc-macro-workshop: `builder` project README and tests 01-06
- `syn` docs: `DeriveInput`, `Data`, `Fields`, `Error::new_spanned`, `Error::to_compile_error`; `quote` docs: `quote!`, `format_ident!`, repetition `#(...)*`
- `trybuild` README

**Build.** `labs/macros-lab/` workspace: `encode-derive` (proc-macro crate) and `encode` (runtime crate with `trait Encode { fn encode_to(&self, out: &mut Vec<u8>); }` impls for ints, `bool`, `Vec<T>`, `String`). `#[derive(Encode)]` for named structs, tuple structs and enums (variant index as `u8` prefix), supporting generics and a `#[encode(skip)]` field attribute.

**Run.** `cd labs/macros-lab && cargo test`; `cargo expand -p encode --example demo`.

**Acceptance criteria** (all required)
- [ ] Round-trip tests pass for structs, tuple structs, enums and a generic struct
- [ ] `trybuild` UI tests: at least 3 pass cases and 3 fail cases (union, unknown attribute, enum with more than 256 variants or a non-`Encode` field) with committed `.stderr`
- [ ] No `panic!`/`unwrap` in the macro; every user error is a spanned `syn::Error`
- [ ] Generated code uses fully qualified paths (`::encode::Encode`, `::core::...`)

**Stretch.** Complete proc-macro-workshop `builder` tests 07-09.

**Checkpoint questions** (answer in `journal/S9-T3.md`; the reviewer asks them again)
1. Why must a proc macro live in its own crate, and what does it run on (host or target)?
2. Why is a spanned `syn::Error` better than `panic!`, from the user's point of view?
3. How did you handle generics, and what bound did you add to the generated `impl`?

## S9-T4 · Attribute & function-like macros
*2 days (6 h)*

**Why it matters.** Attribute macros rewrite the item they decorate (`#[tokio::main]`, `#[pallet::call]`, `#[program]`, `#[contractimpl]`). Function-like macros parse custom syntax (`sqlx::query!`, `construct_runtime!`). Both are the frameworks' main surface.

**Probes**
- `#[proc_macro_attribute]`: `(attr, item)`; wrapping a function body while keeping its signature and span
- `#[proc_macro]` with a custom `impl Parse` for a small grammar
- Preserving user code: emit the original item unchanged plus generated items vs mutating it in place with `VisitMut`

**Reference** (read after the probes, or when stuck)
- proc-macro-workshop: `sorted` (attribute) and `seq` (function-like) READMEs and first tests
- `syn` docs: `ItemFn`, `ItemImpl`, `parse::Parse`, `parse_macro_input!`, `visit_mut::VisitMut`
- The Little Book of Rust Macros: "Procedural Macros" chapter

**Build.** `labs/macros-lab/`: `#[timed]` attribute that wraps a function and logs its duration (works on `async fn` too); `#[weight(10)]` attribute that adds a `const WEIGHT_<NAME>: u64`; function-like `units!(1.5 DOT)` that converts a decimal literal to an integer `u128` with `DECIMALS = 10` at compile time, rejecting precision loss and overflow with spanned errors.

**Run.** `cd labs/macros-lab && cargo test`; `cargo expand --example attrs`.

**Acceptance criteria** (all required)
- [ ] Tests and `trybuild` UI tests pass (at least 2 fail cases per macro)
- [ ] `#[timed]` preserves visibility, generics, `async` and return type
- [ ] `units!` does all arithmetic at expansion time with checked math; the output is a single `u128` literal
- [ ] No floats in `units!` (parse the decimal string), no `panic!` in any macro

**Stretch.** Make `#[timed]` accept `#[timed(name = "transfer")]` using `syn::meta::parser`.

**Checkpoint questions** (answer in `journal/S9-T4.md`; the reviewer asks them again)
1. What are the two token streams an attribute macro receives, and what must it return?
2. Why parse `1.5 DOT` as a string instead of an `f64`?
3. What goes wrong in user error messages if your macro loses the original spans?

## S9-T5 · Capstone: a #[module] attribute macro that generates dispatch
*3 days (9 h)*

**Why it matters.** FRAME's `#[pallet::call]`, Anchor's `#[program]` and Soroban's `#[contractimpl]` all do the same thing: read an `impl` block and generate a `Call` enum plus a dispatch function that routes encoded calls to methods. Build it yourself and those frameworks stop being opaque.

**Probes**
- Reading an `ItemImpl`: methods, `self` receivers, argument patterns and types; skipping non-call items
- Generating `enum Call { Transfer { to: AccountId, amount: u128 }, ... }`, `impl Dispatch for Module` with an exhaustive `match`, and `Encode`/`Decode` via your S9-T3 derive
- Validation as spanned errors: first arg must be `caller: AccountId`, return type must be `DispatchResult`, no generics, no `self` by value

**Reference** (read after the probes, or when stuck)
- Polkadot SDK docs: `frame_support::pallet_macros::call` and "FRAME macros" overview
- Anchor docs: "Program module" (`#[program]`)
- Re-read your S9-T3, S9-T4 journals and, optionally, the archived `call!` macro (`git show archive/legacy-projects:state_machine/macros/src/call/mod.rs`)

**Build.** `labs/macros-lab/module-macro/` + `labs/macros-lab/module-demo/`: `#[module]` on `impl Balances { fn transfer(&mut self, caller: AccountId, to: AccountId, amount: u128) -> DispatchResult; ... }` generates `balances::Call`, `Call::dispatch(self, &mut Balances, caller) -> DispatchResult`, `Call::name(&self) -> &'static str`, and a `const CALLS: &[&str]`. Demo runtime with two modules and an outer `RuntimeCall` enum written by hand. ADR in `docs/adr/` on what the macro generates and why.

**Run.** `cd labs/macros-lab && cargo run -p module-demo` executes an encoded block of calls; `cargo expand -p module-demo` shows the generated code.

**Acceptance criteria** (all required)
- [ ] Dispatch tests: every call routes correctly; encoded calls round-trip; unknown variant index is a typed decode error
- [ ] `trybuild` UI fail cases with spans on the offending token: missing `caller`, wrong return type, generic method, `self` by value
- [ ] Generated code has no `unwrap`, `panic!` or wildcard match arm; paths fully qualified
- [ ] Annotated `cargo expand` output in the journal
- [ ] ADR written

**Stretch.** Add `#[module::weight(expr)]` per method and generate `Call::weight(&self) -> u64`, then a `runtime!` function-like macro that builds the outer `RuntimeCall` from a module list.

**Checkpoint questions** (answer in `journal/S9-T5.md`; the reviewer asks them again)
1. Walk through what your macro generates for one method, and why each generated item exists.
2. Which validations belong in the macro and which in the runtime, and why?
3. How is your design similar to and different from FRAME's `#[pallet::call]` or Anchor's `#[program]`?

## S9-GATE · Stage 9 gate: closed-book exam + catch-up
*1 day (3 h)*

**Exam.** Closed book, 90 min: in `tasks/src/s09_gate.rs` write `hashmap_of!(K, V; k => v, ...)` with trailing comma support and a `count!`-style const; in `labs/s09-gate/` write `#[derive(Describe)]` that generates `fn describe() -> &'static str` listing field names and types for named structs, rejecting enums with a spanned `syn::Error`, plus one `trybuild` fail test. Read a given `cargo expand` output and explain it. Then answer 5 oral questions drawn from the S9 checkpoints.

**Pass criteria.** Solved without notes or AI, compiles, tests pass, the fail test shows the correct span, oral answers correct or corrected with a clear explanation. Remaining time: redo the weakest task's core from a blank file.
