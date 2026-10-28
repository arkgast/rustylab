# Stage 11: Encoding & cryptography

**Phase:** III · Systems
**Outcome:** You can design a canonical binary encoding and prove it against SCALE and Borsh, choose and domain-separate hashes, build Merkle trees with inclusion proofs, and sign transactions so they cannot be forged, malleated or replayed.

Each task: `/study` (recall, then probes: predict, run, explain the gap), solve the katas, run them and read the output, make the gate green, write the journal, then `/review <ID>`.

## S11-T1 · serde in depth
*2 days (6 h)*

**Why it matters.** Every RPC, config file and genesis spec goes through serde. Knowing its data model lets you control wire formats instead of accepting whatever `derive` produces, and borrow from input instead of allocating.

**Probes**
- The serde data model: 29 types, Serializer/Deserializer as visitors; derive output expanded with `cargo expand`
- Attributes: `rename_all`, `tag`/`content`/`untagged`, `default`, `skip_serializing_if`, `deny_unknown_fields`, `with`
- Zero-copy: `#[serde(borrow)] &'a str` vs `String`, allocation counts printed side by side

*Previews:* Allocation counts come from the counting allocator in `tasks/src/lab.rs` (harness).

**Reference** (read after the probes, or when stuck)
- serde.rs: "Data model", "Attributes" (container, variant, field), "Implementing Serialize", "Implementing Deserialize", "Deserializer lifetimes"
- serde_json docs: `from_str`, `Value`, `RawValue`

**Build.** `tasks/src/s11/t01_serde.rs`: `RpcRequest<'a>` (borrowed `method`, `params` as `&RawValue`), an internally tagged `Tx` enum (`Transfer`, `Stake`), `Amount(u128)` that serializes as a decimal string (custom `Serialize`/`Deserialize` with a `Visitor`), `Hex32([u8; 32])` as `0x`-prefixed hex, and a `Genesis` struct with `deny_unknown_fields`.

**Run.** `scripts/run.sh S11-T1` prints round-trips and the rejected inputs with their errors.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s11_t01.rs` pass
- [ ] `Amount` round-trips `u128::MAX` exactly through JSON (no float path)
- [ ] `RpcRequest` deserializes without allocating the method name (a test borrows from the input)
- [ ] Malformed hex, wrong length and unknown fields give errors, never panics

**Stretch.** Write a tiny serializer of your own that prints the serde data model calls (`serialize_struct`, `serialize_field`, ...) for any type.

**Checkpoint questions** (answer in `journal/S11-T1.md`; the reviewer asks them again)
1. Why does JSON lose precision on large integers, and why does it matter for balances?
2. What does `#[serde(borrow)]` require of the input's lifetime, and when can it not borrow?
3. Internally tagged vs untagged enums: which is safer to accept from untrusted input, and why?

## S11-T2 · Binary codecs: Borsh, SCALE & canonical encoding
*3 days (9 h)*

**Why it matters.** A hash or signature is over bytes, not values. If two byte strings decode to the same value, signatures malleate and state roots diverge. Substrate uses SCALE, Solana programs mostly Borsh; you must know why both insist on one encoding per value.

**Probes**
- Little-endian fixed width; length prefixes; enum discriminants; why no self-description
- SCALE compact integers: the four modes, worked byte by byte; non-canonical encodings and why decoders must reject them
- Differential testing: your codec vs the reference crate on random values (proptest)

**Reference** (read after the probes, or when stuck)
- SCALE codec docs (docs.polkadot.com "Data encoding"; parity-scale-codec README): fixed ints, compact ints, `Option`, `Vec`, enums
- Borsh specification (borsh.io): determinism rules, length prefixes, why `HashMap` is sorted
- parity-scale-codec and borsh crate docs (`Encode`, `Decode`, `BorshSerialize`)

**Build.** `tasks/src/s11/t02_codec.rs`: traits `Encode`/`Decode` (`decode(&mut &[u8]) -> Result<Self, CodecError>`), impls for `u8..u128`, `bool`, `Option<T>`, `Vec<T>`, `[u8; N]`, `BTreeMap<K, V>`, a `Compact<u128>` wrapper, and a hand-written impl for `Transfer { from: [u8; 32], to: [u8; 32], amount: Compact<u128>, memo: Option<Vec<u8>> }`. Decoding must consume all input (`decode_all`).

**Run.** `scripts/run.sh S11-T2` prints hex encodings next to `parity-scale-codec` and `borsh` output.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s11_t02.rs` pass, including proptest differential tests against `parity-scale-codec` (SCALE mode) and `borsh`
- [ ] Non-canonical compact encodings, trailing bytes, invalid `bool` bytes and oversize length prefixes are rejected
- [ ] Decoding never allocates more than the input could justify (length prefix capped against remaining bytes)
- [ ] No panicking slice indexing in decode paths

**Stretch.** Write a `compile_fail` doctest showing that `HashMap` does not implement your `Encode`, and explain why in the journal.

**Checkpoint questions** (answer in `journal/S11-T2.md`; the reviewer asks them again)
1. Why must a consensus codec reject non-canonical encodings instead of just accepting them?
2. Walk through how `Compact(70_000)` encodes in SCALE, byte by byte.
3. An attacker sends a length prefix of `u32::MAX` with 4 bytes of payload. What does a naive decoder do, and what does yours do?

## S11-T3 · Hashing, content addressing & domain separation
*1 day (3 h)*

**Why it matters.** Hashes identify blocks, transactions and storage keys. Using the same hash for two purposes without a prefix lets an attacker make one kind of object look like another.

**Probes**
- Preimage, second-preimage and collision resistance; why `twox` is fast but only for trusted keys
- Domain separation: `H("tx" || data)` vs `H(data)`; BLAKE2 personalization
- Content addressing: an ID that is the hash of the canonical encoding

**Reference** (read after the probes, or when stuck)
- `sha2`, `blake2` and `sha3` (Keccak256) crate docs; the `digest` trait
- Polkadot SDK docs: storage hashers (`Blake2_128Concat`, `Twox64Concat`, `Identity`) and when each is safe
- NIST SP 800-185 intro (customization strings) or the BLAKE2 paper section on personalization

**Build.** `tasks/src/s11/t03_hashing.rs`: a `Hasher` trait with `Blake2b256`, `Sha256`, `Keccak256` impls via the `digest` crate; `tagged_hash(tag: &[u8], data: &[u8]) -> [u8; 32]` (length-prefixed tag); `TxId::of(&Transfer)` over the S11-T2 encoding; `storage_key(pallet, item, key)` in the `Blake2_128Concat` layout.

**Run.** `scripts/run.sh S11-T3` prints known-answer vectors and the storage key for a sample account.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s11_t03.rs` pass, including published test vectors for all three hashes
- [ ] A test shows two different tags give different hashes for identical data
- [ ] `tagged_hash` is unambiguous: `("ab", "c")` and `("a", "bc")` differ (test)

**Stretch.** Benchmark the three hashes on 1 KB inputs with criterion and record the numbers.

**Checkpoint questions** (answer in `journal/S11-T3.md`; the reviewer asks them again)
1. Why must the tag be length-prefixed or fixed-size?
2. When is `Twox64Concat` safe as a storage hasher, and when is it a DoS vector?
3. Ethereum uses Keccak256, not NIST SHA3-256. Do they produce the same output, and why should you care?

## S11-T4 · Merkle trees & proofs
*2 days (6 h)*

**Why it matters.** Merkle roots let a light client verify one transaction or one storage value against a 32-byte header field. The classic bug is a proof that treats an inner node as a leaf.

**Probes**
- Building a tree bottom-up; odd leaf counts; RFC 6962 split vs Bitcoin duplicate-last
- Second-preimage defense: `0x00` leaf prefix, `0x01` node prefix
- Inclusion proofs: sibling path + index; verification in O(log n)

**Reference** (read after the probes, or when stuck)
- RFC 6962 (Certificate Transparency) section 2.1: Merkle Hash Trees, audit paths
- Bitcoin whitepaper section 7 (Reclaiming Disk Space) and section 8 (Simplified Payment Verification)
- Bitcoin CVE-2012-2459 write-up (duplicate last leaf ambiguity)

**Build.** `tasks/src/s11/t04_merkle.rs`: `MerkleTree::from_leaves(&[Vec<u8>]) -> Result<MerkleTree, MerkleError>`, `root()`, `prove(index) -> Option<Proof>`, `Proof::verify(root, leaf, index, leaf_count) -> bool`, generic over the S11-T3 `Hasher`; naive recursive version vs iterative level-by-level version side by side.

**Run.** `scripts/run.sh S11-T4` prints a 5-leaf tree level by level and a proof.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s11_t04.rs` pass, including RFC 6962 test vectors
- [ ] A test proves an inner node cannot be passed off as a leaf
- [ ] Proptest: every leaf of a random tree verifies; any flipped bit in leaf, path or index fails
- [ ] Empty tree has a defined root (document the choice)

**Stretch.** Implement a consistency proof (RFC 6962 2.1.2) between tree sizes m and n.

**Checkpoint questions** (answer in `journal/S11-T4.md`; the reviewer asks them again)
1. Without leaf/node prefixes, how could an attacker forge an inclusion proof?
2. What was CVE-2012-2459, and why does RFC 6962's split avoid it?
3. Why does the verifier need the leaf index and the tree size, not only the path?

## S11-T5 · Signatures, nonces & replay protection
*2 days (6 h)*

**Why it matters.** A valid signature only proves someone signed some bytes. Whether those bytes can be replayed on another chain, at another time, or reinterpreted as another message is entirely payload design.

**Probes**
- Keypairs, deterministic Ed25519 signing, `verify_strict` and malleability
- Signing payload = domain tag + chain id + genesis hash + nonce + call + expiry
- Replay types: same chain (nonce), cross-chain (chain id), cross-message (domain tag), stale (expiry)

**Reference** (read after the probes, or when stuck)
- RFC 8032 (Ed25519) sections 1, 5.1 intro and 8 (security considerations)
- ed25519-dalek docs: `SigningKey`, `VerifyingKey`, `verify_strict` vs `verify`
- EIP-155 (chain id in signatures) and the Polkadot SDK docs on `SignedExtension`/transaction extensions (`CheckNonce`, `CheckGenesis`, `CheckMortality`)

**Build.** `tasks/src/s11/t05_signatures.rs`: `SigningPayload { chain_id, genesis_hash, nonce, valid_until, call }` with canonical encoding from S11-T2, `sign(&SigningKey, &SigningPayload) -> Signature`, `verify(&VerifyingKey, &SigningPayload, &Signature) -> Result<(), SigError>`, and a `NonceTracker` (`BTreeMap<AccountId, u64>`) with `check_and_bump`.

**Run.** `scripts/run.sh S11-T5` signs, verifies, then shows each replay attempt rejected with its reason.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s11_t05.rs` pass
- [ ] Changing any single payload field invalidates the signature (test per field)
- [ ] Uses `verify_strict`; the journal explains why
- [ ] `check_and_bump` uses checked arithmetic and only bumps after a successful check

**Stretch.** Add a batch verification path (`ed25519_dalek::verify_batch`) and measure the speedup on 1,000 signatures.

**Checkpoint questions** (answer in `journal/S11-T5.md`; the reviewer asks them again)
1. Why is the chain id inside the signed payload rather than just in the transaction envelope?
2. Strict nonces vs a nonce window: what does each cost in throughput and in safety?
3. What is signature malleability, and how could it break a system that indexes transactions by hash?

## S11-T6 · Capstone: signed transaction pipeline
*2 days (6 h)*

**Why it matters.** This is the front door of every chain: bytes arrive from the network and must be decoded, authenticated and checked for freshness before any state is touched. Each step fails closed.

**Probes**
- Pipeline stages and their error types: decode, verify, nonce, expiry
- Ordering: cheap checks first, but never act on unauthenticated data
- Negative tests as the spec: every rejection reason has a test

**Reference** (read after the probes, or when stuck)
- Re-read your S11-T2 and S11-T5 journals
- Wormhole exploit post-mortem (Feb 2022, signature-verification bypass via unchecked sysvar account)

**Build.** `tasks/src/s11/t06_pipeline.rs`: `UncheckedTx` (wire bytes) → `decode` → `verify` → `CheckedTx` (typestate: only constructible through verification) → `NonceTracker::apply`; `PipelineError` with variants for each stage. Write an ADR on the signing payload layout.

**Run.** `scripts/run.sh S11-T6` processes a batch of 10 wire transactions and prints accepted/rejected with reasons.

**Acceptance criteria** (all required)
- [ ] All tests in `tasks/tests/s11_t06.rs` pass
- [ ] `CheckedTx` has no public constructor; a `compile_fail` doctest proves it
- [ ] At least 8 negative tests: truncated bytes, trailing bytes, bad signature, wrong key, wrong chain id, replayed nonce, future nonce, expired
- [ ] A rejected tx never changes the nonce tracker
- [ ] ADR written in `docs/adr/`

**Stretch.** Fuzz `decode` + `verify` with `cargo fuzz` for 10 minutes (a preview: fuzzing is taught in S13-T3); record the corpus size.

**Checkpoint questions** (answer in `journal/S11-T6.md`; the reviewer asks them again)
1. How does the `CheckedTx` typestate make "forgot to verify" a compile error?
2. Explain the Wormhole bug in one paragraph and the line of your pipeline that prevents its analog.
3. Why check the nonce after the signature and not before?

## S11-GATE · Stage 11 gate: closed-book exam + catch-up
*1 day (3 h)*

**Exam.** Closed book, 90 min, blank file `tasks/src/s11_gate.rs`: implement SCALE compact encode/decode for `u64` (rejecting non-canonical forms) with 4 tests, and a 4-leaf Merkle tree with leaf/node prefixes, `prove(index)` and `verify`, with a test that an inner node cannot pass as a leaf. Then answer 5 oral questions drawn from the S11 checkpoints, including one on replay protection.

**Pass criteria.** Solved without notes or AI, compiles, tests pass, no panicking indexing in decode, oral answers correct or corrected with a clear explanation. Remaining time: redo the weakest kata from a blank file.
