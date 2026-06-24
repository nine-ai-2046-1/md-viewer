## Context

md-viewer is a self-contained Rust CLI that converts markdown/YAML files to HTML and serves them via a localhost HTTP server. Currently:

- **Caching**: An mtime-based cache (`cache.rs`) stores source file modification times. Without `--reload`, existing HTML is served without checking for source changes. With `--reload`, only changed files are rebuilt. This causes stale content when users forget `--reload`.
- **Static assets**: The server only routes `/base/*` (CSS/JS) and `/{name}.html` (pages). Images, videos, and other files referenced in markdown via relative paths have no route and return 404.
- **Markdown links**: `pulldown-cmark` renders `[text](./other.md)` as `<a href="./other.md">`. The browser navigates to `/other.md` which the server cannot resolve to the corresponding `/other.html` page.

## Goals / Non-Goals

**Goals:**
- Always rebuild HTML on every run (no caching, no stale content)
- Serve images, videos, PDFs, and any other files from the `--mds` source directory
- Make markdown cross-references (`.md` links) resolve to rendered `.html` pages
- Maintain security: path traversal prevention, localhost-only binding

**Non-Goals:**
- Hot-reload / live-reload during server operation (out of scope)
- Optimizing rebuild performance for very large document sets
- Changing the `--build` static export mode

## Decisions

### D1: Delete `cache.rs` entirely rather than making cache optional

**Choice**: Remove the module completely.

**Rationale**: The cache adds complexity (mtime tracking, JSON persistence, asset hash detection) for a feature that causes user confusion. Always rebuilding is simpler and the performance cost is negligible for typical use cases (tens to hundreds of markdown files).

**Alternative considered**: Keep cache but make it opt-in via `--no-cache`. Rejected because it adds a new flag to solve a problem introduced by the old flag.

### D2: Add `/source/*` route instead of rewriting image paths

**Choice**: Serve source files at `/source/{relative-path}` and keep markdown image paths as-is.

**Rationale**: Markdown `![alt](./img.png)` produces `<img src="./img.png">`. When the page is at `/sub/page.html`, the browser resolves `./img.png` to `/sub/img.png`. This doesn't map to `/source/sub/img.png` directly. However, since generated pages are at the root level (e.g., `/sub/page.html` → page served from `pages/sub/page.html`), relative paths from the HTML page resolve correctly to `/source/...` only if we place the source directory映射 correctly.

**Actually**: After analysis, the simpler approach is to NOT use a `/source/` prefix. Instead, when the server can't find a path in pages_dir or base_dir, it falls back to checking the source_dir. This way relative paths like `./img.png` from `/sub/page.html` resolve to `/sub/img.png` → source_dir/sub/img.png. No URL rewriting needed for images.

**Revised approach**: Add a fallback route that checks source_dir for any unmatched path. Security: must be within source_dir, reject traversal.

### D3: Rewrite `.md` links in HTML output (builder-side) not server-side

**Choice**: Post-process rendered HTML with regex to replace `href="...md"` with `href="...html"`.

**Rationale**: This is simpler than server-side redirect logic. The regex `href="([^"]*\.md)"` catches all anchor hrefs pointing to `.md` files. Only relative `.md` links are rewritten; external URLs and anchors are unaffected.

**Alternative considered**: Server-side redirect (`302`) for `.md` requests. Rejected because it requires the server to know the mapping between `.md` filenames and generated `.html` names, which adds complexity.

### D4: Use `mime_guess` for source file MIME types

**Choice**: Use the existing `mime_guess` crate (already in Cargo.toml) for MIME type detection on source files.

**Rationale**: Already a dependency, handles more types than manual mapping, and is battle-tested.

## Risks / Trade-offs

- **[Performance]** Always rebuilding may be slower for large document sets. → Mitigation: Acceptable for typical use. If needed in future, can add parallel rendering.
- **[Security]** Serving source directory files increases attack surface. → Mitigation: Path traversal checks, confinement within source_dir, reject hidden files.
- **[Breaking change]** Removing `--reload` flag breaks existing scripts. → Mitigation: The flag was mainly a workaround for the stale content problem this change eliminates.
- **[Link rewriting]** Regex-based `.md` → `.html` replacement could miss edge cases (e.g., `.md` in query strings). → Mitigation: The regex targets only `href` attributes, which is the standard markdown link output.
