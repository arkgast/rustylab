# How the curriculum works

83 tasks in 17 stages and 5 phases, plus one closed-book gate per stage. 3 hours
a day, Monday to Saturday. Dates live in [`../ROADMAP.md`](../ROADMAP.md); this
file defines the rules of the game.

## The arc

| Phase | Stages | You become someone who... |
|-------|--------|---------------------------|
| I · Fluency | 1 Rust refresh · 2 Collections, closures & iterators · 3 Error handling · 4 Traits & generics | writes everyday Rust from a blank file without looking things up |
| II · Depth | 5 Lifetimes & memory · 6 Crates, API design & testing · 7 Concurrency · 8 Async · 9 Macros | writes Rust a senior reviewer would merge: sound APIs, tests that prove invariants, no data races |
| III · Systems | 10 Unsafe, no_std & WASM · 11 Encoding & cryptography | works below the standard library: memory, bytes, signatures, sandboxes |
| IV · Protocol | 12 A runtime from scratch · 13 Execution models & security | builds a deterministic state machine and attacks it |
| V · Chains | 14 Polkadot SDK · 15 Solana · 16 Stellar & Soroban · 17 Senior capstone | ships on real chains and defends the design |

Why this order: each phase uses only what the previous ones taught. Iterators
come right after the basics because idiomatic Rust is written with them.
Errors come before traits because "no panics" must be a habit from week 2.
The runtime (Stage 12) is built before FRAME, Anchor and Soroban so that those
frameworks read as "the thing I built, with production polish", not magic.

## The learning loop for every task

1. **Recall and probes.** `/study`: 2-3 recall questions, then 3-5 probes:
   predict, run, explain the gap. A probe is a runnable snippet of at most 12
   lines; you commit to a prediction, the tutor runs it
   (`tasks/examples/probe.rs`; rerun it yourself with `scripts/run.sh probe`),
   and the output settles it. Idiom probes show the **idiomatic version next
   to the naive one**. Read the task's Reference after the probes, or when
   stuck.
2. **Katas.** Replace the `todo!()`s in `tasks/src/sNN/tMM_*.rs`.
3. **See it.** `scripts/run.sh <ID>` prints every kata's result (⏳ = not done
   yet). Predict the output before you press enter. `scripts/run.sh playground`
   is your scratch space.
4. **Gate.** `scripts/gate.sh <ID>`: fmt, clippy `-D warnings`, the fixed tests,
   your own tests, doctests, and a check that the runner builds.
5. **Journal.** `journal/<ID>.md` (template: `journal/TEMPLATE.md`). The close
   message lists the day's misses as the seed for "What surprised me".
6. **Review.** `/review <ID>`: rubric and oral questions, including what you
   missed in the probes. Only a passing review moves the roadmap.

Phase rules:

- **Phase I** is about fluency. You never have to write a failing example or
  name a compiler error code. Probes may ask why the compiler rejects a
  line; you are never asked for the error code.
- **From Phase II on**, `compile_fail` doctests with error codes become a
  normal tool (introduced in S5-T1), and design-heavy capstones get an ADR in
  `docs/adr/`.
- **Chain stages (V)** live in their own crates under `labs/`, each with its own
  toolchain. Versions are resolved when the task is scaffolded, never in
  advance.

## The daily 3 hours

Two focus blocks of about 85 minutes with a 10-minute break between them; the
second block ends with gate, journal and close. Minutes are targets, not
timers: the tutor never announces the schedule, it only cues the break.

| Block | Minutes | Who leads | What |
|-------|---------|-----------|------|
| Recall | 15 | tutor | 2-3 questions from `.study/queue.md` (yesterday's checkpoints first, then older due items), answered from memory with no files open. Then predict the output of one kata from yesterday's runner; the tutor runs it. |
| Probes | 25-35 | tutor | 3-5 probes: predict, run, explain the gap. Skipped on day 2+ of a task unless probes remain. Read the task's Reference afterwards. |
| Build | 90-100 | you | Katas. One compiler error at a time; read the whole message before you change anything. `scripts/run.sh <ID>` often. The tutor is silent until asked; `hint` climbs the ladder one rung. |
| Break | 10 | tutor, which checks `scripts/clock.sh` at natural seams | At ~90 minutes, one line. Ten minutes away from the screen. |
| Self-review | 15 | you | `scripts/gate.sh <ID>`, delete dead code, re-read your diff as if a stranger wrote it. The tutor reviews only if asked, and only points. |
| Journal | 10 | you | `journal/<ID>.md`. The close message lists the day's misses as the seed for "What surprised me". |
| Close | 2 | tutor | `/study close` (or "done for today"). Then nothing demanding for 10-20 minutes. |

**Day 2 of a two-day task**: recall uses the probes you missed on day 1; no
new probes unless some remain; straight to build.

**Gate day**: `/study` only says "Gate day. Closed book. `/review S<n>-GATE`
when ready."; the review runs the exam.

**Sunday**: no new tasks. 45-60 minutes with `/study`: every due question in
the recall queue, then one earlier kata redone from a blank playground (the
tutor picks the one you recall worst) and checked against the fixed tests'
expectations. Use the rest to catch up if you slipped.

**The recall queue** (`.study/queue.md`): every probe you miss, every hint
from rung 3 up, and every checkpoint of a passed review comes back after 1
study day, then 3, 9 and 27. A miss sends it back to 1; a correct answer at
27 retires it.

**The break cue**: nothing runs on a timer. The tutor runs `scripts/clock.sh`
at natural seams (after recall, after the probes, when you report a run or
gate result, before close); it prints `[study] N min since open, breaks: B`.
At 85 minutes or more with no break logged, the tutor gives the one-line cue.

## The review gate

A task flips to ✅ only through `/review <ID>`, in this order:

1. **Automated gate** (`scripts/gate.sh <ID>`). Red means not ready; there is no
   penalty and the attempt does not count.
2. **Journal present** with the checkpoint questions answered.
3. **Code review and rubric.** The reviewer reads your code and runs your
   runner, not just the tests.
4. **Oral check.** The checkpoint questions again, varied, answered from your
   head.
5. **Verdict.** Score ≥ 80 and no blockers → `pass`. Otherwise `fail` with a
   written fix list; the task goes 🔁 and later dates slip one study day.

### Rubric (100 points)

| Area | Pts | Looking for |
|------|-----|-------------|
| Correctness | 30 | Gate green, edge cases handled, invariants hold on failure paths |
| Idiom | 20 | Ownership used deliberately, no gratuitous `clone`, iterators where natural, clippy clean |
| Design & API | 15 | Types make invalid states hard, errors are typed and informative, small public surface |
| Tests | 15 | Your own tests assert invariants and failure cases, not only the happy path (Phase I: only where the task asks for them) |
| Explanation | 20 | Journal and oral answers are correct, in your own words, and explain *why* |

### Blockers (any one fails the review regardless of score)

- `unwrap`/`expect`/indexing that can panic in non-test library code, unless
  an invariant is documented right next to it (from S1-T5 on).
- `.clone()`, `Rc`, `unsafe` or `'static` added only to make the compiler
  stop complaining, with no explanation of why it is correct.
- Tests edited, ignored or special-cased to go green.
- Code you cannot explain line by line.

## Integrity rules

- The point is your understanding, not a green checkmark. Use Claude for
  explanations, hints and review. If it writes your solution, you lose the
  learning, and the oral check will show it.
- Hint ladder (`hint` in chat, one rung per message): (1) a question, (2) the
  concept and where it is in the Reference, (3) the same idea on a different
  problem, run as a probe, (4) the solution (`show me`), which costs the
  Explanation points for that part until you re-derive it at review. Every
  hint is logged in `.study/log/<ID>.md`, and the reviewer reads it.
- Stage gates are closed book: no notes, no AI.

## Slip policy

- A late finish or a failed review shifts every later unfinished row by the
  same number of study days (`scripts/roadmap.py` does this).
- Three late tasks in a row: drop stretch goals, never the 3 hours or the gate.
- Stretch goals are optional and never affect the score.

## Scaffolding

Stage 1 is fully scaffolded: stubs in `tasks/src/s01/`, runners in
`tasks/examples/`, fixed tests in `tasks/tests/`. Every later task is
scaffolded just in time by `/study <ID>`. That way the tests match what
you have actually learned, dependency versions are current, and nothing in
the repo spoils a solution.
