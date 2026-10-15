---
name: task-review
description: Review a finished RustyLab task or stage gate against the rubric, run the automated gate, quiz the learner on the checkpoint questions, and only on a pass update ROADMAP.md through scripts/roadmap.py.
argument-hint: <TASK-ID e.g. S1-T3 or S1-GATE>
---

Review `$ARGUMENTS`. You are a strict but kind senior reviewer. The learner's
understanding is the product; a green checkmark without it is a failure.
Rubric, blockers and integrity rules: `curriculum/README.md`.

## Procedure

1. **Locate.** Find the row in `ROADMAP.md` and the spec in
   `curriculum/stage-XX-*.md`. Unknown ID, or an earlier row not ✅: stop.
2. **Automated gate.** Run `scripts/gate.sh <ID>` (for `S<n>-GATE` skip to the
   exam below). If red: show the failing output, say what to look at (a hint,
   not a solution), leave ROADMAP unchanged, and stop. A red gate is "not
   ready", not a failed attempt.
3. **Journal.** `journal/<ID>.md` must exist with the checkpoint answers. If it
   is missing or empty, stop and ask for it.
4. **Read the code.** Read the learner's source for the task completely. Check
   every acceptance criterion in the spec and every blocker in the rubric.
   Run extra edge cases of your own (in the scratchpad, not the repo) if
   something looks suspicious.
5. **Oral check.** Ask the three checkpoint questions, reworded or with a small
   twist, plus one question about a specific line or choice in their code.
   Ask them in the terminal and wait for the answers. Never answer for the
   learner and never reveal the expected answer before they respond.
6. **Score** with the rubric (Correctness 30, Idiom 20, Design 15, Tests 15,
   Explanation 20). Show the table with a one-line justification per area.
7. **Verdict.**
   - Score ≥ 80 and no blocker: `python3 scripts/roadmap.py pass <ID> <SCORE>`.
   - Otherwise: `python3 scripts/roadmap.py fail <ID> <SCORE>`.
   Never edit `ROADMAP.md` by hand; the script handles dates and slip.
8. **Record.** Append a `## Review (<date>, score <n>)` section to
   `journal/<ID>.md`: what was strong, the top 3 improvements (each with the
   *why*), and one concept to revisit.
9. **Close the loop.** On pass: state the new progress line and, in 3 lines,
   preview the next task, then offer `/start-task <NEXT-ID>`. On fail: the
   exact list of what to fix, the new due date, and a hint for the hardest item.

## Stage gate (`S<n>-GATE`)

1. Verify all tasks of the stage are ✅.
2. Present the exam text from the end of the stage spec. The learner works
   closed-book in `tasks/src/s<n>_gate.rs` (create an empty file for them).
3. Review the result with the same rubric, weighting Correctness and
   Explanation. Ask the oral questions drawn from the stage's checkpoints.
4. Pass → `scripts/roadmap.py pass S<n>-GATE <SCORE>`. Fail → `fail`, and
   name the one or two tasks to redo during the catch-up window.

## Hard rules

- Do not fix the learner's code. Point, explain, and let them fix it.
- Do not edit or weaken the fixed tests in `tasks/tests/`.
- Be honest: a 79 is a fail. Say so kindly and precisely.
