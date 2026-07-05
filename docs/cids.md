# CIDs

CID means **Content Identifier**.

A normal web URL points to a location:

```text
https://some-server.com/song.mp3
```

A CID points to content by cryptographic identity:

```text
ipfs://bafy...
```

Meaning:

> Find the object whose content matches this identifier, no matter which peer stores it.

## chao_0x identifier types

A single asset may have several identifiers:

```text
Raw file hash       = exact byte identity
CID                 = IPFS/content-addressed storage pointer
Fingerprint         = fuzzy identity for same song/text across formats
Manifest CID        = content-addressed asset metadata
Asset ID            = chain/Nostr/protocol identity
```

Example:

```text
MySong.mp3
├─ sha256: exact file bytes
├─ audio_fingerprint: same recording across encodings
├─ cid: IPFS object
├─ manifest_cid: asset manifest
└─ chao_asset_id: release/index/economic identity
```
