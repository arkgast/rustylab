# Stage 8: Macros

**Outcome:** You can read and write declarative and procedural macros, test them, and give good diagnostics.

**Window:** 2027-01-15 → 2027-01-28 · tracker: [`ROADMAP.md`](../ROADMAP.md)

Each task: read first, build in `tasks/` (or the crate named), write the journal entry, then run `/task-review <ID>`.

## S8-T1 · macro_rules! fundamentals
*2 days (6 h) · due 2027-01-16*

**Why it matters.** Macros let you abstract over syntax. You need them to read any serious Rust codebase.

**Read first**
- The Little Book of Rust Macros: first chapters
- The Rust Reference: Macros By Example

**Build.** `hashmap!`, `btreemap!`, `assert_matches_all!` and `bitflags_lite!` using fragment specifiers and repetitions.

**Acceptance criteria** (all required)
- [ ] Each macro has 3+ tests including trailing comma and empty input
- [ ] A compile-fail doctest for misuse with the error visible in the journal
- [ ] Journal explains hygiene with an example that would collide without it

**Stretch.** Make `hashmap!` pre-size the map with a count computed at compile time.

**Checkpoint questions** (answer in `journal/S8-T1.md`; the reviewer will ask them again)
1. What are `expr`, `ident`, `tt`, `ty` and when do you need `tt`?
2. What does hygiene protect and what doesn't it?
3. When should a function replace a macro?

## S8-T2 · Advanced declarative macros
*2 days (6 h) · due 2027-01-19*

**Why it matters.** Recursion, internal rules and tt-munchers are the patterns behind most production `macro_rules!`.

**Read first**
- The Little Book of Rust Macros: patterns
- Look at how `bitflags` or `matches!` are written

**Build.** `str_enum!` generating an enum with `Display`, `FromStr`, `ALL` const and `COUNT`, via internal rules or a tt-muncher.

**Acceptance criteria** (all required)
- [ ] Generates for any number of variants (test with 1, 3 and 20)
- [ ] `cargo expand` output annotated in the journal
- [ ] Error messages for invalid input are understandable
- [ ] Respects visibility and attributes passed through

**Stretch.** Support explicit discriminants.

**Checkpoint questions** (answer in `journal/S8-T2.md`; the reviewer will ask them again)
1. What is a tt-muncher and why does it hit the recursion limit?
2. How do internal rules (`@name`) work?
3. What are the limits of `macro_rules!` that make proc macros necessary?

## S8-T3 · Derive macros with syn & quote
*2 days (6 h) · due 2027-01-21*

**Why it matters.** Derive macros are the most common proc-macro form you will write or read.

**Read first**
- `syn`, `quote`, `proc-macro2` docs
- David Tolnay's `proc-macro-workshop`

**Build.** A new `describe_derive` proc-macro crate: `#[derive(Describe)]` generating `fn describe() -> &'static str` and `fn field_names() -> &'static [&'static str]` for named structs.

**Acceptance criteria** (all required)
- [ ] Works with generics and where clauses (use `split_for_impl`)
- [ ] Rejects enums and tuple structs with a spanned `syn::Error`
- [ ] Tests with `trybuild` for both pass and fail cases
- [ ] Journal: token stream vs AST vs `proc_macro2`

**Stretch.** Support `#[describe(skip)]` helper attribute.

**Checkpoint questions** (answer in `journal/S8-T3.md`; the reviewer will ask them again)
1. Why do proc macros live in their own crate?
2. What is a `Span` and why does it matter for diagnostics?
3. Why is `proc_macro2` used instead of `proc_macro`?

## S8-T4 · Attribute & function-like macros
*2 days (6 h) · due 2027-01-23*

**Why it matters.** Attribute macros transform items; function-like macros parse custom syntax. The `state_machine` macros are in this family.

**Read first**
- The Rust Reference: Procedural Macros
- Read `state_machine/macros/src/call/parse.rs`

**Build.** A `#[timed]` attribute that wraps a function body with `Instant` timing and a `route!{ GET '/x' => handler }` function-like macro with custom `Parse`.

**Acceptance criteria** (all required)
- [ ] `#[timed]` preserves signature, attributes, generics and `async`-ness (document limits)
- [ ] `route!` implements `syn::parse::Parse` and gives spanned errors
- [ ] `trybuild` pass/fail tests
- [ ] Journal: parse → validate → expand separation, compared with the `state_machine` macros

**Stretch.** Emit `compile_error!` with multiple errors using `syn::Error::combine`.

**Checkpoint questions** (answer in `journal/S8-T4.md`; the reviewer will ask them again)
1. How do attribute macros receive input differently from derive?
2. How do you give an error pointing at the user's token?
3. What can't a proc macro know?

## S8-T5 · Capstone: extend the runtime macros
*3 days (9 h) · due 2027-01-27*

**Why it matters.** Extending a macro you didn't write is exactly what you'll do on a real team.

**Read first**
- `state_machine/macros/src/{call,runtime}`

**Build.** Agree one feature with the reviewer (e.g. generate `Call` docs, per-call weight attribute, or aggregate `Error` enums) and implement it end to end in `state_machine/macros`.

**Acceptance criteria** (all required)
- [ ] Feature has `trybuild` tests and runtime tests
- [ ] Expanded code annotated in `docs/macros.md` using `cargo expand`
- [ ] Error messages tested for at least 3 misuse cases
- [ ] No change to existing behavior (all existing tests green)

**Stretch.** Write a short guide: 'How `runtime!` expands'.

**Checkpoint questions** (answer in `journal/S8-T5.md`; the reviewer will ask them again)
1. Walk through the expansion of `runtime!` from input tokens to generated items.
2. Where would a bug in the macro surface: expansion or use site?
3. What did you change in parse vs expand and why?

## S8-GATE · Stage gate
*1 day (3 h) · due 2027-01-28*

**Exam.** Closed book, 90 min: write a `macro_rules!` with repetition and an internal rule, and a derive macro skeleton with `syn`/`quote`; explain hygiene and spans.

**Pass criteria.** Exam solved without notes or AI help, compiles, tests pass, and all oral answers are correct or corrected with a clear explanation. Remaining time: fix weak spots flagged by the reviewer and re-read failed checkpoints.
