## Context

The project currently ships as two identifiers that are inconsistent with each other:

- Binary name: `markdown-viewer` (long, verbose)
- Data directory: `~/.opens/markdown-viewer/` (inherits the long name)
- Project / repo directory: `md-viewer` (short form)

The binary name is defined in `Cargo.toml` (`[[bin]] name = "markdown-viewer"`) and the data path is hardcoded in `src/assets.rs` (`resolve_root()` → `~/.opens/markdown-viewer`). Every usage example, help string, doc comment, and README reference must also be updated.

## Goals / Non-Goals

**Goals:**
- Rename compiled binary from `markdown-viewer` → `md-viewer`
- Change runtime data directory from `~/.opens/markdown-viewer/` → `~/.opens/md-viewer-data/`
- Update all internal string literals, doc comments, and help text that reference the old names
- Update `README.md` examples and data directory table
- Keep the change mechanical and reviewable — no logic changes, no new features

**Non-Goals:**
- Automatic migration of existing `~/.opens/markdown-viewer/` data to the new path
- Versioned migration tooling or upgrade path scripts
- Changing the crate name (`name = "markdown-viewer"` in `[package]`) — only the `[[bin]]` name changes

## Decisions

### D1 — New data directory name: `md-viewer-data`

**Chosen:** `~/.opens/md-viewer-data/`

**Alternatives considered:**
- `~/.opens/md-viewer/` — mirrors the binary name exactly, but risks confusion with the project repo directory also named `md-viewer`
- `~/.opens/mdv/` — shortest option, but too opaque; not self-describing

**Rationale:** `md-viewer-data` is unambiguous, clearly separates runtime data from source, and avoids a name clash with the repo directory.

### D2 — No automatic data migration

**Chosen:** Users must manually move `~/.opens/markdown-viewer/` → `~/.opens/md-viewer-data/` if they want to preserve cached pages and mtime history.

**Rationale:** The data directory contains only derived artifacts (rendered HTML, mtime cache, asset hash). All of it can be recreated by running the tool once. A migration script adds complexity and edge cases (partial copy, permissions, symlinks) for negligible benefit. A clear note in the README is sufficient.

### D3 — Crate `[package] name` is NOT renamed

**Chosen:** `[package] name` stays `"markdown-viewer"`; only `[[bin]] name` changes.

**Rationale:** The crate name affects `cargo install` and the crate registry identity. Renaming it is a separate, higher-impact decision. The binary name is what users type on the command line, so that is the right scope for this change.

## Risks / Trade-offs

| Risk | Mitigation |
|------|-----------|
| Existing users lose cached pages and mtime history after upgrading | Document manual migration step in README; data is fully regenerable |
| CI or scripts that hardcode `markdown-viewer` binary path will break | This is a **BREAKING** change; called out in proposal and changelog |
| Forgetting to update a reference (comment, test string, help text) | Grep for all occurrences of `"markdown-viewer"` and `".opens/markdown-viewer"` before closing the PR |

## Migration Plan

1. Apply code changes (Cargo.toml, assets.rs, main.rs, README.md)
2. Run `cargo build` — binary produced at `target/debug/md-viewer`
3. Run `cargo test` — all tests must pass
4. Manually test: `./target/debug/md-viewer --md README.md`
   - Confirm new data dir `~/.opens/md-viewer-data/` is created
   - Confirm old dir `~/.opens/markdown-viewer/` is untouched
5. Update README with migration note for existing users
6. Tag release as breaking change

**Rollback:** Revert Cargo.toml `[[bin]] name` and `resolve_root()` string; old binary name and data path are restored immediately.

## Open Questions

_None — scope is fully defined and mechanical._
