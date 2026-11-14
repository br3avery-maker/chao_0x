# Proof of Index

## Mining replacement

chao_0x should replace wasteful algorithmic mining with useful index mining.

```text
Mining = maintaining and proving useful network state
```

## Node role

A chao_0x node is:

```text
indexer
+ curator
+ router
+ storage witness
+ optional host
```

It earns because it helps the network answer:

- Who has this file?
- Is this asset still available?
- What tags describe this object?
- Which peers served it successfully?
- Which mirror is fastest?
- Which version is current?
- Which assets are related?
- Which index entries are fake or stale?

## Proof families

```text
Proof-of-Index
- Node maintains useful searchable metadata.

Proof-of-Availability
- Node proves content is retrievable.

Proof-of-Routing
- Node knows useful routes to content.

Proof-of-Curation
- Node improves tags, shelves, recommendations, or relationships.

Proof-of-Freshness
- Node's index reflects current network state.

Proof-of-Delivery
- Node successfully served chunks/content.

Proof-of-Challenge
- Node detects stale/fake/malicious claims.
```

## Private index + public commitment

A hash cannot be decoded. An encrypted index can be decoded by the owner.

Use:

```text
Encrypted Index Bundle
+ Index Commitment Hash / Merkle Root
```

Flow:

```text
local index database
        ↓
canonicalize entries
        ↓
Merkle tree
        ↓
index root
        ↓
public Nostr/Base commitment
```

Private side:

```text
Encrypted index lives on device or BYOCloud.
Hub decrypts it locally using user keys.
```

Public side:

```text
Network sees index root, entry count, timestamp, and proof responses.
```

## Challenge examples

- Prove asset X is in your index.
- Prove CID Y has at least one working route.
- Prove you can serve random chunk Z.
- Prove your metadata matches the signed creator manifest.
- Prove your index is not stale.

Success earns reward. Failure lowers reputation or triggers slashing if staked.


## Local commitment profile: `chao-index-merkle-v0`

`chao-core::index` implements an **ordered membership commitment**, not an
availability, freshness, ownership, signature, or reward proof. This is an
experimental local profile; it is not a claim that the network or other
implementations have adopted it. A verifier must obtain the root and algorithm
from a trusted, authenticated commitment and reject unknown or absent algorithm
identifiers for this profile. The commitment itself is unsigned. Callers are
responsible for validating the owner object and protecting their signing keys.

### Exact bytes

Each entry is encoded as the ASCII bytes `chao.index.entry.v0` followed by a
zero byte, then these fields in order: `asset_id`, `content_hash`,
`manifest_cid`, `asset_type`, `mode`, `updated_at`. Each string is its exact
UTF-8 bytes prefixed by its byte length as an unsigned 64-bit big-endian
integer. Strings are not Unicode-normalized. `manifest_cid` is a single zero
byte for `None`, or a byte `01` followed by the length-prefixed string for
`Some` (including an empty string). `updated_at` is an unsigned 64-bit
big-endian integer. No validation or interpretation of identifiers is implied.

- A leaf is `SHA256(00 || entry_bytes)`.
- An internal node is `SHA256(01 || left || right)`, using raw 32-byte hashes.
  When a level has an odd node count, its final node is duplicated.
- The internal root for an empty index is `SHA256(02)`; for a one-entry index
  it is that entry's leaf.
- The published root is `SHA256(03 || count_u64_be || internal_root)`.
  The count binding distinguishes an odd tree from one containing an extra
  duplicate. Hash strings use `sha256:` followed by 64 lowercase hexadecimal
  digits.

Entries are committed in caller-supplied order; no implicit sorting or
content deduplication occurs. Identical entries at different positions remain
separate occurrences. A proof contains the zero-based leaf index, entry count,
and sibling hashes from leaf to root. The verifier checks the index, count,
proof depth, hash encoding, and the duplicated sibling at an odd boundary;
extra or missing siblings fail. Callers must compare the root against a
trusted commitment and ensure its `entry_count` equals the proof count. The
older `canonical_entry_hash` function is a separate legacy JSON hash and is
**not** a leaf in this profile.

The fixed vectors in `crates/chao-core/tests/index_commitment.rs` include
Unicode, absent and present optional values, empty input, odd trees, and
malformed proofs. Generate the checked-in sample with:

```sh
cargo run -q -p chao-core --example index_commitment
```

The sample `examples/index-commitment-generated.json` was generated from two
synthetic entries and can be validated against the existing commitment schema.
No entry text is included in that object. Roots and membership proofs still
leak information: low-entropy entries can be guessed and tested, and a proof
reveals the entry supplied with it, its position and the count. This profile
provides no encryption or hiding guarantee. Do not publish private metadata,
owner fields, proofs, or roots without considering those disclosures.
