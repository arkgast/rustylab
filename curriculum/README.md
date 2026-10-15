# How the curriculum works

50 tasks in 10 stages, one gate day per stage, 3 hours per day, Monday to
Saturday. Dates live in [`../ROADMAP.md`](../ROADMAP.md); this file defines the
rules of the game.

## The daily 3 hours

| Block | Minutes | What |
|-------|---------|------|
| Recall | 20 | Blank page. Write what you remember from yesterday and from the last task of the previous stage. No notes. |
| Concept | 30 | Read the task's "Read first" list. Ask Claude for theory + example if stuck (`/start-task` gives it). |
| Build | 100 | Write the code. One compiler error at a time; read the full message before changing anything. |
| Self-review | 20 | `cargo fmt`, `cargo clippy`, delete dead code, re-read your diff as if someone else wrote it. |
| Journal | 10 | Fill `journal/<ID>.md`: what broke, what you learned, answers to the checkpoint questions. |

**Sunday**: no new tasks. 45-60 minutes of spaced repetition: re-implement one
earlier task's core function from a blank file, or explain one checkpoint out
loud. Use the rest to catch up if you slipped.

## The review gate

A task flips to ✅ only through `/task-review <ID>`, in this order:

1. **Automated gate** (`scripts/gate.sh <ID>`): `fmt`, `clippy -D warnings`,
   the task's fixed tests, and doc tests. Red = not ready; no penalty, no
   attempt counted.
2. **Journal present** at `journal/<ID>.md` with the checkpoint questions answered.
3. **Code review + rubric** (below). The reviewer reads your code, not just the
   test output.
4. **Oral check**: the reviewer asks the three checkpoint questions again,
   varied. You answer from your head.
5. **Verdict**: score ≥ 80 and no blockers → `pass`. Otherwise `fail` with a
   written list of what to fix; the task goes 🔁 retry and later due dates
   slip one study day.

### Rubric (100 points)

| Area | Pts | Looking for |
|------|-----|-------------|
| Correctness | 30 | Gate green, edge cases handled, invariants hold on failure paths |
| Idiom | 20 | Ownership used deliberately, no gratuitous `clone`, iterators/combinators where natural, clippy clean |
| Design & API | 15 | Types make invalid states hard, errors are typed and informative, small public surface |
| Tests | 15 | Your own tests assert invariants and failure cases, not only the happy path |
| Explanation | 20 | Journal answers and oral answers are correct, in your own words, and explain *why* |

### Blockers (any one fails the review regardless of score)

- `unwrap`/`expect`/indexing that can panic in non-test library code (unless an
  invariant is documented right next to it).
- `.clone()`, `Rc`, `unsafe` or `'static` added only "to make the compiler stop
  complaining", with no explanation of why it is correct.
- Tests edited, ignored or special-cased to go green.
- Code you cannot explain line by line.

## Integrity rules

- The point is your understanding, not a green checkmark. Using Claude for
  *explanations, hints and review* is the intended use; having it write the
  solution defeats the purpose and the oral check will expose it.
- Hint ladder: (1) a question, (2) a pointer to the concept, (3) a snippet
  that shows the idea on a different example, (4) a solution, which costs you
  the "Explanation" points for that part until you re-derive it.
- Stage gates are closed book: no notes, no AI.

## Slip policy

- Late finish or failed review shifts every later unfinished row by the same
  number of study days (handled by `scripts/roadmap.py`).
- Slip is visible at the top of the roadmap. Three consecutive late tasks
  means: shrink the stretch goals, never the 3 hours or the gate.
- Stretch goals are optional and never affect the score.

## Stage gates

The last row of each stage is `S<n>-GATE`: a 60-90 minute closed-book exam
(spec at the end of each stage file) followed by catch-up on weak spots. Same
verdict mechanism: `/task-review S<n>-GATE`.

## Task scaffolding

Stage 1 is fully scaffolded (stubs in `tasks/src/s01/`, fixed tests in
`tasks/tests/`). Later stages are scaffolded just in time with
`/start-task <ID>` so that the tests can reflect what you actually learned
and drafted in the journal, and so that nothing in the repo is a spoiler.
