## Why

When using `--mds` to serve a folder of markdown files, two problems exist:
1. **Stale content**: Without `--reload`, previously built HTML is served even if the source markdown changed. Users see a mix of old+new content because the browser caches stale pages.
2. **Broken images/links**: Markdown files often reference images, videos, and other files via relative paths. The server has no route to serve these files from the source directory, so all `<img>`, `<video>`, and relative `<a>` links are broken. Additionally, hyperlinks pointing to other `.md` files navigate to raw `.md` paths instead of the corresponding rendered `.html` pages.

## What Changes

- **Remove mtime caching**: Delete `cache.rs` module entirely. Always rebuild all HTML pages on every run, eliminating stale content issues.
- **Remove `--reload` flag**: Since we always rebuild, the flag becomes unnecessary.
- **Remove asset hash detection**: No longer needed without caching.
- **Add `/source/*` route**: Serve static files (images, videos, PDFs, etc.) directly from the `--mds` source directory, supporting arbitrary subdirectory depths.
- **Rewrite `.md` hyperlinks**: Post-process rendered HTML to convert `href="...md"` links to `href="...html"` so markdown cross-references resolve to the correct rendered pages.

## Capabilities

### New Capabilities

- `source-file-serving`: Serve arbitrary files from the --mds source directory via a /source/* HTTP route, enabling images, videos, and other assets referenced in markdown to load correctly.

### Modified Capabilities

- `cli-interface`: Remove `--reload` flag and mtime cache directory requirement. Always-rebuild becomes the default (and only) behavior.
- `http-server`: Add `/source/*` route for serving source directory files. Add `.md` → `.html` redirect fallback route.
- `markdown-conversion`: Add post-processing step to rewrite relative `.md` hyperlinks to `.html` in rendered output.

## Impact

- **Deleted files**: `src/cache.rs`
- **Modified files**: `src/main.rs` (remove cache logic, update ServerConfig), `src/server.rs` (add source_dir route, .md redirect), `src/builder.rs` (add link rewriting)
- **Breaking change**: `--reload` flag removed (CLI change)
- **Dependencies**: No new dependencies (uses existing `regex` and `mime_guess` crates)
