# chao_0x

**chao_0x** is a local-first decentralized data/economy hub.

Core thesis:

> Point the hub at useful user-controlled data → AI prepares it → the protocol classifies it → the network indexes it → storage/peers distribute it → assets can earn, burn, stake, trade, mirror, route value, or power mini-apps.

This repo is an early architecture scaffold for the chao_0x system.

## What chao_0x is

chao_0x is not only a token, wallet, NFT project, media player, storage layer, or mining app.

It is a **data asset network** where folders, archives, music, writing, static sites, mini-apps, datasets, models, and readable libraries can become useful network surfaces.

## Primary loops

### Creator loop

```text
point hub at folder
→ AI prepares
→ user confirms mode
→ network publishes/indexes
→ asset earns
```

### Indexer loop

```text
scan useful data
→ build local AI-enriched index
→ publish index commitment
→ answer peer challenges
→ earn for useful network state
```

### Seeder loop

```text
host useful data
→ serve chunks/routes
→ prove availability/delivery
→ earn rewards
```

### Curator loop

```text
collect/tag/recommend useful data
→ publish curation graph
→ earn from discovery/use
```

## Protocol split

```text
Nostr       = discovery, signed events, store/listing metadata
Base/EVM    = first economy layer: token, staking, NFTs, liquidity, rewards
Minima      = sovereign-node research branch
IPFS        = content addressing / CIDs
Arweave     = permanent public artifacts
Filecoin    = storage-market/deal layer
BYOCloud    = encrypted fallback mirrors
WebRTC      = live peer delivery
Local AI    = formatting, tagging, indexing, risk scoring, metadata
Android     = local hub shell + foreground service + WebView mini-app runtime
Rust core   = hashing, signatures, manifests, proof engine, adapters
```

## MVP target

The first MVP should prove the data loop, not the entire sovereign OS.

```text
Android app
+ Rust proof/index core
+ local folder scanner
+ music + readable text indexer
+ manifest generator
+ Nostr event publisher
+ Base testnet asset anchor
+ basic dashboard
```

See `/docs` for the system map and protocol notes.
