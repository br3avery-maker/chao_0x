# Music Flow

## Goal

A creator can point the hub at a directory of songs, grant access, and the hub prepares those files for network discovery, streaming, seeding, and rewards.

## Flow

```text
User points hub at /Music
        ↓
Hub scans files
        ↓
Hashes + fingerprints audio
        ↓
AI tags genre/mood/style
        ↓
Risk layer checks obvious matches
        ↓
User confirms release/index mode
        ↓
Manifest generated
        ↓
Optional MusicReleaseAsset minted
        ↓
Nostr store listing published
        ↓
Peers stream/seed/mirror
        ↓
Creator, seeders, curators earn
```

## Asset categories

```text
MusicReleaseAsset
PlaylistAsset
CuratorShelf
SeederReceipt
GhostTrackBounty
MirrorReceipt
RemixLineageAsset
```

## UI discovery modes

```text
Search
- direct lookup by title, creator, tag, mood, hash, CID

Radar
- what the network is playing/seeding now

Crate
- user's local music library and earning surface

Bounties
- requested tracks, creator drops, preservation requests
```
