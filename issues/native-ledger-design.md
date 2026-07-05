# Native Ledger Design Task

This file mirrors the GitHub issue direction in repo form.

## Goal

Define the first native chao_0x account-chain / block-lattice ledger.

## Questions

- What is the exact account address format?
- What signature scheme should account chains use?
- What block hash canonicalization is stable enough for consensus?
- How do send/receive blocks settle?
- How do asset-release blocks reference manifests?
- How do index-commitment blocks get challenged?
- How do forks/conflicts get resolved?
- Is voting representative-based, stake-weighted, reputation-weighted, or hybrid?
- How does useful indexing work create issuance/rewards?

## Acceptance criteria

- Native block schema is reviewed.
- `chao-core` can hash native blocks deterministically.
- Account-chain append rules are documented.
- Conflict/fork detection is specified.
- External-chain bridges are explicitly non-canonical.
