# chao_0x System Map

## One-sentence architecture

**chao_0x is a local-first AI-indexed data economy hub that turns user-controlled folders, readable archives, music, sites, mini-apps, datasets, and models into discoverable network assets, using Nostr for metadata/discovery, decentralized storage and peer delivery for distribution, Base/EVM for the first economy layer, Minima as a sovereign-node branch, and built-in earn/spend/burn/stake mechanics across the whole framework.**

## High-level stack

```text
User Data Layer
music, fic, sites, apps, datasets, models, archives
        ↓
Local AI / Indexing Layer
tags, summaries, formatting, embeddings, risk scoring
        ↓
Manifest Layer
hashes, metadata, mode, storage, rights/risk, economic hooks
        ↓
Data Asset Layer
release anchors, index assets, curation assets, receipts
        ↓
Discovery / Gossip Layer
Nostr events, relays, store listings, reputation signals
        ↓
Storage / Delivery Layer
IPFS, Arweave, Filecoin, WebRTC, BYOCloud, local peers
        ↓
Economy Layer
earn, spend, burn, stake, tip, slash, challenge
        ↓
App / Hub UI Layer
Android shell, WebView mini-apps, dappstore, wallet approvals
```

## Core correction

Do not model this as "algorithmic mining."

Model it as:

```text
Useful network work
= indexing + routing + hosting + curating + proving availability
```

The network rewards nodes for maintaining useful knowledge and availability, not for wasting compute.
