## Why

The current CLI always starts an HTTP server after building, coupling the build step with serving. Users need a way to generate a portable static site (e.g., for CI/CD, GitHub Pages, or offline distribution) and to serve an already-built folder independently — matching common static-site tooling patterns.

## What Changes

- Add `--build <OUTPUT_DIR>` flag: converts `--md` / `--mds` source(s) to a self-contained static HTML site in the given directory, then exits — no server started.
- Add `--serve <STATIC_DIR>` flag: starts the localhost-only HTTP server to serve an existing static folder. Does not accept `--md` / `--mds`. Requires `index.html` to be present in the target directory; rejects execution otherwise.
- `--serve` accepts `--port` (same auto-increment behaviour as today).
- `--build` and `--serve` are mutually exclusive with each other and with the existing `--md` / `--mds` server modes.
- The output of `--build` is designed to be directly consumable by `--serve`.

## Capabilities

### New Capabilities

- `static-build`: Build mode — generate a self-contained static HTML site (HTML pages + all assets copied) to a user-specified output directory, then exit. No web server is started.
- `static-serve`: Serve mode — start a localhost-only HTTP server that serves files from a pre-built static directory. Validates `index.html` exists; rejects otherwise. No building or file watching occurs.

### Modified Capabilities

## Impact

- `src/main.rs` — new CLI flags `--build` / `--serve`; new mode dispatch functions `run_build_mode()` and `run_serve_mode()`
- `src/server.rs` — refactor `ServerConfig` so the serve root is configurable; `--serve` uses the user-supplied directory directly instead of the internal `pages/` + `base/` split
- `src/assets.rs` — new `copy_assets_to()` helper that copies the embedded assets into the build output directory
- `src/builder.rs` — no structural changes; reused as-is by build mode
- No new crate dependencies required
