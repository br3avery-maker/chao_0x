# chao_0x System Map

## One-sentence architecture

**chao_0x is a local-first AI-indexed data economy network using a native account-chain / block-lattice ledger where every wallet is its own node, turning user-controlled folders, readable archives, music, sites, mini-apps, datasets, and models into discoverable native data assets with built-in earn/spend/burn/stake/challenge mechanics.**

## High-level stack

```text
User Wallet / Node Layer
one wallet = one account chain = one local node identity
        ↓
User Data Layer
music, fic, sites, apps, datasets, models, archives
        ↓
Local AI / Indexing Layer
tags, summaries, formatting, embeddings, risk scoring
        ↓
Manifest Layer
hashes, metadata, mode, storage, rights/risk, economic hooks
        ↓
Native Block-Lattice Layer
account-chain blocks, asset blocks, index blocks, reward blocks
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
native earn, spend, burn, stake, tip, slash, challenge
        ↓
App / Hub UI Layer
Android shell, WebView mini-apps, dappstore, wallet approvals
```

## Core correction

Do not model this as "algorithmic mining" or as an EVM-first project.

Model it as:

```text
Native block-lattice ledger
+ useful network work
= account-chain ownership + indexing + routing + hosting + curating + proving availability
```

The network rewards nodes for maintaining useful knowledge and availability, not for wasting compute or routing every primitive through external contracts.
