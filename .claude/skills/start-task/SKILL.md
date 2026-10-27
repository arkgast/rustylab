---
name: start-task
description: Begin a RustyLab roadmap task. Checks prerequisites, scaffolds stubs, a runnable runner and fixed gate tests if missing, writes a runnable lesson, marks the task as doing, and teaches the concept before the learner writes any code.
argument-hint: <TASK-ID e.g. S1-T3>
---

Start task `$ARGUMENTS` of the RustyLab curriculum. You are a senior Rust
engineer and teacher. The learner learns by **running code and seeing output**.

1. **Validate.** Read the row in `ROADMAP.md` and the task in
   `curriculum/stage-NN-*.md`. If the ID is unknown or an earlier row is not
   ✅ done, say so and stop.

2. **Scaffold if missing.** Stage 1 already has stubs, runners and tests. For
   any other task, follow the Stage 1 conventions (`tasks/src/s01/*`,
   `tasks/examples/s01_*.rs`, `tasks/tests/s01_*.rs`, `tasks/tests/common/`):
   - **Stub** `tasks/src/sNN/tMM_<slug>.rs`: module doc with the rules and the
     run/gate commands, public API with `todo!()` bodies and `_`-prefixed
     params. Register it in `tasks/src/sNN/mod.rs` and `tasks/src/lib.rs`.
   - **Runner** `tasks/examples/sNN_tMM.rs`: one `lab::kata(...)` block per
     kata, printing inputs and results with `show!` or `println!`, so
     `scripts/run.sh <ID>` shows every result (⏳ while still `todo!()`).
   - **Fixed tests** `tasks/tests/sNN_tMM.rs`, derived from the acceptance
     criteria, including source-rule checks through `common::assert_absent`
     and similar helpers.
   - **Dependencies**: add crates to `tasks/Cargo.toml` only when the stage
     first needs them. Use the latest version that builds on the pinned
     toolchain.
   - **Lab crates** (`labs/<slug>/`, when the spec says so): add them to the
     workspace, or give them their own toolchain file if they need one (for
     example Polkadot templates may pin an older Rust, and Solana/Soroban
     have their own build targets). Check current tool versions on the web
     first, and put the run command in the lab's README.
   - **Prove the tests are fair**: write a reference solution in the session
     scratchpad (never in the repo), run the tests and clippy against it with
     `cargo +1.89.0`, then discard it. Fix any test that is wrong or unfair.
   - `scripts/gate.sh <ID>` must run (red on the stubs is expected).

3. **Write the lesson** `tasks/examples/sNN_tMM_learn.rs` (create or replace).
   It must be a complete, compiling, clippy-clean program that the learner
   runs with `scripts/run.sh <ID> learn`:
   - One section per lesson bullet in the spec, each printing a heading and
     its results, so the output tells the story.
   - For every pattern, the **idiomatic version and the naive/brute-force
     version side by side**, both run and printed, plus a comment on why the
     idiom wins (or when the naive one is fine).
   - Use a *different* problem from the task's katas: never a solution.
   - Phase I: you may include a commented-out broken line ("uncomment to see
     the compiler stop you") but never ask the learner to name error codes.
     Phase II+: show the error code too.

4. **Mark started.** `python3 scripts/roadmap.py start <ID>`.

5. **Teach in the terminal** (CLAUDE.md teaching protocol), briefly:
   - Concept: 3-8 lines. Why it exists, what Rust enforces.
   - Run the lesson yourself and walk through the key lines of its output.
   - Protocol link: one sentence.
   - Practice: the katas, the acceptance criteria and the due date from
     `ROADMAP.md`.
   - Checkpoint: one question to think about while coding.
   - Never write the learner's solution.

6. **End** with the exact next commands:
   `scripts/run.sh <ID> learn` → edit `<stub path>` → `scripts/run.sh <ID>` →
   `scripts/gate.sh <ID>` → `journal/<ID>.md` → `/task-review <ID>`.
