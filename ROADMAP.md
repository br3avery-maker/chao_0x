# Roadmap

## Phase 0 — Architecture

- Define native account-chain / block-lattice system map
- Define sovereign accounting kernel
- Define Data / Identity Kernel boundaries
- Define Reward / Proof Kernel boundaries
- Define OS service proof inputs
- Define data asset manifest
- Define index commitment model
- Define native block types
- Define Nostr event sketches
- Define Android/Rust/WebView architecture

## Phase 1 — Data / Identity Kernel

- account identity
- key/signature model
- native `chao_0x:` identifiers
- content hashing
- sovereign file IDs
- canonical manifests
- account-chain block structure
- schema validation
- deterministic block hashing
- basic append rules

## Phase 2 — Reward / Proof Kernel

- proof object model
- proof validation interfaces
- reward claim blocks
- stake blocks
- burn blocks
- slash blocks
- challenge blocks
- challenge response blocks
- proof-of-index primitive
- proof-of-storage primitive
- proof-of-uptime primitive
- proof-of-delivery primitive
- proof-of-curation primitive

## Phase 3 — OS Services MVP

- Android folder scanner
- music file indexing
- readable text indexing
- local AI metadata mock or basic model/API adapter
- local encrypted index
- service scheduler
- uptime measurement
- storage availability measurement
- bandwidth/delivery receipt input
- battery/thermal/network observation input

## Phase 4 — Local Data Hub MVP

- manifest generator
- local account-chain event log
- publish index root block
- release asset block
- availability claim block
- delivery receipt block
- basic contribution dashboard
- wallet/proof/reward dashboard

## Phase 5 — Network Verification

- peer pings for uptime
- random chunk challenges for storage
- route tests for availability
- delivery receipts
- representative vote or dispute placeholder
- consume/search Nostr event feed
- map Nostr events back to native account-chain blocks

## Phase 6 — Storage / Delivery

- IPFS upload
- Arweave publish option
- WebRTC chunk serving
- mirror receipts
- proof-of-availability challenge flow

## Phase 7 — Mini-App Runtime

- safe reward/proof APIs
- app manifest
- permission prompts
- WebView mini-app loader
- local dappstore
- app update policy
- mini-app blocks for publish/install/permission events

## Phase 8 — Bridge / Interop Later

- optional EVM/Base bridge research
- optional wrapped CHAO
- optional external NFT/export anchor
- optional Minima interoperability research
- optional Solana or other chain adapters
