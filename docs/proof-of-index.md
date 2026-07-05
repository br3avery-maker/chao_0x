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
