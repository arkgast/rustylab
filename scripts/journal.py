#!/usr/bin/env python3
"""Write a task's journal one step at a time: `cargo x journal S1-T1`.

Creates journal/<ID>.md from the template with the task's title and real
checkpoint questions, then walks every unanswered section. Each answer is saved
at once, so you can stop anytime and rerun to continue. Sections stay plain
Markdown: editing the file by hand works too.
"""
import datetime as dt
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
HEADING = re.compile(r"^#{2,3} ")


def spec(task_id):
    """(title, checkpoint questions) from the task's section in curriculum/."""
    for f in sorted((ROOT / "curriculum").glob("stage-*.md")):
        m = re.search(rf"^## {task_id} · (.+?)\n(.*?)(?=^## |\Z)", f.read_text(), re.M | re.S)
        if m:
            after = m.group(2).partition("**Checkpoint questions**")[2]
            return m.group(1).strip(), re.findall(r"^\d+\. (.+)$", after, re.M)
    sys.exit(f"{task_id} is not in curriculum/; expected an ID like S1-T1 or S1-GATE")


def create(path, task_id):
    title, questions = spec(task_id)
    text = (ROOT / "journal" / "TEMPLATE.md").read_text()
    text = text.replace("<TASK-ID> · <Title>", f"{task_id} · {title}")
    text = text.replace("- Date started:", f"- Date started: {dt.date.today().isoformat()}")
    answers = "\n\n".join(f"### {q}\n(Answer in your own words; explain why.)" for q in questions)
    text = text.replace("1.\n2.\n3.", answers or "(Notes from the exam.)")
    path.write_text(text)


def sections(lines):
    """Line ranges (heading, first body line, end) of every leaf section."""
    heads = [i for i, line in enumerate(lines) if HEADING.match(line)]
    out = []
    for i, end in zip(heads, heads[1:] + [len(lines)]):
        if end - i > 1 or i == heads[-1]:
            out.append((i, i + 1, end))
    return [s for s in out if not (s[1] < s[2] and all(not lines[j].strip() for j in range(s[1], s[2])) and s[2] < len(lines) and lines[s[2]].startswith("### "))]


def unanswered(lines, sec):
    body = "\n".join(lines[sec[1]:sec[2]]).strip()
    return not body or (body.startswith("(") and body.endswith(")"))


def main(argv):
    if len(argv) != 1 or not re.fullmatch(r"S\d+-(T\d+|GATE)", argv[0]):
        sys.exit("usage: cargo x journal S<stage>-T<task>   e.g. S1-T3")
    task_id = argv[0]
    path = ROOT / "journal" / f"{task_id}.md"
    if not path.exists() or "<TASK-ID>" in path.read_text():
        create(path, task_id)
    if not sys.stdin.isatty():
        print(f"{path.relative_to(ROOT)} is ready; run this in a terminal for the guided steps.")
        return
    print("Blank line finishes an answer. On an empty answer: Enter skips, q stops. Saved as you go.")
    # Index walks forward; each save may change line counts, so sections are recomputed.
    done = 0
    while True:
        lines = path.read_text().split("\n")
        todo = [s for s in sections(lines) if unanswered(lines, s)]
        if done >= len(todo):
            break
        sec = todo[done]
        print(f"\n{lines[sec[0]].lstrip('# ')}  [{len(todo) - done} left]")
        hint = "\n".join(lines[sec[1]:sec[2]]).strip()
        if hint:
            print(hint)
        answer = []
        while True:
            try:
                line = input("> " if not answer else "  ")
            except EOFError:
                line = "q" if not answer else ""
            if not line.strip():
                break
            if line.strip() == "q" and not answer:
                print(f"Saved. Continue with: cargo x journal {task_id}")
                return
            answer.append(line)
        if not answer:
            done += 1
            continue
        lines[sec[1]:sec[2]] = [*answer, ""]
        path.write_text("\n".join(lines))
    print(f"\n{path.relative_to(ROOT)}: nothing left to answer" if done == 0
          else f"\n{path.relative_to(ROOT)}: {done} section(s) skipped; rerun to answer them.")


if __name__ == "__main__":
    main(sys.argv[1:])
