#!/usr/bin/env bash
# Session clock, run by the /study tutor at natural seams (after recall, after
# the probes, when the learner reports a gate or run result, before close).
# While a /study session is open today it prints one line:
#   [study] 47 min since open, breaks: 0
# and prints nothing otherwise. The tutor gives the single break cue when it
# reports 85 min or more and breaks: 0. Always exits 0.
# bash 3.2 (macOS) compatible; one `date` call. CLAUDE_PROJECT_DIR, when set,
# points it at another project root (used to test it on a scratch session).
f="${CLAUDE_PROJECT_DIR:-$(cd "$(dirname "$0")/.." && pwd)}/.study/session.md"
[ -f "$f" ] || exit 0

opened="" closed="" breaks=0
while IFS= read -r line || [ -n "$line" ]; do
  value=${line#*:}
  value=${value%%#*}
  value=${value//[[:space:]]/}
  case "$line" in
    opened:*) opened=$value ;;
    closed:*) closed=$value ;;
    breaks:*) breaks=$value ;;
  esac
done <"$f"

set -- $(date '+%Y-%m-%d %H %M')
today=$1 hour=$2 minute=$3

case "$opened" in
  "$today"T[0-2][0-9]:[0-5][0-9]) ;;
  *) exit 0 ;;
esac
[ -z "$closed" ] || [ "$closed" = none ] || exit 0

time=${opened#*T}
mins=$(( (10#$hour * 60 + 10#$minute) - (10#${time%%:*} * 60 + 10#${time#*:}) ))
[ "$mins" -ge 0 ] || mins=0
echo "[study] $mins min since open, breaks: $breaks"
