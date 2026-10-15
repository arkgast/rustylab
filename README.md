# RustyLab

A structured, deadline-driven path from Rust fundamentals to protocol
engineering.

This is not a pile of random exercises. It is a deliberate deep dive into the
mechanics that make Rust suited to high-assurance systems such as blockchain
infrastructure: the goal is to understand *why* Rust enforces its constraints
and how they rule out whole classes of bugs.

## The plan at a glance

- **10 stages, 50 tasks**, plus a closed-book exam at the end of every stage
- **3 focused hours per day, Monday to Saturday**; Sunday is spaced repetition
- **~375 study hours** in about 21 weeks (see [`ROADMAP.md`](ROADMAP.md) for dates)
- Every task has **acceptance criteria, fixed tests, a journal entry and an
  oral review**. A task is done only when the review passes.

| # | Stage | You will be able to... |
|---|-------|------------------------|
| 1 | Foundations | predict borrow-checker outcomes; model failure with `Option`/`Result` |
| 2 | Types & traits | make invalid states unrepresentable; choose generics vs `dyn` |
| 3 | Lifetimes & memory | reason about where data lives and for how long |
| 4 | Error design | design error APIs; audit for panics; make operations atomic |
| 5 | Collections & iterators | pick deterministic collections; write lazy pipelines |
| 6 | Testing & correctness | specify invariants; property-test against a model; benchmark |
| 7 | Concurrency | write race-free parallel code; prove parallel equals sequential |
| 8 | Macros | read and write declarative and procedural macros |
| 9 | State machines | build deterministic transitions with commit/rollback |
| 10 | Protocol engineering | build hashing, Merkle proofs, signatures, codec, consensus, gossip; map to FRAME |

## Learning method

Each task follows the same loop: **concept → example → build → journal →
review**. Every day is 20 min recall, 30 min concept, 100 min build, 20 min
self-review, 10 min journal. Full rules, rubric and integrity policy:
[`curriculum/README.md`](curriculum/README.md).

## How to work a task

```sh
python3 scripts/roadmap.py status     # what is next and whether you are late
# in Claude Code:
/start-task S1-T1                     # scaffold, theory, example
# edit tasks/src/s01/t01_ownership.rs, write journal/S1-T1.md
scripts/gate.sh S1-T1                 # automated gate: fmt, clippy, tests, doctests
/task-review S1-T1                    # rubric + oral check; updates the roadmap on pass
```

## Repository layout

| Path | What it is |
|------|------------|
| `ROADMAP.md` | Tracker: status, due dates, scores (updated only by reviews) |
| `curriculum/` | Task specs per stage and the rules of the game |
| `tasks/` | Your exercises (`src/`) and the fixed gate tests (`tests/`) |
| `journal/` | Your notes and checkpoint answers per task |
| `state_machine/` | Mini blockchain runtime with pallets, reused in later stages |
| `state_machine/macros/` | Procedural macros that generate dispatch code |
| `guessing_game/`, `calculator/` | Early reference programs |
| `scripts/` | `gate.sh` and `roadmap.py` |
| `docs/adr/` | Design decisions you write along the way |

## Getting started

```sh
cargo test -p state_machine            # the runtime you will keep extending
cargo run -p state_machine
scripts/gate.sh S1-T1                  # fails until you implement the stubs: that is the point
```

Rust toolchain: 1.89 (edition 2024).

## Why this matters

Blockchain infrastructure demands determinism, memory safety, concurrency
control, and explicit state transitions. Rust provides these guarantees through
constraints, and RustyLab explores those constraints directly.

## Long-term direction

Moving from application-level Rust to infrastructure-grade Rust suitable for
blockchain runtimes and protocol-level systems (`no_std`, Substrate / FRAME).
