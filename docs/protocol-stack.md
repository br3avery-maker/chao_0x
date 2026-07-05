# Protocol Stack

## Nostr

Role: signed discovery and metadata gossip.

Use Nostr for:

- asset announcements
- index commitments
- peer availability claims
- music/readable-data listings
- mini-app manifests
- curation feeds
- bounty notices
- proof receipts
- reputation signals

Do not use Nostr for:

- full file storage
- large app bundles
- raw streaming payloads
- consensus
- the primary token economy

## Base / EVM

Role: first public economy layer.

Use Base/EVM for:

- CHAO token
- NFT/release anchors
- staking
- burn mechanics
- liquidity pools
- reward contracts
- mini-app fees
- bounties
- creator drops

## Minima

Role: sovereign-node branch.

Use Minima for:

- mobile full-node experiments
- MiniDapp compatibility
- P2P value transfer
- Maxima messaging
- local sovereign mode

Minima should not carry the entire system at MVP stage.

## IPFS / Arweave / Filecoin / BYOCloud

- IPFS: content addressing and CIDs.
- Arweave: permanent public artifacts.
- Filecoin: storage-provider market/deal layer.
- BYOCloud: encrypted fallback mirrors using user-owned cloud storage.
- Local peers: live availability and direct serving.

## WebRTC / libp2p

Role: live peer delivery.

Use for:

- file chunk serving
- music streaming
- peer routes
- bandwidth rewards
- local swarm transfer
