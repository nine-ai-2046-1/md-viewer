## Why

The current binary name `markdown-viewer` is long and inconsistent with the project directory name `md-viewer`. The data directory `~/.opens/markdown-viewer` embeds the old long name, making it awkward to type and inconsistent with the short form used everywhere else. Aligning both to shorter, consistent identifiers improves usability and reduces friction in daily use.

## What Changes

- **BREAKING** Rename the compiled binary from `markdown-viewer` to `md-viewer` (Cargo binary name in `Cargo.toml`)
- **BREAKING** Change the runtime data directory from `~/.opens/markdown-viewer/` to `~/.opens/md-viewer-data/`
- Update all internal references to the old binary name and data path (help text, error messages, comments, tests)
- Update `README.md` to reflect the new binary name and data directory path

## Capabilities

### New Capabilities

_None — this is a rename-only change; no new capabilities are introduced._

### Modified Capabilities

- `cli-interface`: CLI binary name changes from `markdown-viewer` to `md-viewer`; all `--help` output and usage examples are updated accordingly
- `asset-management`: Runtime base path changes from `~/.opens/markdown-viewer/` to `~/.opens/md-viewer-data/`; directory resolution logic updated

## Impact

- `Cargo.toml`: `[[bin]] name` field changes from `markdown-viewer` to `md-viewer`
- `src/assets.rs`: `resolve_root()` path hardcoded string `"markdown-viewer"` → `"md-viewer-data"`
- `src/main.rs`: CLI `name`, `about`, usage examples in doc comments updated
- `README.md`: All `markdown-viewer` command examples and data directory paths updated
- **Existing data** at `~/.opens/markdown-viewer/` is NOT migrated automatically; users must move it manually or let the tool recreate it
- Release binary artifact name changes (affects any scripts or CI that reference the binary)
