# Account Chain

Each wallet has its own signed history.

Core fields:

- account
- height
- previous
- kind
- payload
- signature

The first block opens the account. Later blocks point to the prior block for that account.

The native ledger is the set of all account histories plus cross-account references.

A release asset is recorded as a native ledger event. An index update is also recorded as a native ledger event.

External chains are not the source of truth for these records.
