//! `cargo x <command>`: one entry point for the repo's tools (the xtask pattern).
//! A thin dispatcher on purpose: the logic stays in `scripts/`.
use std::path::Path;
use std::process::{Command, ExitCode};

const HELP: &str = "\
RustyLab commands

  cargo x status                what is due next, how late
  cargo x plan                  your dates and intensity (.study/plan.json)
  cargo x init [YYYY-MM-DD]     build ROADMAP.md and the plan (before any progress)
  cargo x init --force [DATE]   rebuild anyway, discarding all statuses and scores
  cargo x probe [--release]     rerun the tutor's snippet
  cargo x play [--release]      run your playground
  cargo x run S1-T1 [-- args]   run your katas (cargo x S1-T1 also works)
  cargo x gate S1-T1            fmt, clippy, tests, runner build
  cargo x journal S1-T1         write the journal, one step at a time
  cargo x after 3               the date 3 study days from today";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (cmd, rest) = match args.split_first() {
        Some((cmd, rest)) => (cmd.as_str(), rest),
        None => ("help", &[][..]),
    };
    let (program, mut full): (&str, Vec<String>) = match cmd {
        "run" => ("scripts/run.sh", vec![]),
        "probe" => ("scripts/run.sh", vec!["probe".into()]),
        "play" | "playground" => ("scripts/run.sh", vec!["playground".into()]),
        "gate" => ("scripts/gate.sh", vec![]),
        "status" | "plan" | "init" | "after" => {
            ("python3", vec!["scripts/roadmap.py".into(), cmd.into()])
        }
        "journal" => ("python3", vec!["scripts/journal.py".into()]),
        "help" | "-h" | "--help" => {
            println!("{HELP}");
            return ExitCode::SUCCESS;
        }
        id if id.starts_with('S') => ("scripts/run.sh", vec![id.into()]),
        other => {
            eprintln!("unknown command `{other}`\n\n{HELP}");
            return ExitCode::from(2);
        }
    };
    full.extend_from_slice(rest);

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("..");
    match Command::new(program).args(&full).current_dir(root).status() {
        Ok(status) if status.success() => ExitCode::SUCCESS,
        Ok(_) => ExitCode::FAILURE,
        Err(err) => {
            eprintln!("could not start {program}: {err}");
            ExitCode::FAILURE
        }
    }
}
