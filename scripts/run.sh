#!/usr/bin/env bash
# Run a task and see its output in the console.
#   scripts/run.sh S1-T1            your katas, with their results (⏳ = still todo!())
#   scripts/run.sh S1-T5 -- repl    extra arguments go to the program after `--`
#   scripts/run.sh probe            the tutor's current snippet, tasks/examples/probe.rs (written by /study)
#   scripts/run.sh playground       your scratch file, tasks/examples/playground.rs
#   scripts/run.sh probe --release  probe or playground with the release profile
set -euo pipefail
cd "$(dirname "$0")/.."

usage="usage: scripts/run.sh S<stage>-T<task> [-- args]   |   scripts/run.sh probe|playground [--release] [-- args]"
id="${1:-}"
shift || true

if [[ "$id" == "probe" || "$id" == "playground" ]]; then
  release=""
  if [[ "${1:-}" == "--release" ]]; then
    release="--release"
    shift
  fi
  [[ "${1:-}" == "--" ]] && shift
  # $release is unquoted on purpose: empty means no flag (bash 3.2 has no safe empty arrays under set -u).
  exec cargo run -q -p tasks $release --example "$id" -- "$@"
fi
if [[ ! "$id" =~ ^S([0-9]+)-T([0-9]+)$ ]]; then
  echo "$usage" >&2
  exit 2
fi
example=$(printf "s%02d_t%02d" "${BASH_REMATCH[1]}" "${BASH_REMATCH[2]}")
if [[ $# -gt 0 && "$1" != "--" ]]; then
  echo "$usage" >&2
  exit 2
fi
[[ "${1:-}" == "--" ]] && shift

if [[ ! -f "tasks/examples/${example}.rs" ]]; then
  echo "tasks/examples/${example}.rs does not exist yet." >&2
  echo "Run '/study ${id}' in Claude Code: it scaffolds the task." >&2
  exit 3
fi
exec cargo run -q -p tasks --example "$example" -- "$@"
