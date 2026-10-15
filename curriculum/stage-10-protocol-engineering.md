# Stage 10: Protocol engineering

**Outcome:** You can build the core of a small blockchain from primitives and explain every design decision, then map it to FRAME/Substrate.

**Window:** 2027-02-15 → 2027-03-10 · tracker: [`ROADMAP.md`](../ROADMAP.md)

Each task: read first, build in `tasks/` (or the crate named), write the journal entry, then run `/task-review <ID>`.

## S10-T1 · Hashing & content addressing
*2 days (6 h) · due 2027-02-16*

**Why it matters.** Hashes give you identity, integrity and tamper evidence. Everything else builds on them.

**Read first**
- `sha2`/`blake2` crate docs
- Bitcoin whitepaper §3-4

**Build.** A hash-chained `Chain` of blocks with `blake2` or `sha2`, `Hash` newtype, and a verify function that finds the first corrupted block.

**Acceptance criteria** (all required)
- [ ] Tamper test at every position returns the exact index
- [ ] Hash input is canonical: documented byte layout
- [ ] Domain separation tag used and justified
- [ ] No `unsafe`

**Stretch.** Add a hashing trait so the algorithm is pluggable.

**Checkpoint questions** (answer in `journal/S10-T1.md`; the reviewer will ask them again)
1. What properties do you rely on from the hash (pre-image, second pre-image, collision)?
2. Why must the byte encoding be canonical?
3. What is domain separation?

## S10-T2 · Merkle trees & proofs
*3 days (9 h) · due 2027-02-19*

**Why it matters.** Merkle proofs let light clients verify state without the whole data set.

**Read first**
- Merkle tree explainers; RFC 6962 (Certificate Transparency) §2.1

**Build.** A binary Merkle tree with inclusion proofs and verification, handling odd leaf counts and second-preimage attacks (leaf/node domain separation).

**Acceptance criteria** (all required)
- [ ] Proof size is `O(log n)` (asserted)
- [ ] Property: every leaf's proof verifies; any bit flip fails
- [ ] Test for the odd-count duplication pitfall and your chosen handling
- [ ] Root of empty tree defined and documented

**Stretch.** Implement a sparse Merkle tree over 256-bit keys.

**Checkpoint questions** (answer in `journal/S10-T2.md`; the reviewer will ask them again)
1. Why prefix leaves and nodes differently?
2. How does a light client use a proof?
3. What's the cost of updating one leaf?

## S10-T3 · Signatures & replay protection
*3 days (9 h) · due 2027-02-23*

**Why it matters.** Authorization is cryptographic. Getting the signed payload wrong is the classic vulnerability.

**Read first**
- `ed25519-dalek` docs
- RFC 8032 overview

**Build.** Keypairs, `SignedTransaction` with signing payload = (chain_id, nonce, call), verification before dispatch; reject cross-chain replay.

**Acceptance criteria** (all required)
- [ ] Wrong chain id, wrong nonce, modified call each rejected (tests)
- [ ] Signing payload is a documented canonical byte layout
- [ ] Batch verification benchmarked vs one-by-one
- [ ] Secrets never logged or `Debug`-printed (test via `Debug` output)

**Stretch.** Add a multi-sig account type.

**Checkpoint questions** (answer in `journal/S10-T3.md`; the reviewer will ask them again)
1. What exactly is signed and why include chain id?
2. Why verify signatures before touching state?
3. What is malleability?

## S10-T4 · Serialization & a SCALE-like codec
*3 days (9 h) · due 2027-02-26*

**Why it matters.** Every protocol is defined by its bytes. Writing a codec by hand makes the format real.

**Read first**
- SCALE codec spec (Substrate docs)
- `parity-scale-codec` docs

**Build.** A `Encode`/`Decode` trait pair with compact integer encoding, `Vec`, `Option`, tuples and structs; fuzz decode with arbitrary bytes.

**Acceptance criteria** (all required)
- [ ] Round-trip property tests for all types
- [ ] Decoder never panics on arbitrary input (proptest with random bytes)
- [ ] Compact encoding matches the SCALE spec vectors
- [ ] Differential test against `parity-scale-codec`

**Stretch.** Add a derive macro for `Encode`/`Decode`.

**Checkpoint questions** (answer in `journal/S10-T4.md`; the reviewer will ask them again)
1. Why do you need bounds on length prefixes when decoding?
2. Compact integers: how are the 4 modes distinguished?
3. Why must decoding be canonical (no multiple encodings)?

## S10-T5 · Consensus toy: PoA/PoW & fork choice
*3 days (9 h) · due 2027-03-02*

**Why it matters.** Consensus is how independent nodes agree on one history. Reorgs are where state rollback gets real.

**Read first**
- Bitcoin longest-chain rule; Aura round-robin overview

**Build.** A `Node` that produces blocks (round-robin authority or toy PoW), validates and imports them, tracks forks, picks the best chain and reorgs state using the overlay.

**Acceptance criteria** (all required)
- [ ] Reorg of depth 3 results in the same state as replaying the winner from genesis
- [ ] Invalid blocks rejected without side effects
- [ ] Fork-choice rule documented and property tested
- [ ] Finality/confirmation depth discussed in the journal

**Stretch.** Add simple finality (GRANDPA-lite: 2/3 votes) as a design sketch.

**Checkpoint questions** (answer in `journal/S10-T5.md`; the reviewer will ask them again)
1. How do you roll back state on a reorg efficiently?
2. What assumption does your consensus make about honest participants?
3. What's the difference between safety and liveness here?

## S10-T6 · Networking-lite: gossip simulation
*3 days (9 h) · due 2027-03-05*

**Why it matters.** Distributed behavior is the hardest to reason about; a deterministic simulator lets you test it.

**Read first**
- libp2p gossipsub overview

**Build.** Simulate N nodes over channels with configurable latency/drop; gossip transactions and blocks; assert eventual convergence.

**Acceptance criteria** (all required)
- [ ] 5 nodes with 20% message loss converge to the same head (seeded RNG, reproducible)
- [ ] Duplicate messages are idempotent
- [ ] Partition-heal test: nodes reconverge
- [ ] Simulation is deterministic given a seed

**Stretch.** Add a sync protocol to catch up a late joiner.

**Checkpoint questions** (answer in `journal/S10-T6.md`; the reviewer will ask them again)
1. How do you make a distributed test reproducible?
2. What happens on a partition and who wins on heal?
3. How do you stop gossip storms?

## S10-T7 · Capstone: no_std core & FRAME mapping
*3 days (9 h) · due 2027-03-09*

**Why it matters.** Real runtimes compile to `no_std` WASM. Mapping your own design to FRAME proves you understand both.

**Read first**
- Substrate/Polkadot SDK docs: FRAME pallets
- The Embedded Rust Book: `no_std`

**Build.** Split the runtime core into a `no_std + alloc` crate, then write a FRAME-style version of your Balances pallet (follow the official tutorial) and a comparison document.

**Acceptance criteria** (all required)
- [ ] Core crate builds with `--no-default-features` for `wasm32-unknown-unknown` or a `no_std` check target
- [ ] `docs/frame-mapping.md`: table mapping your concepts to FRAME (Config, Call, Error, Event, storage, origin, weight)
- [ ] A list of 5 things FRAME gives you that your toy lacks
- [ ] Final design review prepared

**Stretch.** Run your pallet inside a minimal Substrate/Polkadot SDK template node.

**Checkpoint questions** (answer in `journal/S10-T7.md`; the reviewer will ask them again)
1. Why must runtimes be `no_std`?
2. What does FRAME's `Origin` model add over a plain account id?
3. What would it take to upgrade your runtime without a hard fork?

## S10-GATE · Stage gate
*1 day (3 h) · due 2027-03-10*

**Exam.** Final exam, 3h: design review of your own chain (state, consensus, networking, upgrade story) plus a live-coding extension requested by the reviewer.

**Pass criteria.** Exam solved without notes or AI help, compiles, tests pass, and all oral answers are correct or corrected with a clear explanation. Remaining time: fix weak spots flagged by the reviewer and re-read failed checkpoints.
