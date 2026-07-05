# Data Asset Protocol

## Thesis

A Data Asset is not automatically a copyright claim.

A Data Asset is a structured network object that can be:

- indexed
- discovered
- routed
- hosted
- mirrored
- streamed
- licensed
- tipped
- staked
- challenged
- burned
- traded
- versioned
- remixed
- referenced by mini-apps

## Core asset classes

```text
ReleaseAsset
IndexAsset
CurationAsset
MirrorReceipt
SeederReceipt
StorageCommitment
MiniAppAsset
StaticSiteAsset
MusicReleaseAsset
FanworkIndexAsset
DatasetAsset
ModelAsset
GameAsset
BountyAsset
RemixLineageAsset
```

## Asset modes

```text
Creator Release
- User made it and intentionally publishes it.

Index-Only
- Hub stores metadata/tags/links, not the full content.

Mirror Allowed
- Content can be stored/rehosted.

Public Domain / Open License
- Safe for archive/remix/monetization depending on terms.

Fanwork / Transformative
- Readable/indexable/taggable, but underlying canon is not claimed.

Curator Asset
- The valuable object is the collection, shelf, tag map, or recommendation route.

Storage Receipt
- Proof someone preserved or served a file.

Seeder Receipt
- Proof someone delivered content successfully.
```

## Release anchor vs ownership claim

For chao_0x, an NFT-style asset anchor should mean:

```text
This identity released/indexed/curated/hosted this data object,
with this content hash,
this manifest,
this mode,
this storage policy,
and this economic behavior.
```

It should not automatically mean:

```text
I own all copyright in this underlying work.
```
