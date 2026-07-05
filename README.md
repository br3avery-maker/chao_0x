# chao_0x

**chao_0x** is a local-first decentralized data/economy network built around a native account-chain / block-lattice ledger and a sovereign accounting kernel.

Core thesis:

> Each user wallet is its own node/account chain. Point the hub at useful user-controlled data → OS services prepare, index, store, serve, and verify it → the Reward / Proof Kernel accounts for useful presence → native assets can earn, burn, stake, trade, mirror, route value, or power mini-apps.

This repo is an early architecture scaffold for the chao_0x system.

## What chao_0x is

chao_0x is not an EVM-first token, wallet, NFT project, media player, storage layer, or mining app.

It is a **native data asset network** where wallets/accounts maintain their own chains and folders, archives, music, writing, static sites, mini-apps, datasets, models, and readable libraries can become useful network surfaces.

The economy is not a later add-on. It is part of the operating system.

## Kernel stack

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

## Ledger shape

```text
chao_0x network
├─ account chain / wallet node A
├─ account chain / wallet node B
├─ account chain / wallet node C
└─ ...
```

Each wallet owns its own ordered chain of signed blocks. The global network is the lattice formed by all account chains plus the cross-account references between them.

## Native identifiers

Native chao objects should use a `chao_0x:` prefix.

```text
chao_0x:account:...
chao_0x:block:...
chao_0x:asset:...
chao_0x:index:...
chao_0x:proof:...
```

The prefix marks something as a native chao object, not just a random file, JSON blob, URL, or external-chain object.

## Primary loops

### Creator loop

```text
point hub at folder
→ OS services scan/index/prepare
→ user confirms mode
→ account chain records release/index event
→ network publishes/indexes
→ asset earns native rewards
```

### Indexer loop

```text
scan useful data
→ build local AI-enriched index
→ publish index commitment block
→ answer peer challenges
→ earn native rewards for useful network state
```

### Seeder loop

```text
host useful data
→ serve chunks/routes
→ prove availability/delivery
→ receive native rewards
```

### Curator loop

```text
collect/tag/recommend useful data
→ publish curation block/graph
→ earn from discovery/use
```

## Protocol split

```text
Data / Identity Kernel = accounts, keys, hashes, IDs, manifests, blocks, schemas
Reward / Proof Kernel  = proof validation, rewards, stakes, burns, slashes
OS Services            = scan, index, store, serve, schedule, measure
Native chao_0x ledger  = block-lattice economy and account-chain history
Wallet/account chain   = user's own node/state chain
Nostr                  = discovery, signed events, store/listing metadata
IPFS                   = content addressing / CIDs
Arweave                = permanent public artifacts
Filecoin               = storage-market/deal layer
BYOCloud               = encrypted fallback mirrors
WebRTC                 = live peer delivery
Local AI               = formatting, tagging, indexing, risk scoring, metadata
Android                = local hub shell + foreground service + WebView mini-app runtime
External chains        = optional bridge/export layers, not the base network
```

## MVP target

The first MVP should prove the native data/index/accounting loop, not the entire sovereign OS.

```text
Android app
+ Rust data/identity kernel
+ Rust reward/proof kernel
+ local folder scanner
+ music + readable text indexer
+ manifest generator
+ Nostr event publisher
+ native account-chain event log
+ basic contribution dashboard
```

See `/docs` for the system map and protocol notes.
