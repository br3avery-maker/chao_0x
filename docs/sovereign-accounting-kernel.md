# Sovereign Accounting Kernel

## Core correction

The chao_0x core is not only hashes, signatures, manifests, and account-chain blocks.

The core is a sovereign accounting kernel.

Rewards and proofs are not app-layer points. They are native ledger primitives.

```text
chao_0x rewards useful presence.
```

Useful presence includes:

- time
- effort
- energy
- storage
- uptime
- bandwidth
- indexing
- curation
- delivery
- availability
- challenge responses

## Layer map

```text
Apps
  ↓
Web Runtime
  ↓
Network
  ↓
OS Services
  ↓
Reward / Proof Kernel
  ↓
Data / Identity Kernel
  ↓
Device / Storage
```

## Data / Identity Kernel

This is the lowest logical kernel layer.

It defines:

- accounts
- keys
- signatures
- content hashes
- sovereign file IDs
- canonical manifests
- account-chain blocks
- schema validation

Native chao identifiers use the `chao_0x:` prefix.

Examples:

```text
chao_0x:account:...
chao_0x:block:...
chao_0x:asset:...
chao_0x:index:...
chao_0x:proof:...
```

The prefix marks something as a native chao object, not just a random file, JSON blob, URL, or external-chain object.

## Reward / Proof Kernel

This is the second core layer.

It validates useful contribution and produces native ledger events.

Proof families:

```text
proof_of_storage
proof_of_uptime
proof_of_delivery
proof_of_index
proof_of_curation
proof_of_energy
proof_of_effort
proof_of_availability
proof_of_challenge_response
```

Native accounting events:

```text
reward_claim
slash
stake
unstake
burn
challenge
challenge_response
```

The chain is not only a record of what happened. It is also a record of what useful contribution was proven.

## OS Services

OS services create proof inputs.

Examples:

- scanner service proves indexing work
- storage service proves bytes preserved
- uptime service proves availability windows
- network service proves delivery
- AI/indexer service proves metadata work
- scheduler records active service time
- battery/thermal/network monitors estimate energy cost

OS services do work. The kernel validates and records it.

## Network Layer

The network lets other nodes verify or challenge claims.

Examples:

- peer pings for uptime
- random chunk challenges for storage
- route tests for availability
- delivery receipts
- Nostr announcements
- IPFS/WebRTC proof material
- representative votes or dispute outcomes

Rewards must not come from self-reported activity alone.

## Web Runtime

Mini-apps can request access to reward/proof APIs, but they are not trusted directly.

A mini-app may request:

```text
index job
storage pin
publish manifest
proof status
reward balance
```

Only OS services and the core should create signed proof/reward blocks.

## Apps

Apps show and use the system:

- wallet
- contribution dashboard
- storage dashboard
- uptime dashboard
- file/index browser
- music/library app
- curation shelves
- publishing tool
- challenge/dispute viewer

## Design rule

```text
Apps request work.
OS services perform work.
Network peers verify or challenge work.
Reward / Proof Kernel accounts for work.
Data / Identity Kernel defines the objects and signatures.
```

The economy is part of the operating system.
