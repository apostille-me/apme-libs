# Architecture

`apme-libs` contains reusable Apostille Me domain, document-status, provider-routing, serialization, and API contract helpers.

## Canonical package boundary

- `apme-interfaces` owns wire formats and generated contract types.
- `apme-libs` consumes interfaces and owns reusable, runtime-light behavior.
- `apme-clients` exposes versioned SDKs built on the interface contracts.
- `apme-sync` owns offline-first reconciliation.
- API, web, and CLI repositories compose these packages rather than copying their source.

The long `apostille-me-libs` repository is a historical bootstrap alias, not a package source. Its generic two-field `Record` scaffold is intentionally not migrated because it duplicates neither the canonical domain model nor production behavior.

## Zed and Git submodules

Use `apostille-me/apme-libs` as the only Zed coordinate. A retained Git submodule must have an explicit editable-workspace, inventory, embedded-source, experiment-reference, or legacy role; do not resolve the same repository through both Zed and a gitlink in one composition.

A root `.zpkg.toml` allows `zed overtake --git-submodules` to adopt an exact gitlink while preserving `.gitmodules` and the pinned commit.
