## Why

Markdown images use relative paths (e.g., `![alt](screenshots/photo.png)`), but the server only serves source files under the `/source/*` route. When the browser renders the HTML page at `/verification-item-01`, it resolves `screenshots/photo.png` to `/screenshots/photo.png` — which returns 404. The server only handles `/source/screenshots/photo.png`. This means **all images in markdown are broken** in `--mds` mode.

## What Changes

- Add `rewrite_image_paths()` function in `src/builder.rs` to post-process rendered HTML
- Rewrite `<img src="...">` attributes to prefix relative paths with `/source/`
- Only rewrite relative paths (skip absolute URLs, data: URIs, and paths already starting with `/source/`)
- Call `rewrite_image_paths()` in `render_body()` alongside existing `rewrite_md_links()`

## Capabilities

### New Capabilities

(None — this is a fix to existing behavior, not a new capability)

### Modified Capabilities

- `markdown-conversion`: Add post-processing step to rewrite relative image `src` attributes with `/source/` prefix so images load correctly from the source directory

## Impact

- **Modified files**: `src/builder.rs` (add `rewrite_image_paths()` function, call it in `render_body()`)
- **No dependencies**: Uses existing regex infrastructure
- **No breaking changes**: Only affects relative image paths; absolute URLs and data URIs are untouched
