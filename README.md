# chao_0x

**chao_0x** is a local-first decentralized data/economy network built around a native account-chain / block-lattice ledger.

Core thesis:

> Each user wallet is its own node/account chain. Point the hub at useful user-controlled data → AI prepares it → the protocol classifies it → the network indexes it → storage/peers distribute it → native assets can earn, burn, stake, trade, mirror, route value, or power mini-apps.

This repo is an early architecture scaffold for the chao_0x system.

## What chao_0x is

chao_0x is not an EVM-first token, wallet, NFT project, media player, storage layer, or mining app.

It is a **native data asset network** where wallets/accounts maintain their own chains and folders, archives, music, writing, static sites, mini-apps, datasets, models, and readable libraries can become useful network surfaces.

## Ledger shape

```text
chao_0x network
├─ account chain / wallet node A
├─ account chain / wallet node B
├─ account chain / wallet node C
└─ ...
```

Each wallet owns its own ordered chain of signed blocks. The global network is the lattice formed by all account chains plus the cross-account references between them.

## Primary loops

### Creator loop

```text
point hub at folder
→ AI prepares
→ user confirms mode
→ account chain records release/index event
→ network publishes/indexes
→ asset earns
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
Native chao_0x ledger = block-lattice economy, assets, rewards, staking, burns
Wallet/account chain  = user's own node/state chain
Nostr                 = discovery, signed events, store/listing metadata
IPFS                  = content addressing / CIDs
Arweave               = permanent public artifacts
Filecoin              = storage-market/deal layer
BYOCloud              = encrypted fallback mirrors
WebRTC                = live peer delivery
Local AI              = formatting, tagging, indexing, risk scoring, metadata
Android               = local hub shell + foreground service + WebView mini-app runtime
Rust core             = hashing, signatures, manifests, proof engine, ledger adapters
External chains       = optional bridge/export layers, not the base network
```

## MVP target

The first MVP should prove the native data/index loop, not the entire sovereign OS.

```text
Android app
+ Rust proof/index/ledger core
+ local folder scanner
+ music + readable text indexer
+ manifest generator
+ Nostr event publisher
+ native account-chain event log
+ basic dashboard
```

See `/docs` for the system map and protocol notes.
