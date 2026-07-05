# Nostr Event Sketches

These are draft event shapes. Final kinds should be chosen carefully to avoid conflicts and align with NIP conventions.

## Data asset announcement

```json
{
  "kind": 33340,
  "content": "",
  "tags": [
    ["type", "chao_data_asset"],
    ["asset_type", "music_release"],
    ["manifest_cid", "ipfs://bafy..."],
    ["chain", "base"],
    ["contract", "0x..."],
    ["token_id", "17"],
    ["sha256", "..."],
    ["risk", "low"]
  ]
}
```

## Index commitment

```json
{
  "kind": 33341,
  "content": "",
  "tags": [
    ["type", "chao_index_commitment"],
    ["index_root", "0x..."],
    ["entry_count", "18472"],
    ["updated_at", "1783290000"],
    ["index_mode", "private_commitment"]
  ]
}
```

## Availability claim

```json
{
  "kind": 33342,
  "content": "",
  "tags": [
    ["type", "chao_availability_claim"],
    ["cid", "ipfs://bafy..."],
    ["asset_id", "base:0x...:17"],
    ["availability", "foreground_service"],
    ["transport", "webrtc"]
  ]
}
```

## Delivery receipt

```json
{
  "kind": 33343,
  "content": "",
  "tags": [
    ["type", "chao_delivery_receipt"],
    ["asset_id", "base:0x...:17"],
    ["cid", "ipfs://bafy..."],
    ["provider", "npub..."],
    ["consumer", "npub..."],
    ["bytes", "1048576"]
  ]
}
```
