//! RustyLab guided exercises. One module per stage, one file per task.
//!
//! Workflow: read `curriculum/stage-XX-*.md`, run the lesson
//! (`scripts/run.sh <ID> learn`), replace every `todo!()` in the task file,
//! watch the results with `scripts/run.sh <ID>`, then make the gate green with
//! `scripts/gate.sh <ID>`. The tests in `tasks/tests/` are fixed: do not edit
//! them.

pub mod lab;
pub mod s01;
