# Stage 10: Unsafe, no_std & WebAssembly

**Phase:** III · Systems
**Outcome:** You can write a small `unsafe` abstraction with a stated safety contract and prove it clean under Miri, call C through FFI, build a `no_std + alloc` library, and run a Rust WASM guest inside a metered wasmtime host: the exact toolchain under Substrate runtimes, Solana programs and Soroban contracts.

Each task: `/study` (recall, then probes: predict, run, explain the gap), solve the katas, run them and read the output, make the gate green, write the journal, then `/review <ID>`.

## S10-T1 · Unsafe Rust, invariants & Miri
*2 days (6 h)*

**Why it matters.** `unsafe` does not turn off the borrow checker; it unlocks five extra operations and moves the proof obligation from the compiler to you. Every runtime host, allocator and codec fast path contains some. Knowing how to fence it behind a safe API is what separates a senior engineer from someone who copies `transmute` from Stack Overflow.

**Probes**
- The five unsafe superpowers; soundness vs safety; why a safe fn that can cause UB is a bug even if nobody calls it wrong
- `// SAFETY:` comments and `# Safety` doc sections: invariant, precondition, who upholds it
- Naive version (`Vec` + indices with `unwrap`) vs raw-pointer version side by side; Miri catching a dangling read and an aliasing violation live

**Reference** (read after the probes, or when stuck)
- The Rustonomicon: "Meet Safe and Unsafe", "How Safe and Unsafe Interact", "Working with Unsafe", "Aliasing"
- The Rust Book ch. 20.1 (Unsafe Rust)
- Miri README (github.com/rust-lang/miri): installation, `cargo miri test`, Stacked/Tree Borrows
- std docs: `std::ptr` module (`read`, `write`, `NonNull`, `addr_of!`), `MaybeUninit`, `slice::from_raw_parts`

**Build.** `tasks/src/s10/t01_unsafe.rs`: a fixed-capacity `RingBuffer<T, const N: usize>` over `[MaybeUninit<T>; N]` with `push`, `pop`, `peek`, `len` and a correct `Drop`; `split_at_mut_raw(&mut [T], usize) -> Option<(&mut [T], &mut [T])>` re-implemented with raw pointers; `unsafe fn read_u32_le(ptr: *const u8) -> u32` with a documented contract and a safe wrapper `read_u32_le_checked(&[u8]) -> Option<u32>`.

**Run.** `scripts/run.sh S10-T1`, then `cargo +nightly miri test -p tasks --test s10_t01`.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s10_t01.rs` pass under plain `cargo test` and under `cargo miri test`
- [ ] Every `unsafe` block has a `// SAFETY:` comment naming the invariant it relies on
- [ ] Every `unsafe fn` has a `# Safety` doc section
- [ ] `RingBuffer` drops exactly the initialized elements (a test counts drops)
- [ ] No public safe function can cause UB for any input

**Stretch.** Write a deliberately unsound variant behind `#[cfg(test)]`, show Miri's report in the journal, then delete it.

**Checkpoint questions** (answer in `journal/S10-T1.md`; the reviewer asks them again)
1. What is the difference between an `unsafe fn` and a safe fn containing an `unsafe` block, in terms of who must prove what?
2. Why must `RingBuffer` use `MaybeUninit` instead of `[T; N]` with `Default`, and what goes wrong in `Drop` if you get it wrong?
3. Miri passed. What does that prove, and what does it not prove?

## S10-T2 · FFI & repr(C)
*1 day (3 h)*

**Why it matters.** Crypto libraries, databases (RocksDB under Substrate) and VMs cross the C ABI. FFI is where layout, ownership and null-termination stop being guaranteed by Rust.

**Probes**
- `extern "C"`, `unsafe extern` blocks (edition 2024), linking a C file with a `build.rs` and `cc`
- `repr(Rust)` vs `repr(C)`: field order, padding, `size_of`/`align_of` printed side by side
- `CString` vs `&str`: interior NULs, who frees the memory, never `free` a Rust pointer from C

**Reference** (read after the probes, or when stuck)
- The Rustonomicon: "FFI", "Alternative representations" (`repr(C)`, `repr(transparent)`)
- `cc` crate docs (docs.rs/cc); std docs: `std::ffi::CString`, `CStr`, `c_char`

**Build.** `labs/ffi-checksum/`: a crate with `c/checksum.c` (`uint32_t adler32(const uint8_t*, size_t)` and `int greet(const char*, char* out, size_t out_len)`), `build.rs` with `cc`, a `#[repr(C)] struct Header { version: u8, len: u32, flags: u16 }`, and safe Rust wrappers `adler32(&[u8]) -> u32` and `greet(&str) -> Result<String, FfiError>`.

**Run.** `cargo run -p ffi-checksum` prints the checksum of a test vector, the greeting and `size_of::<Header>()` for `repr(C)` vs `repr(Rust)`.

**Acceptance criteria** (all required)
- [ ] `adler32` matches the pure-Rust `adler` crate on 5 inputs including empty
- [ ] `greet` returns a typed error for input with an interior NUL and for a too-small buffer
- [ ] A test asserts the `repr(C)` size and field offsets (`std::mem::offset_of!`)
- [ ] Every FFI call is wrapped in a safe fn with a `// SAFETY:` comment

**Stretch.** Expose a Rust function to C with `#[unsafe(no_mangle)] pub extern "C"` and call it from the C side.

**Checkpoint questions** (answer in `journal/S10-T2.md`; the reviewer asks them again)
1. Why is the size of `Header` different under `repr(C)` and `repr(Rust)`?
2. Who owns the buffer `greet` writes into, and what would happen if C kept the pointer after returning?
3. Why does `CString::new` return a `Result`?

## S10-T3 · no_std + alloc
*2 days (6 h)*

**Why it matters.** Substrate runtimes compile to WASM with `no_std`, Solana programs build for SBF without most of std, and Soroban contracts are `no_std`. The pattern is always the same: a core crate on `core` + `alloc` with a `std` feature for tests and tooling.

**Probes**
- `#![no_std]`, `extern crate alloc`, `#![cfg_attr(not(feature = "std"), no_std)]`
- `alloc::collections::BTreeMap` replaces `HashMap`; `core::fmt` replaces `std::io`; errors without `std::error::Error`
- Proving it: `cargo build --target wasm32-unknown-unknown --no-default-features` and `cargo tree -e features`

**Reference** (read after the probes, or when stuck)
- The Embedded Rust Book: "no_std" chapter
- `core` and `alloc` crate docs (what is missing: `HashMap`, `std::io`, threads, time)
- Polkadot SDK docs: "Build a deterministic runtime" / why runtimes are `no_std`

**Build.** `labs/ledger-core/`: a `no_std + alloc` crate with `Ledger { balances: BTreeMap<[u8; 32], u128> }`, `transfer`, `mint`, `total_issuance`, a `LedgerError` implementing `core::fmt::Display` (and `std::error::Error` only behind `feature = "std"`), and a `std`-gated `Ledger::dump_json(&self) -> String`.

**Run.** `cargo test -p ledger-core` and `cargo build -p ledger-core --target wasm32-unknown-unknown --no-default-features`.

**Acceptance criteria** (all required)
- [ ] Builds for `wasm32-unknown-unknown` with default features off
- [ ] No dependency pulls in `std` when the feature is off (show `cargo tree -e features` in the journal)
- [ ] Checked arithmetic; total issuance is invariant under `transfer` (test)
- [ ] `std` feature is additive: enabling it never changes behavior, only adds APIs

**Stretch.** Add a `#[global_allocator]`-free `no_std` binary for `thumbv7em-none-eabihf` that only links the crate (no run needed).

**Checkpoint questions** (answer in `journal/S10-T3.md`; the reviewer asks them again)
1. Why must Cargo features be additive, and what breaks in a workspace if one isn't?
2. Why is `HashMap` absent from `alloc`, and why is that a feature for a blockchain runtime?
3. What does `alloc` need from the environment that `core` does not?

## S10-T4 · WebAssembly: a wasmtime host and guest
*2 days (6 h)*

**Why it matters.** Substrate runtimes, ink! and Soroban contracts are WASM modules executed by a host that exposes storage and crypto as imported functions. Building both sides once makes every host/guest boundary in those chains legible.

**Probes**
- `wasm32-unknown-unknown`, `crate-type = ["cdylib"]`, `#[unsafe(no_mangle)] extern "C"` exports
- Passing bytes across the boundary: guest exports `alloc`, host writes into linear memory, `(ptr, len)` pairs
- Host functions via `Linker::func_wrap`; a trap vs a returned error code

**Reference** (read after the probes, or when stuck)
- Rust and WebAssembly book: "What is WebAssembly?", linear memory
- wasmtime docs: "Embedding in Rust", `Linker`, `Store`, `Memory`, `Func::wrap`
- WebAssembly spec overview: imports, exports, linear memory, traps

**Build.** `labs/wasm-host/` with two crates: `guest/` (`no_std`, exports `add(i32, i32) -> i32`, `alloc(len) -> ptr`, `hash_and_log(ptr, len)` which calls an imported `env.log(ptr, len)` and `env.blake2(ptr, len, out_ptr)`) and `host/` (loads the guest `.wasm`, provides the imports, reads guest memory with bounds checks, prints results).

**Run.** `cargo build -p guest --target wasm32-unknown-unknown --release && cargo run -p wasm-host`.

**Acceptance criteria** (all required)
- [ ] Host prints `add(2,3) = 5` and the logged message from the guest
- [ ] Every host read of guest memory is bounds-checked and returns a typed error, never panics
- [ ] A guest that passes an out-of-range pointer produces a trap or error in the host, not a host crash (test)
- [ ] The guest `.wasm` is under 50 KB in release (record the size)

**Stretch.** Re-run the guest in the browser with `wasm-bindgen` and compare what each host provides.

**Checkpoint questions** (answer in `journal/S10-T4.md`; the reviewer asks them again)
1. Why can't the host just take a `&str` from the guest, and what does `(ptr, len)` cost you in safety?
2. What is a trap, and why is it the right failure mode for a sandbox?
3. Why does the guest export its own `alloc` instead of the host allocating in guest memory?

## S10-T5 · Capstone: a metered WASM contract sandbox
*3 days (9 h)*

**Why it matters.** Every smart-contract platform must stop an infinite loop and charge for work. Soroban and ink! do it with metering inside a WASM host and storage exposed only through host functions. You will build that loop end to end.

**Probes**
- Fuel: deterministic instruction counting vs wall-clock timeouts (and why chains reject the latter)
- Host storage API: `storage_get`, `storage_set`, gas charged per byte in the host function
- Transactional execution: buffer writes, commit only if the call returns success

**Reference** (read after the probes, or when stuck)
- wasmtime docs: `Config::consume_fuel`, `Store::set_fuel`, `Store::get_fuel`, `ResourceLimiter`
- Soroban docs: "Contract lifecycle", "Fees and metering"; ink! docs: "Storage" overview
- Polkadot SDK docs: weights and why execution must be bounded

**Build.** `labs/wasm-sandbox/`: host `Sandbox { storage: BTreeMap<Vec<u8>, Vec<u8>> }` with `call(wasm, entry, input, fuel_limit) -> Result<CallReceipt, SandboxError>`; host functions `storage_get`, `storage_set`, `input_len`, `read_input`, `set_return`; a `counter` guest contract (`increment`, `get`) and a `looper` guest that never terminates. Write an ADR in `docs/adr/` on fuel vs timeout and write buffering.

**Run.** `cargo run -p wasm-sandbox` runs: increment ×3, get, looper (out of fuel), a failing call that tries to write then traps.

**Acceptance criteria** (all required)
- [ ] `looper` stops with `SandboxError::OutOfFuel`, deterministically, at the same fuel point every run
- [ ] A trapped or out-of-fuel call leaves storage byte-for-byte unchanged (test)
- [ ] Storage host functions charge fuel proportional to bytes and reject oversize values with an error
- [ ] `CallReceipt` reports fuel used; same input gives same fuel used across 10 runs (test)
- [ ] ADR written; no `unwrap`/`expect` in host code

**Stretch.** Add a memory limit with `ResourceLimiter` and a guest that tries to grow memory to 1 GB.

**Checkpoint questions** (answer in `journal/S10-T5.md`; the reviewer asks them again)
1. Why do blockchains meter by instructions instead of by time?
2. Where exactly do you buffer writes, and what makes the rollback on trap correct?
3. Map your sandbox to Soroban: what is its equivalent of your fuel, storage and `set_return`?

## S10-GATE · Stage 10 gate: closed-book exam + catch-up
*1 day (3 h)*

**Exam.** Closed book, 75 min, blank file `tasks/src/s10_gate.rs`: implement `struct Stack<T> { ptr: NonNull<T>, len: usize, cap: usize }` with `new`, `push` (grows with `alloc::alloc::realloc`), `pop -> Option<T>` and `Drop`, every `unsafe` block justified with `// SAFETY:`, and 4 tests that pass under `cargo miri test`. Then answer 5 oral questions drawn from the S10 checkpoints, including one on the host/guest boundary.

**Pass criteria.** Solved without notes or AI, Miri clean, no public safe fn can cause UB, oral answers correct or corrected with a clear explanation. Remaining time: rebuild the S10-T4 host from a blank file.
