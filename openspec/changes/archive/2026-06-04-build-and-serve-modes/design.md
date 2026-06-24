## Context

`markdown-viewer` currently always couples build and serve: every run converts sources to HTML and immediately starts an HTTP server. Users who want to generate a portable static site for deployment (CI/CD, GitHub Pages, Docker, shared folder) have no way to get the output without also starting a server. Conversely, users with an already-built site cannot use the tool as a lightweight static file server without re-triggering a build.

The existing server in `server.rs` is tightly bound to the internal cache layout (`base_dir` = `~/.opens/…/base`, `pages_dir` = `~/.opens/…/pages`). Making it serve an arbitrary directory requires a small refactor of `ServerConfig`.

## Goals / Non-Goals

**Goals:**
- `--build <dir>`: produce a self-contained static site in a user-chosen directory, then exit — no server started
- `--serve <dir>`: start the localhost-only HTTP server against a pre-built directory; validate `index.html` presence before binding; no build step
- Output of `--build` is directly consumable by `--serve`
- Backward-compatible: existing `--md` / `--mds` behaviour unchanged

**Non-Goals:**
- File watching / live reload in `--serve` mode
- Incremental build cache when using `--build` (cache still lives in `~/.opens/…/cache/`)
- CDN deployment helpers or relative-path asset rewriting
- `--build` without `--md` / `--mds` (no source, no output)

## Decisions

### D1 — `--build` as modifier, not standalone mode

`--build <dir>` is an output-destination modifier used alongside `--md` or `--mds`. When present it redirects HTML output to `<dir>/` and suppresses server start. Without `--build`, behaviour is unchanged (output to internal cache + server starts).

*Alternatives considered:*
- A separate `build` sub-command: more discoverable but breaks the existing flag style and would require refactoring clap setup significantly.
- Always build to a temp dir then serve: hides the output; not useful for CI use cases.

### D2 — Output structure mirrors internal cache layout

```
<output_dir>/
  index.html        ← always generated
  <page>.html       ← rendered pages
  base/
    style.css
    app.js
    prism.js
    prism.css
    fuse.js
    sidebar.js
```

Assets use server-relative `/base/…` paths — the site requires an HTTP server (not `file://`). This is intentional: the tool is a viewer, not an offline bundle.

For `--md --build`: a minimal `index.html` redirect page pointing to the single page is generated so `--serve` can be used immediately on the output.

*Alternatives considered:*
- Inline all assets into each HTML file: makes files self-contained for `file://` but produces very large files and violates DRY.
- Relative paths (`./base/…`): works for `file://` but breaks when served under a sub-path.

### D3 — `--serve` reuses existing `ServerConfig` with root-dir parameterisation

`ServerConfig` gains a `root_dir: PathBuf` field. `base_dir` and `pages_dir` are derived from it (`root_dir/base` and `root_dir` respectively). For `--serve`, the root is the user-supplied directory. For the existing server modes, the root is `~/.opens/…/`.

Page names for `--serve` are discovered by scanning `root_dir` for `*.html` files (excluding `base/`). This avoids a separate inventory file.

*Alternatives considered:*
- Two separate `ServerConfig` variants: more type-safe but adds complexity with no practical benefit at this scale.
- Require a manifest file in the build output: adds coupling between `--build` and `--serve`; the directory scan is simpler and sufficient.

### D4 — `copy_assets_to(target: &Path)` in `assets.rs`

A new function extracts the embedded assets (from `include_dir!`) directly to `target/base/`. This is distinct from `ensure_assets()` which writes to the internal cache. Both use the same embedded `DIR` constant.

### D5 — `--serve` validates `index.html` before binding

If `<dir>/index.html` is absent, the process exits with a clear error message before binding to the port. This prevents silently serving a broken site.

## Risks / Trade-offs

- **Stale build output** — `--serve` serves whatever is in the directory; if the user re-runs `--build` concurrently with `--serve`, they may see partial state. Mitigation: document that `--build` should complete before `--serve` starts; atomic directory swap is out of scope.
- **`/base/…` paths require HTTP** — users expecting `file://` access will be confused. Mitigation: document clearly in `--help` and error output.
- **Page discovery scan in `--serve`** — scanning `root_dir` for `*.html` on every server start is acceptable at this scale (tens of files); not suitable for directories with thousands of files. Mitigation: acceptable given the tool's target use case.
