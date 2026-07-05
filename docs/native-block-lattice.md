# Native chao_0x Block-Lattice

## Correction

chao_0x is not an EVM-first system.

The intended network is a native chao_0x ledger where each user wallet/account owns its own chain and participates as a node.

```text
wallet = account = source chain = node identity
```

External chains may exist later as bridges, but they are not the base economy layer.

## Ledger model

chao_0x should use an account-chain / block-lattice design:

```text
Network ledger
├─ account chain A
├─ account chain B
├─ account chain C
├─ account chain D
└─ ...
```

Each account maintains its own ordered sequence of blocks.

A block belongs to one account chain and updates that account's state.

## Why this fits chao_0x

The data hub already assumes local ownership and local activity:

- user-controlled folders
- local AI indexes
- encrypted local index bundles
- Nostr/store discovery
- storage and bandwidth proofs
- creator releases
- curation receipts
- seeding receipts

Those should not be forced into a global EVM contract first.

They should be native block types on each user's account chain.

## Block families

```text
Value blocks
- send
- receive
- stake
- burn
- reward claim

Data asset blocks
- release asset
- update manifest
- deprecate version
- link storage route

Index blocks
- publish index root
- update index commitment
- answer challenge

Availability blocks
- seed claim
- mirror receipt
- delivery receipt

Curation blocks
- shelf/list/playlist
- tag assertion
- recommendation graph

App blocks
- mini-app publish
- install receipt
- permission grant/revoke

Governance / trust blocks
- representative delegation
- challenge
- vote
- slash / dispute outcome
```

## Send / receive split

For value transfer, use a two-sided model:

```text
sender account chain:
  send block → reduces sender balance and references receiver

receiver account chain:
  receive block → accepts pending incoming amount
```

This keeps each account chain locally append-only and lets the receiver control when funds/assets are accepted.

## Assets in the lattice

A Data Asset is not only an NFT on an external chain.

A native asset is a release/index/curation/storage object recorded in the publisher's account chain.

```text
MusicReleaseAsset
FanworkIndexAsset
StaticSiteAsset
MiniAppAsset
DatasetAsset
ModelAsset
SeederReceipt
MirrorReceipt
CurationAsset
BountyAsset
```

External NFT bridges can be added later, but the native asset record is canonical inside chao_0x.

## Consensus direction

The open design question is how conflicts are resolved.

A block-lattice needs a rule for contradictory account blocks, such as two blocks claiming the same previous block.

Potential direction:

```text
account chains are locally signed
conflicting forks trigger representative / quorum / stake-weighted voting
index and availability claims are challengeable
rewards depend on successful proofs, not blind publication
```

This resembles the broad family of account-chain/block-lattice systems, but chao_0x should be designed around data availability, indexing, and asset routing rather than only payments.

## Mining replacement

Native chao_0x issuance should not come from algorithmic waste.

It should come from useful work:

```text
Proof-of-Index
Proof-of-Availability
Proof-of-Routing
Proof-of-Curation
Proof-of-Delivery
Proof-of-Challenge
```

Rewards are native ledger events, not ERC-20 emissions.

## Bridge position

External chains are adapters:

```text
Ethereum / Base / EVM = optional bridge/export layer
Solana = optional bridge/export layer
Minima = inspiration / possible interoperability branch
Nostr = discovery and event gossip, not consensus
IPFS/Arweave/Filecoin = storage layers, not canonical ledger
```

## One-line correction

```text
chao_0x is a native account-chain/block-lattice data economy network where every wallet is its own node, and useful data-indexing work replaces wasteful mining.
```
