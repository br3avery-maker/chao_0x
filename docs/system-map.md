# chao_0x System Map

## One-sentence architecture

**chao_0x is a local-first AI-indexed data economy network using a native account-chain / block-lattice ledger where every wallet is its own node, with a sovereign accounting kernel that rewards useful presence: time, effort, energy, storage, uptime, bandwidth, indexing, curation, delivery, availability, and challenge responses.**

## High-level stack

```text
Apps
wallet, music/library, publishing, curation, dashboards
        ↓
Web Runtime
safe APIs for mini-apps and local UI
        ↓
Network
sync, discovery, verification, challenge, relay, peer delivery
        ↓
OS Services
scan, index, store, serve, schedule, measure
        ↓
Reward / Proof Kernel
proof validation, reward claims, stake, burn, slash, challenge outcomes
        ↓
Data / Identity Kernel
accounts, keys, signatures, hashes, native IDs, manifests, account-chain blocks, schemas
        ↓
Device / Storage
files, SQLite, keystore, media, battery, network, local storage
```

## Native object prefixes

```text
chao_0x:account:...
chao_0x:block:...
chao_0x:asset:...
chao_0x:index:...
chao_0x:proof:...
```

These prefixes mark native chao objects, not random files, JSON blobs, URLs, or external-chain objects.

## Core correction

Do not model this as "algorithmic mining," an EVM-first project, or an app with optional rewards.

Model it as:

```text
Native block-lattice ledger
+ sovereign accounting kernel
+ useful network work
= account-chain ownership + indexing + routing + hosting + curating + proving availability
```

The network rewards nodes for maintaining useful presence and useful data state, not for wasting compute or routing every primitive through external contracts.

## Trust boundary

```text
Apps request work.
OS services perform work.
Network peers verify or challenge work.
Reward / Proof Kernel accounts for work.
Data / Identity Kernel defines the objects and signatures.
```

Rewards must not come from self-reported app activity alone.
