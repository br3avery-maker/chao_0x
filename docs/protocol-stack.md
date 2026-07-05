# Protocol Stack

## Native chao_0x Block-Lattice

Role: canonical ledger, economy, native assets, staking, burns, rewards, and challenge outcomes.

chao_0x is intended to be its own network:

```text
wallet = account = account chain = node identity
```

Use the native ledger for:

- CHAO token/account balances
- send/receive blocks
- release anchors
- native data assets
- staking
- burn mechanics
- reward routing
- mini-app fees
- bounties
- curation receipts
- seeder receipts
- mirror receipts
- challenge/slash events

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

## External Chains

Role: optional bridge/export layer only.

External chains such as Ethereum, Base, Solana, or other networks may later support:

- wrapped CHAO
- exported release anchors
- bridge receipts
- marketplace mirrors
- liquidity outside the native network

They are not the base economy layer.

## Minima

Role: inspiration, comparison, or possible interoperability branch.

Use Minima research for:

- mobile-node lessons
- MiniDapp comparison
- P2P value-transfer comparison
- Maxima-style messaging ideas
- sovereign-device design references

Minima should not be treated as the canonical chao_0x ledger unless explicitly re-scoped.

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
