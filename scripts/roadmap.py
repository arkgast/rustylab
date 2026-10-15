#!/usr/bin/env python3
"""Update ROADMAP.md deterministically.

Usage:
  scripts/roadmap.py status                 # progress + what is due next
  scripts/roadmap.py start  <ID>            # mark 🔨 doing
  scripts/roadmap.py pass   <ID> <SCORE>    # review passed: ✅, record date + score
  scripts/roadmap.py fail   <ID> <SCORE>    # review failed: 🔁, +1 study day slip

Options (before the command): --today YYYY-MM-DD   (for testing), --file PATH

Study days are Monday..Saturday. Finishing late (or failing a review) shifts the
due date of every later unfinished row by the same number of study days.
"""
import datetime as dt
import pathlib
import re
import sys

ROW = re.compile(
    r"^\| (S\d+-(?:T\d+|GATE)) \| (.*?) \| (\d+) \| (\d{4}-\d{2}-\d{2}) \| (.*?) \| (.*?) \| (.*?) \|$"
)
TODO, DOING, RETRY, DONE = "⬜ todo", "🔨 doing", "🔁 retry", "✅ done"
PASS_SCORE = 80


def is_study_day(d):
    return d.weekday() != 6


def add_study_days(d, n):
    while n > 0:
        d += dt.timedelta(days=1)
        if is_study_day(d):
            n -= 1
    return d


def study_days_between(a, b):
    """Study days in (a, b]; 0 if b <= a."""
    n, d = 0, a
    while d < b:
        d += dt.timedelta(days=1)
        if is_study_day(d):
            n += 1
    return n


def parse(path):
    lines = path.read_text().split("\n")
    rows = {}
    for i, line in enumerate(lines):
        m = ROW.match(line)
        if m:
            rid, title, days, due, status, done, score = m.groups()
            rows[i] = dict(id=rid, title=title, days=days, due=dt.date.fromisoformat(due),
                           status=status, done=done, score=score)
    return lines, rows


def render(r):
    return (f"| {r['id']} | {r['title']} | {r['days']} | {r['due'].isoformat()} | "
            f"{r['status']} | {r['done']} | {r['score']} |")


def refresh_progress(lines, rows, slip_add):
    tasks = [r for r in rows.values() if not r["id"].endswith("GATE")]
    gates = [r for r in rows.values() if r["id"].endswith("GATE")]
    done_t = sum(r["status"] == DONE for r in tasks)
    done_g = sum(r["status"] == DONE for r in gates)
    finish = max(r["due"] for r in rows.values())
    s = next(i for i, l in enumerate(lines) if "progress:start" in l)
    e = next(i for i, l in enumerate(lines) if "progress:end" in l)
    old = "\n".join(lines[s:e])
    m = re.search(r"Slip: \*\*(\d+)\*\*", old)
    slip = int(m.group(1)) + slip_add if m else slip_add
    started = re.search(r"Started: \*\*(\d{4}-\d{2}-\d{2})\*\*", old).group(1)
    planned_days = re.search(r"Study days planned: \*\*(\d+)\*\*", old).group(1)
    block = [
        lines[s],
        f"- Tasks done: **{done_t} / {len(tasks)}** · Stage gates passed: **{done_g} / {len(gates)}**",
        f"- Study days planned: **{planned_days}** (3 h/day = {int(planned_days) * 3} h)",
        f"- Started: **{started}** · Planned finish: **{finish.isoformat()}**",
        f"- Slip: **{slip}** study days",
    ]
    lines[s:e] = block


def shift_later(rows, after_idx, n):
    if n <= 0:
        return
    for i, r in rows.items():
        if i > after_idx and r["status"] != DONE:
            r["due"] = add_study_days(r["due"], n)


def save(path, lines, rows, slip_add):
    for i, r in rows.items():
        lines[i] = render(r)
    refresh_progress(lines, rows, slip_add)
    path.write_text("\n".join(lines))


def main(argv):
    today, path = dt.date.today(), pathlib.Path(__file__).resolve().parent.parent / "ROADMAP.md"
    while argv and argv[0].startswith("--"):
        if argv[0] == "--today":
            today = dt.date.fromisoformat(argv[1])
        elif argv[0] == "--file":
            path = pathlib.Path(argv[1])
        else:
            sys.exit(f"unknown option {argv[0]}")
        argv = argv[2:]
    if not argv:
        sys.exit(__doc__)
    cmd, args = argv[0], argv[1:]
    lines, rows = parse(path)
    by_id = {r["id"]: i for i, r in rows.items()}

    if cmd == "status":
        pending = [r for r in rows.values() if r["status"] != DONE]
        done = len(rows) - len(pending)
        print(f"{done}/{len(rows)} rows done. Today: {today.isoformat()}")
        for r in pending[:5]:
            late = study_days_between(r["due"], today)
            flag = f"  ⚠ {late} study day(s) late" if late else ""
            print(f"  {r['id']:<9} {r['status']:<9} due {r['due']}  {r['title']}{flag}")
        return

    if not args or args[0] not in by_id:
        sys.exit(f"unknown task id {args[:1]}; try one of {', '.join(list(by_id)[:6])}...")
    idx = by_id[args[0]]
    r = rows[idx]

    if cmd == "start":
        if r["status"] == DONE:
            sys.exit(f"{r['id']} is already done")
        earlier = [x for i, x in rows.items() if i < idx and x["status"] != DONE]
        if earlier:
            sys.exit(f"finish {earlier[0]['id']} first (tasks are sequential)")
        r["status"] = DOING
        save(path, lines, rows, 0)
        print(f"{r['id']} → {DOING}")
    elif cmd in ("pass", "fail"):
        if len(args) < 2:
            sys.exit("score required (0-100)")
        score = int(args[1])
        if cmd == "pass" and score < PASS_SCORE:
            sys.exit(f"score {score} < {PASS_SCORE}: use `fail`")
        if cmd == "pass":
            r.update(status=DONE, done=today.isoformat(), score=str(score))
            new_due = max(r["due"], today)
            while not is_study_day(new_due):
                new_due += dt.timedelta(days=1)
        else:
            r.update(status=RETRY, score=str(score))
            new_due = add_study_days(max(r["due"], today), 1)
        slip = study_days_between(r["due"], new_due)
        r["due"] = new_due if cmd == "fail" else r["due"]
        shift_later(rows, idx, slip)
        save(path, lines, rows, slip)
        print(f"{r['id']} → {r['status']} (score {score}); slip +{slip} study day(s)")
    else:
        sys.exit(f"unknown command {cmd}")


if __name__ == "__main__":
    main(sys.argv[1:])
