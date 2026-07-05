# Correction Log

## 2026-07-05 — Native block-lattice correction

Earlier scaffold language described Base/EVM as the first economy layer.

That was wrong.

Corrected architecture:

```text
chao_0x is its own native account-chain / block-lattice network.
Each wallet is its own node/account chain.
External chains are optional bridge/export layers only.
```

Files corrected:

- `README.md`
- `ROADMAP.md`
- `docs/system-map.md`
- `docs/protocol-stack.md`
- `docs/native-block-lattice.md`
- `ledger/native/README.md`
- `schemas/chao.native_block.v0.schema.json`
- `examples/native-release-block.json`
- `crates/chao-core/src/lib.rs`
- `bridges/README.md`

Old direction removed:

- `contracts/base/README_ONLY/contracts-plan.md`

Remaining cleanup:

- Some older example release manifests may still include external-chain-shaped identity fields and should be revised after the native manifest schema stabilizes.
