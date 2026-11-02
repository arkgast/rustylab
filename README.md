# RustyLab

A structured, deadline-driven path from re-learning Rust to senior Rust
engineering, and from there to blockchain engineering on **Polkadot, Solana
and Stellar**.

Every concept is learned the same way: **predict what a snippet prints, run it
and see the gap**, then solve katas and run them, pass the gate, explain your
answers.

## The plan at a glance

- **17 stages, 83 tasks, 5 phases**, plus a closed-book exam at the end of every stage
- **3 focused hours a day, Monday to Saturday**; Sunday is spaced repetition
- **189 study days (~567 h)**; dates and progress are local, in `ROADMAP.md` (see Setup)
- A task is done only when its review passes: automated gate, rubric ≥ 80, oral check

| Phase | Stage | You will be able to... |
|-------|-------|------------------------|
| I · Fluency | 1 Rust refresh | write basic programs from memory; see moves and borrows |
| | 2 Collections, closures & iterators | process data with pipelines; pick deterministic collections |
| | 3 Error handling | design typed errors; never panic on bad input |
| | 4 Traits & generics | model domains with traits, newtypes and associated types |
| II · Depth | 5 Lifetimes & memory | reason about where data lives and for how long |
| | 6 Crates, API design & testing | ship library-quality APIs with property tests and benchmarks |
| | 7 Concurrency | write race-free parallel code; prove parallel equals sequential |
| | 8 Async | build tokio services with timeouts, cancellation and backpressure |
| | 9 Macros | read and write declarative and procedural macros |
| III · Systems | 10 Unsafe, no_std & WASM | write sound unsafe code; run sandboxed WASM contracts |
| | 11 Encoding & cryptography | build canonical codecs, Merkle proofs and signed transactions |
| IV · Protocol | 12 A runtime from scratch | build a deterministic blockchain runtime end to end |
| | 13 Execution models & security | exploit and fix the classic smart-contract bugs; audit your own code |
| V · Chains | 14 Polkadot SDK | write, test, benchmark and migrate FRAME pallets |
| | 15 Solana | write native and Anchor programs; test them with LiteSVM / Mollusk |
| | 16 Stellar & Soroban | write and deploy Soroban contracts |
| | 17 Senior capstone | read production chain code, contribute upstream, defend a final project |

## Setup

Needs Rust 1.89 and Python 3. Dates and progress are personal, so they are not
in git. Create them once, with your start date:

```sh
cargo x init 2026-11-02    # writes ROADMAP.md and .study/plan.json
cargo x plan               # start, proposed finish, hours per day, intensity
```

Move to a new machine by copying those two files: they exist nowhere else. The
schedule always assumes 3 hours Monday to Saturday; the plan records that but
changing it there does not reschedule anything.

## Commands

`cargo x` is this repo's `npm run`: an alias in `.cargo/config.toml` for the
small `xtask` crate. `cargo x help` lists everything.

| Command | What it does |
|---------|--------------|
| `cargo x status` | What is due next and how late you are |
| `cargo x plan` | Your dates and intensity |
| `cargo x probe` | Rerun the tutor's snippet (`--release` for the release profile) |
| `cargo x play` | Run your playground, `scratch/src/bin/playground.rs` |
| `cargo x run S1-T1` | Run your katas (short form `cargo x S1-T1`; program args after `--`) |
| `cargo x gate S1-T1` | fmt, clippy, fixed tests, your tests, runner build |
| `cargo x journal S1-T1` | Answer the journal step by step; stop anytime, rerun to continue |
| `cargo x after 3` | The date 3 study days from today |

**Probe or playground?** The probe belongs to the tutor and is overwritten by
every probe, so anything you want to keep goes in the playground, which the
tutor never overwrites. Both live in the gitignored `scratch/` crate, created on
first use, so a broken experiment can never fail your gate.

## A study day

```text
/study                      # opens today: 2-3 recall questions, then probes or build
                            #   a probe: predict, it runs, you see the gap
cargo x probe        # rerun the tutor's snippet after you change it
cargo x run S1-T1        # your katas (⏳ = todo!())
cargo x play   # your scratch file
cargo x gate S1-T1       # fmt, clippy, fixed tests, own tests
cargo x journal S1-T1       # guided, one question at a time; the close message seeds it
/review S1-T1               # oral check + rubric; the only thing that moves the roadmap
/study close                # or just say "done": logs the day, queues tomorrow's recall
```

In chat: hint · skip · show me (costs points) · break · done

Sunday: /study runs only repetition. Rest 10-20 minutes after every close; sleep is
where today's misses become tomorrow's recall.

Full rules, rubric and integrity policy: [`curriculum/README.md`](curriculum/README.md).

## Repository layout

| Path | What it is |
|------|------------|
| `ROADMAP.md` | Local, gitignored tracker: status, due dates, scores (updated only by reviews) |
| `scratch/` | Local, gitignored crate: the tutor's probe and your playground |
| `xtask/` | The `cargo x` command |
| `curriculum/` | Stage specs and the rules of the game |
| `tasks/` | Your exercises (`src/`), runners (`examples/`), fixed gate tests (`tests/`) |
| `labs/` | Separate crates for later stages (async, macros, WASM, runtime, chains) |
| `journal/` | Your notes and checkpoint answers per task |
| `.study/` | Tutor state: today's session, the recall queue, per-task logs (written by `/study` and `/review`) |
| `scripts/` | `run.sh`, `gate.sh`, `roadmap.py`, `clock.sh` (session clock the tutor checks for the break cue) |
| `docs/` | ADRs, audits and write-ups you produce along the way |

Earlier experiments (`calculator`, `guessing_game`, `state_machine`) are
archived on branch `archive/legacy-projects`.

Rust toolchain: 1.89 (edition 2024).
