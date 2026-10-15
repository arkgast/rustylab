---
name: start-task
description: Begin a RustyLab roadmap task. Checks prerequisites, scaffolds stubs and fixed gate tests if missing, marks the task as doing, and teaches the concept with an example before the learner writes any code.
argument-hint: <TASK-ID e.g. S1-T3>
---

Start task `$ARGUMENTS` of the RustyLab curriculum.

1. **Validate.** Read `ROADMAP.md` and `curriculum/stage-XX-*.md` for the task.
   If the ID is unknown, or an earlier row is not ✅ done, say so and stop.
2. **Scaffold if missing.** Stage 1 already has stubs and tests. For any other
   task, create (matching the Stage 1 conventions in `tasks/src/s01/*` and
   `tasks/tests/s01_*.rs`):
   - `tasks/src/sNN/tMM_<slug>.rs`: public API stubs with `todo!()` bodies and
     `_`-prefixed parameters, a module doc stating the rules and the gate
     command. Register it in `tasks/src/sNN/mod.rs` and `tasks/src/lib.rs`.
   - `tasks/tests/sNN_tMM.rs`: fixed tests derived from the acceptance
     criteria, including source-level checks (no banned constructs) where the
     criteria demand them.
   - Before handing over, prove the tests are satisfiable: write a reference
     solution in the session scratchpad (never in the repo), run the tests
     against it with the pinned toolchain, then discard it. Fix any test that
     is wrong or unfair.
   - `scripts/gate.sh <ID>` must run (fmt, clippy -D warnings, tests, doctests).
   Do not leave solutions in the repo.
3. **Mark started.** `python3 scripts/roadmap.py start <ID>`.
4. **Teach before the learner codes** (follow CLAUDE.md teaching protocol):
   - Concept: 3-8 lines of theory, why it exists, what Rust enforces.
   - Example: minimal runnable snippet on a *different* problem than the task
     (a failing-then-fixed one when the lesson is a compiler rule).
   - Protocol link: one sentence tying it to blockchain/runtime work.
   - Practice: restate the task's deliverable, the acceptance criteria and
     the due date, and point to `journal/TEMPLATE.md`.
   - Do not write the learner's solution.
5. **End** with the exact next commands: edit the file path, run
   `scripts/gate.sh <ID>`, write `journal/<ID>.md`, then `/task-review <ID>`.
