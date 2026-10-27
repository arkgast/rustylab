#!/usr/bin/env bash
# Automated review gate for a RustyLab task.
#   scripts/gate.sh S1-T3
# Runs fmt, clippy, the task's fixed tests, its own unit tests and doctests,
# and checks that the runner (`scripts/run.sh <ID>`) builds. Exit 0 = green.
# The roadmap is NOT updated here: only `/review` does that after the
# human-style review (rubric + checkpoint questions).
set -euo pipefail
cd "$(dirname "$0")/.."

id="${1:-}"
if [[ ! "$id" =~ ^S([0-9]+)-T([0-9]+)$ ]]; then
  echo "usage: scripts/gate.sh S<stage>-T<task>   e.g. S1-T3" >&2
  exit 2
fi
stage=$(printf "%02d" "${BASH_REMATCH[1]}")
task=$(printf "%02d" "${BASH_REMATCH[2]}")
test_target="s${stage}_t${task}"

if [[ ! -f "tasks/tests/${test_target}.rs" ]]; then
  echo "No gate tests for ${id} yet (tasks/tests/${test_target}.rs missing)." >&2
  echo "Later-stage tasks are scaffolded when you reach them: ask for '/study ${id}'." >&2
  exit 3
fi

step() { printf '\n\033[1m▶ %s\033[0m\n' "$*"; }

step "cargo fmt --check (tasks)"
cargo fmt -p tasks -- --check

step "cargo clippy (tasks, deny warnings)"
cargo clippy -p tasks --all-targets -- -D warnings

step "cargo test ${test_target} (fixed gate tests)"
cargo test -p tasks --test "${test_target}"

mod=$(find "tasks/src/s${stage}" -maxdepth 1 -name "t${task}_*.rs" -exec basename {} .rs \; | head -1)
step "your own unit tests in ${mod}"
cargo test -p tasks --lib -- "s${stage}::${mod}"

step "doc tests for ${mod}"
cargo test -p tasks --doc -- "${mod}"

step "runner builds (scripts/run.sh ${id})"
cargo build -q -p tasks --example "${test_target}"

printf '\n\033[32m✔ automated gate green for %s\033[0m\n' "$id"
echo "Next: write journal/${id}.md, then run /review ${id}"
