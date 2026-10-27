//! Your scratch space: `scripts/run.sh playground`.
//!
//! Paste snippets from probes, break them, read the compiler errors, try
//! ideas. Nothing here is reviewed or graded.

fn main() {
    let words = ["ownership", "borrowing", "lifetimes"];
    let lengths: Vec<usize> = words.iter().map(|w| w.len()).collect();
    println!("{words:?} -> {lengths:?}");
}
