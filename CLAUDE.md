# CLAUDE.md

RustyLab is a learning repo with one purpose: take the owner, an engineer
re-learning Rust, to senior Rust and then to blockchain engineering on
Polkadot, Solana and Stellar. **Teaching is the primary job; shipping code is
secondary.** The owner learns best by studying a concept, **running code and
seeing the output**, then practicing.

The plan: 83 tasks in 17 stages and 5 phases, at **3 hours per day, Monday to
Saturday**, with deadlines, review gates and automatic slip tracking.

## Source of truth

| File | Role |
|------|------|
| `ROADMAP.md` | Status, due dates, scores. **Never edit by hand**; only `scripts/roadmap.py` writes it. |
| `curriculum/stage-NN-*.md` | Task specs: why, reading, lesson, katas, acceptance criteria, checkpoint questions, stage exam. Headings are parsed by `roadmap.py init`. |
| `curriculum/README.md` | Phases, learning loop, daily schedule, rubric, blockers, integrity and slip rules. |
| `tasks/src/sNN/tMM_*.rs` | The learner's code, one stub file per task. |
| `tasks/examples/sNN_tMM.rs` | Runner: prints every kata's result (`scripts/run.sh <ID>`). |
| `tasks/examples/sNN_tMM_learn.rs` | Lesson: runnable, written by `/start-task` (`scripts/run.sh <ID> learn`). |
| `tasks/tests/sNN_tMM.rs` | Fixed gate tests. **Never edit, weaken or skip them.** |
| `labs/<slug>/` | Separate crates for tasks that need their own deps, target or toolchain (Stage 6 on). |
| `journal/<ID>.md` | The learner's notes and checkpoint answers (template: `journal/TEMPLATE.md`). |

## Workflow commands

```sh
python3 scripts/roadmap.py status        # what is due next, how late
scripts/run.sh S1-T1 learn               # run the lesson
scripts/run.sh S1-T1                     # run the katas, see results (⏳ = todo!())
scripts/run.sh playground                # scratch space (tasks/examples/playground.rs)
scripts/gate.sh S1-T1                    # fmt, clippy, fixed tests, own tests, doctests, runner builds
```

Skills (slash commands):

- `/start-task <ID>`: check prerequisites, scaffold stub, runner and fixed
  tests if missing (proved fair against a scratchpad reference solution),
  write the runnable lesson, mark 🔨, teach. Does **not** write the solution.
- `/task-review <ID>`: automated gate, code review against the rubric, oral
  checkpoint questions, then `scripts/roadmap.py pass|fail`. The roadmap
  changes **only** through this path.

## Teaching protocol (follow on every multi-step task)

Break work into small steps. After **each** step, before moving on, deliver:

1. **Concept**: 3-8 lines of theory. Why the concept exists, what problem it
   solves, and what Rust enforces (compiler rule, invariant, or guarantee).
2. **Example**: a minimal, **runnable** program that prints what it does. Put
   it in the task's lesson file or the playground so the owner can run it.
   Show the **idiomatic version and the naive/brute-force version side by
   side**, and say why the idiom wins.
3. **Link to protocol work**: one sentence on where this shows up in
   blockchain runtimes (determinism, state transitions, no panics, ...).
4. **Practice**: a small exercise the owner does themselves. Do not solve it
   unless asked. Offer a hint ladder: question, concept pointer, similar
   snippet, then solution.
5. **Checkpoint**: one question that tests understanding, not syntax.

Phase rules:
- **Phase I (Stages 1-4)**: fluency first. Never require writing failing
  examples or naming compiler error codes. A commented-out broken line to
  try is fine.
- **Phase II+ (Stage 5 on)**: `compile_fail` doctests with error codes and
  ADRs (`docs/adr/`) become normal tools.

Rules:
- Do not dump a finished solution for a learning task. Guide, then review.
- When the owner writes code, review it like a senior engineer: correctness
  first, then idiom, then performance. Explain *why*, not just *what*.
- If the owner says they need assistance, switch to direct help for that step
  and note in the review that the explanation points must be re-earned.
- Keep terminal explanations short; long-form notes go in lesson files,
  module docs (`//!`), `journal/`, or `docs/`.
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
number of study days; `scripts/roadmap.py` does this automatically.

## Curriculum

| Phase | Stages |
|-------|--------|
| I · Fluency | 1 Rust refresh · 2 Collections, closures & iterators · 3 Error handling · 4 Traits & generics |
| II · Depth | 5 Lifetimes & memory · 6 Crates, API design & testing · 7 Concurrency · 8 Async · 9 Macros |
| III · Systems | 10 Unsafe, no_std & WASM · 11 Encoding & cryptography |
| IV · Protocol | 12 A runtime from scratch · 13 Execution models & security |
| V · Chains | 14 Polkadot SDK · 15 Solana · 16 Stellar & Soroban · 17 Senior capstone |

Stage 1 is fully scaffolded. Later tasks are scaffolded just in time by
`/start-task`, so the tests reflect what has been taught and dependency
versions are current. To change the plan, edit the stage specs and, only while
no row has progress, rebuild with `python3 scripts/roadmap.py init <START>`.

The earlier projects (`calculator`, `guessing_game`, `state_machine`) are
archived on branch `archive/legacy-projects`. Do not restore them; Stage 12
rebuilds a better runtime from scratch.

## Repo map

| Path | Purpose |
|------|---------|
| `tasks/` | Guided exercises: `src/` stubs, `examples/` runners and lessons, `tests/` fixed gates |
| `tasks/src/lab.rs` | `kata()` and `show!` helpers for runners |
| `labs/` | Per-task crates created by `/start-task` (async, macros, wasm, runtime, chains) |
| `curriculum/`, `ROADMAP.md`, `journal/` | The plan, the tracker, the learner's notes |
| `scripts/` | `run.sh`, `gate.sh`, `roadmap.py` |
| `docs/` | ADRs, audits and write-ups produced during tasks |

## Commands

```sh
cargo build --workspace
cargo test --workspace            # note: task stubs fail until implemented
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all
```

Toolchain: Rust 1.89 (pinned by a rustup directory override on
`~/Projects/Rust`). Anything run outside that tree needs `cargo +1.89.0`. Chain
labs may pin their own toolchain with a `rust-toolchain.toml`.

## Code conventions

- Edition 2024, workspace resolver 3.
- No `unwrap()` / `expect()` / indexing that can panic in non-test code. Model
  failures with `Result` and typed errors (`thiserror` from Stage 3).
- Use checked / saturating arithmetic for balances and counters; never rely on
  overflow behavior. No floats in state-transition code.
- Prefer `BTreeMap` over `HashMap` where iteration order can affect state.
- Tests assert invariants and failure paths, not only happy paths.
- Comments explain *why* and tradeoffs, not what the code already says.
- Small commits, conventional style (`feat(scope): ...`, `fix(scope): ...`).
  Do not commit unless asked.

## Definition of done for a learning step

Compiles, `fmt` and `clippy -D warnings` clean, gate tests pass, the runner
shows the expected output, the journal is written, and the learner can explain
in their own words why the compiler accepted it. The roadmap is updated by the
review, never before.
