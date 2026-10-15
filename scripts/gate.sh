#!/usr/bin/env bash
# Automated review gate for a RustyLab task.
#   scripts/gate.sh S1-T3
# Runs fmt, clippy and the task's fixed tests. Exit 0 = automated gate green.
# The roadmap is NOT updated here: only `/task-review` does that after the
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
  echo "Later-stage tasks are scaffolded when you reach them: ask for '/start-task ${id}'." >&2
  exit 3
fi

step() { printf '\n\033[1m▶ %s\033[0m\n' "$*"; }

step "cargo fmt --check (tasks)"
cargo fmt -p tasks -- --check

step "cargo clippy (tasks, deny warnings)"
cargo clippy -p tasks --all-targets -- -D warnings

step "cargo test ${test_target} (fixed gate tests)"
cargo test -p tasks --test "${test_target}"

mod=$(ls "tasks/src/s${stage}/" | grep -E "^t${task}_" | head -1 | sed 's/\.rs$//')
step "doc tests for ${mod}"
cargo test -p tasks --doc -- "${mod}"

printf '\n\033[32m✔ automated gate green for %s\033[0m\n' "$id"
echo "Next: write journal/${id}.md, then run /task-review ${id}"
