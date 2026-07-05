# Native Ledger Plan

This folder is a placeholder for the native chao_0x ledger design.

## Goal

chao_0x should be its own account-chain / block-lattice network.

```text
wallet = account = account chain = node identity
```

## Suggested native modules

```text
chao-ledger
- account-chain block types
- block validation
- fork/conflict detection
- send/receive state transitions

chao-token
- native CHAO balances
- burns
- reward claims
- staking balances

chao-assets
- release assets
- index assets
- curation assets
- mini-app assets
- storage and seeder receipts

chao-rewards
- proof-of-index rewards
- proof-of-availability rewards
- proof-of-routing rewards
- proof-of-curation rewards
- delivery receipts

chao-challenges
- challenge creation
- challenge responses
- dispute outcome blocks
- slashing/reputation hooks

chao-bridges
- optional later bridges to EVM/Base/Solana/etc.
```

## First block types

```text
open_account
send
receive
burn
stake
unstake
release_asset
update_asset_manifest
publish_index_root
availability_claim
curation_claim
delivery_receipt
challenge
challenge_response
reward_claim
representative_vote
```

## Design rule

External contracts are not the source of truth. Native account-chain blocks are the source of truth.
