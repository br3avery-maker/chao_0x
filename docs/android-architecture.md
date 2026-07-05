# Android Architecture

## Recommended MVP stack

```text
Kotlin/Java Android shell
+ Rust core
+ WebView UI
+ JNI or UniFFI bridge
+ foreground service
+ Nostr/WebRTC/storage adapters
```

## Why

Kotlin/Java should own:

- Android permissions
- MediaStore / file access
- notifications
- foreground service lifecycle
- wallet approval screens
- WebView container
- OS integration

Rust should own:

- hashing
- signatures
- proof engine
- local database
- manifest generation
- index commitments
- protocol adapters
- cryptographic verification

Web UI should own:

- dashboard
- music player
- readable library
- dappstore
- asset manager
- wallet
- bounties
- creator tools

## Correction

Do not call it an invisible unkillable daemon.

Use:

```text
user-visible foreground swarm service
```
