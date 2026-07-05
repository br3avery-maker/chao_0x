# Android App Plan

Recommended stack:

```text
Kotlin/Java Android shell
+ Rust chao-core
+ WebView UI
+ JNI/UniFFI bridge
+ foreground service
```

First Android modules:

```text
FolderPermissionManager
MediaIndexer
ReadableTextIndexer
ManifestGeneratorBridge
NostrPublisher
LocalIndexDb
ForegroundSwarmService
WalletApprovalActivity
WebViewMiniAppHost
```

Correction:

Use a user-visible foreground service for live swarm activity. Do not design around invisible background behavior.
