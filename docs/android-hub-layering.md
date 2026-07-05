# Android Hub Layering

## Intent

`chao_0x` is an Android-first personal data operating environment, not only a wallet, token app, or storage client. The hub should combine local files, AI indexing, sovereign manifests, network publishing, mini-apps, and native rewards inside one layered system.

The core design principle is that contribution rewards are built into the kernel. Time, effort, energy, storage, bandwidth, and uptime are not optional marketplace features; they are first-class accounting inputs for the native ledger.

## Layer Stack

```text
Apps
Web Runtime
Network
OS Services
Reward / Proof Kernel
Data / Identity Kernel
Device / Storage
```

## Device / Storage

This layer is the physical Android device and user-controlled storage. It includes shared storage, app-private storage, SD cards, MediaStore, local SQLite databases, encrypted backups, and Android Keystore-backed secrets.

This layer answers: where does the user's data physically live?

## Data / Identity Kernel

This is the lowest logical layer of the hub. It defines accounts, keys, signatures, content hashes, canonical manifests, account-chain blocks, schemas, and sovereign object identifiers.

The `chao_0x:` prefix should identify native sovereign objects, for example:

```text
chao_0x:account:...
chao_0x:asset:...
chao_0x:block:...
chao_0x:index:...
chao_0x:proof:...
```

The prefix marks data as part of the native chao object model instead of a generic file, URL, or JSON blob.

## Reward / Proof Kernel

This layer validates useful contribution and turns it into native ledger state. It should support cryptographic proof and challenge mechanics wherever possible, especially for storage, uptime, delivery, and index claims.

Core contribution categories:

- time: useful participation windows
- effort: indexing, curating, tagging, moderating, creating, or reviewing
- energy: battery or compute spent doing useful work
- storage: bytes preserved, mirrored, or pinned
- uptime: reachable service windows
- bandwidth: delivered content or successful routes
- indexing: searchable metadata and commitments
- curation: shelves, playlists, tags, relationships, and recommendations

Reward output should be native `chao_0x` ledger activity such as reward claims, stake, slash, burn, and challenge outcomes. Rewards should not depend on app-local points or an external EVM token.

## OS Services

OS services perform the work that creates proof inputs. Examples include scanner, indexer, AI preparation, storage manager, uptime monitor, delivery service, scheduler, notification service, media processor, and local search.

The services may measure local activity, but the kernel decides what is valid, signed, claimable, or challengeable.

## Network

The network layer connects hubs and external protocols. It moves signed objects, manifests, proofs, commitments, chunks, and discovery events without owning the data model.

Expected adapters include Nostr discovery, WebRTC delivery, IPFS export, BYOCloud sync, Arweave/Filecoin publishing, peer challenges, delivery receipts, and representative or dispute messages.

## Web Runtime

The Web Runtime is the programmable surface for mini-apps. It should expose safe APIs through WebView or a local bridge for search, manifests, publishing, signing requests, media playback, storage jobs, and proof status.

Mini-apps can request work, but they should not directly mint rewards or create trusted proofs. OS services and the core kernel remain the authority.

## Apps

Apps are replaceable user experiences on top of the hub. Initial apps may include a file explorer, music library, readable archive, manifest editor, wallet/account-chain viewer, contribution dashboard, storage dashboard, Nostr browser, publishing tool, curation shelves, and AI assistant.

The hub should remain coherent if individual apps are redesigned or replaced, because the lower layers preserve identity, data, proofs, and history.

## Operating Model

```text
Apps use hub capabilities.
Web Runtime exposes safe APIs.
Network verifies, challenges, syncs, and discovers.
OS Services scan, index, store, serve, and measure.
Reward / Proof Kernel validates contribution and records claims.
Data / Identity Kernel defines accounts, hashes, manifests, blocks, and signatures.
Device / Storage provides files, databases, keys, battery, and network access.
```

The result is an Android hub where local ownership, useful work, sovereign data, and native rewards are built into the operating environment from the beginning.
