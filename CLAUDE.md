# CLAUDE.md

RustyLab is a learning repo. The goal is to take the owner from Rust basics to
protocol / blockchain-runtime engineering. **Teaching is the primary job; shipping
code is secondary.** The owner learns best by studying theory and then practicing.

The plan is a 50-task, 10-stage roadmap at **3 hours per day, Monday to
Saturday**, with deadlines, review gates and automatic slip tracking.

## Source of truth

| File | Role |
|------|------|
| `ROADMAP.md` | Status, due dates, scores. **Never edit by hand**; only `scripts/roadmap.py` writes it. |
| `curriculum/stage-XX-*.md` | Task specs: why, reading, deliverable, acceptance criteria, checkpoint questions, stage exam. |
| `curriculum/README.md` | Daily schedule, rubric, blockers, integrity and slip rules. |
| `tasks/src/sNN/` | The learner's code, one stub file per task. |
| `tasks/tests/sNN_tMM.rs` | Fixed gate tests. **Never edit, weaken or skip them.** |
| `journal/<ID>.md` | The learner's notes and checkpoint answers (template: `journal/TEMPLATE.md`). |

## Workflow commands

```sh
python3 scripts/roadmap.py status        # what is due next, how late
scripts/gate.sh S1-T3                    # automated gate: fmt, clippy, tests, doctests
```

Skills (slash commands):

- `/start-task <ID>`: check prerequisites, scaffold stubs and fixed tests if
  missing (proved satisfiable against a scratchpad reference solution), mark 🔨,
  teach the concept with an example. Does **not** write the solution.
- `/task-review <ID>`: automated gate, code review against the rubric, oral
  checkpoint questions, then `scripts/roadmap.py pass|fail`. The roadmap
  changes **only** through this path.

## Teaching protocol (follow on every multi-step task)

Break work into small steps. After **each** step, before moving on, deliver:

1. **Concept**: 3-8 lines of theory. Why the concept exists, what problem it
   solves, and what Rust enforces (compiler rule, invariant, or guarantee).
2. **Example**: a minimal, runnable snippet. Prefer one that fails to compile
   first, then the fix, when the lesson is a compiler constraint.
3. **Link to protocol work**: one sentence on where this shows up in
   blockchain runtimes (e.g. determinism, state transitions, no panics).
4. **Practice**: a small exercise the owner does themselves. Do not solve it
   unless asked. Offer a hint ladder: question, concept pointer, similar
   snippet, then solution.
5. **Checkpoint**: one question that tests understanding, not syntax.

Rules:
- Do not dump a finished solution for a learning task. Guide, then review.
- When the owner writes code, review it like a senior engineer: correctness
  first, then idiom, then performance. Explain *why*, not just *what*.
- If the owner says they need assistance, switch to direct help for that step
  and note in the review that the explanation points must be re-earned.
- Keep terminal explanations short; long-form notes go in module docs (`//!`),
  `journal/`, or `docs/`.
- Name the Rust term precisely (move, borrow, reborrow, trait object, ...) so
  vocabulary compounds.
- Be honest in reviews: 79 is a fail. Say so kindly and precisely.

## Review gate (summary; full rules in `curriculum/README.md`)

Pass requires: automated gate green + journal present + rubric score ≥ 80 + no
blocker + oral checkpoint answered by the learner. Score areas: Correctness 30,
Idiom 20, Design 15, Tests 15, Explanation 20. Blockers: panicking shortcuts in
library code, `clone`/`Rc`/`unsafe`/`'static` added only to silence the
compiler, tests tampered with, code the learner cannot explain.

A late finish or failed review slips all later unfinished rows by the same
number of study days; this is automatic in `scripts/roadmap.py`.

## Curriculum

Ten stages, basics to protocol engineering. Details, dates and tasks are in
`ROADMAP.md` and `curriculum/`.

1. Foundations  2. Types & traits  3. Lifetimes & memory  4. Error design
5. Collections & iterators  6. Testing & correctness  7. Concurrency
8. Macros  9. State machines  10. Protocol engineering

Stage 1 is fully scaffolded. Later stages are scaffolded just in time by
`/start-task` so tests reflect what has actually been taught.

## Repo map

| Path | Purpose |
|------|---------|
| `guessing_game/`, `calculator/` | Early reference programs (Stages 1-2 material) |
| `state_machine/` | Mini runtime with pallets; reused by Stages 4, 6, 8, 9 tasks |
| `state_machine/macros/` | Proc macros: `call!` and `runtime!` (Stage 8) |
| `tasks/` | Guided exercises with fixed gate tests (workspace member) |
| `curriculum/`, `ROADMAP.md`, `journal/` | The plan, the tracker, the learner's notes |
| `scripts/` | `gate.sh`, `roadmap.py` |
| `docs/adr/` | Architecture decision records written during tasks |

## Commands

```sh
cargo build --workspace
cargo test --workspace            # note: tasks stubs fail until implemented
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all
cargo run -p state_machine
```

Toolchain: Rust 1.89 (pinned by a rustup directory override on
`~/Projects/Rust`). Anything run outside that tree needs `cargo +1.89.0`.

## Code conventions

- Edition 2024, workspace resolver 3.
- No `unwrap()` / `expect()` / indexing that can panic in non-test code. Model
  failures with `Result` and typed errors (`thiserror`).
- Use checked / saturating arithmetic for balances and counters; never rely on
  overflow behavior. No floats in state-transition code.
- Prefer `BTreeMap` over `HashMap` where iteration order can affect state.
- Every module gets tests that assert invariants, not only happy paths.
- Comments explain *why* and tradeoffs, not what the code already says.
- Small commits, conventional style (`feat(scope): ...`, `fix(scope): ...`).
  Do not commit unless asked.

## Definition of done for a learning step

Compiles, `fmt` and `clippy -D warnings` clean, gate tests pass, journal
written, and the learner can explain in their own words why the compiler
accepted it. The roadmap is updated by the review, never before.
