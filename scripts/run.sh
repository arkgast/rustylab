#!/usr/bin/env bash
# Run a task and see its output in the console.
#   scripts/run.sh S1-T1            your katas, with their results (⏳ = still todo!())
#   scripts/run.sh S1-T1 learn      the lesson for the task (written by /start-task)
#   scripts/run.sh S1-T5 -- repl    extra arguments go to the program after `--`
#   scripts/run.sh playground       your scratch file, tasks/examples/playground.rs
set -euo pipefail
cd "$(dirname "$0")/.."

id="${1:-}"
shift || true

if [[ "$id" == "playground" ]]; then
  exec cargo run -q -p tasks --example playground -- "$@"
fi
if [[ ! "$id" =~ ^S([0-9]+)-T([0-9]+)$ ]]; then
  echo "usage: scripts/run.sh S<stage>-T<task> [learn] [-- args]   |   scripts/run.sh playground" >&2
  exit 2
fi
example=$(printf "s%02d_t%02d" "${BASH_REMATCH[1]}" "${BASH_REMATCH[2]}")
if [[ "${1:-}" == "learn" ]]; then
  example="${example}_learn"
  shift
fi
[[ "${1:-}" == "--" ]] && shift

if [[ ! -f "tasks/examples/${example}.rs" ]]; then
  echo "tasks/examples/${example}.rs does not exist yet." >&2
  echo "Run '/start-task ${id}' in Claude Code: it scaffolds the task and writes the lesson." >&2
  exit 3
fi
exec cargo run -q -p tasks --example "$example" -- "$@"
