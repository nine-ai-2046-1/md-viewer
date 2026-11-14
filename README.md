# md-viewer

A self-contained CLI tool that renders Markdown and YAML files as beautiful HTML pages, served from a localhost HTTP server. Inspired by Docusaurus.

## Features

- **Single file** or **folder** mode — view one file or an entire directory
- **Static build** — export a self-contained site to disk (`--build`)
- **Static serve** — serve a pre-built directory without rebuilding (`--serve`)
- **Dark / light theme** toggle, persisted in `localStorage`
- **Collapsible sidebar** with full file tree and fuzzy search (Fuse.js)
- **Toggleable TOC** panel on the right, with scroll spy
- **Syntax highlighting** via Prism.js 1.30.0
- **Mtime cache** — only rebuild files that have changed (`--reload`)
- **Localhost-only** — binds to `127.0.0.1`; path traversal prevention; security headers
- Single self-contained binary (~1.5 MB); no runtime dependencies

## Installation

### macOS — download a prebuilt binary (no clone, no Rust)

| Your Mac | Download |
|---|---|
| Apple Silicon (M1/M2/M3/M4) | [`md-viewer-0.1.0-aarch64-apple-darwin.tar.gz`](https://github.com/nine-ai-2046-1/md-viewer/releases/download/v0.1.0/md-viewer-0.1.0-aarch64-apple-darwin.tar.gz) |
| Intel | [`md-viewer-0.1.0-x86_64-apple-darwin.tar.gz`](https://github.com/nine-ai-2046-1/md-viewer/releases/download/v0.1.0/md-viewer-0.1.0-x86_64-apple-darwin.tar.gz) |

Not sure which one? Run `uname -m`: `arm64` = Apple Silicon, `x86_64` = Intel.

All releases: https://github.com/nine-ai-2046-1/md-viewer/releases

```bash
# Apple Silicon (swap the filename for Intel)
curl -L -o md-viewer.tar.gz \
  https://github.com/nine-ai-2046-1/md-viewer/releases/download/v0.1.0/md-viewer-0.1.0-aarch64-apple-darwin.tar.gz

tar -xzf md-viewer.tar.gz
cd md-viewer-0.1.0-aarch64-apple-darwin
./md-viewer --help
```

Each release also ships a `.sha256` sidecar so you can verify the archive:

```bash
shasum -a 256 -c md-viewer-0.1.0-aarch64-apple-darwin.tar.gz.sha256
```

**If macOS refuses to open it:** the binary is unsigned, so browser-downloaded copies get a quarantine flag. Clear it (no admin rights needed):

```bash
xattr -d com.apple.quarantine ./md-viewer
```

Downloading with `curl` avoids the flag entirely. Building from source also produces an unflagged binary.

Optional — put it on your `PATH`:

```bash
mkdir -p ~/bin && mv md-viewer ~/bin/
```

### Linux / Windows

No prebuilt binaries are published for these platforms — build from source (see below), or open an issue if you want them.

### Build from source (any platform)

Requirements: Rust 1.70+.

```bash
git clone https://github.com/nine-ai-2046-1/md-viewer
cd md-viewer
cargo build --release --locked
# Binary: target/release/md-viewer
```

To install system-wide:

```bash
cp target/release/md-viewer /usr/local/bin/
```

## Usage

### View a single file

```bash
md-viewer --md README.md
```

Automatically opens `http://127.0.0.1:9141` in your default browser (`open` on macOS, `xdg-open` on Linux).

### View a folder of files

```bash
md-viewer --mds ./docs
```

Renders all `.md`, `.yaml`, and `.yml` files recursively. Generates an index page with file cards and a searchable sidebar.

### Custom port

```bash
md-viewer --mds ./docs --port 8080
```

If the port is occupied, the next available port is used automatically.

### Only rebuild changed files (`--reload`)

```bash
md-viewer --mds ./docs --reload
```

Compares source file modification times against a cache. Only files modified since the last run are re-rendered. Ideal for large documentation sets.

> **Note:** `--reload` only applies to standard serve mode (`--md` / `--mds`). The `--build` command always performs a full rebuild of all files regardless of this flag.

### Export a static site (`--build`)

```bash
# Single file
md-viewer --md README.md --build ./dist

# Folder
md-viewer --mds ./docs --build ./dist
```

Renders all sources to `./dist/`, writes assets to `./dist/base/`, and exits. The output requires an HTTP server (not `file://`).

### Serve a pre-built directory (`--serve`)

```bash
md-viewer --serve ./dist
```

Starts a localhost HTTP server against a directory previously built with `--build`. Does not perform any rendering.

### Build-then-serve workflow

```bash
md-viewer --mds ./docs --build ./dist
md-viewer --serve ./dist
```

## CLI Reference

```
Usage: md-viewer [OPTIONS]

Options:
      --md <FILE>           Single markdown or YAML file to view (.md / .yaml / .yml).
                            Use with --build to export to a static site instead of serving
      --mds <DIR>           Directory of markdown and YAML files to view (recursive scan).
                            Use with --build to export to a static site instead of serving
      --build <OUTPUT_DIR>  Build output directory: render sources to a self-contained
                            static site and exit. Requires --md or --mds. Does not start
                            a web server
      --serve <STATIC_DIR>  Serve a pre-built static directory over localhost HTTP.
                            Requires index.html to exist. Does not build anything
      --port <PORT>         Port to serve on. Auto-increments if occupied [default: 9141]
      --reload              Check source file modification times; rebuild only changed
                            files. Without this flag, existing HTML is served without
                            rebuilding
  -h, --help                Print help
  -V, --version             Print version
```

### Flag Compatibility

| Flag | `--md` | `--mds` | `--build` | `--serve` | `--port` | `--reload` |
|------|--------|---------|-----------|-----------|----------|------------|
| `--md` | — | ✗ | ✓ | ✗ | ✓ | ✓ |
| `--mds` | ✗ | — | ✓ | ✗ | ✓ | ✓ |
| `--build` | ✓ | ✓ | — | ✗ | ✗ | ✗ |
| `--serve` | ✗ | ✗ | ✗ | — | ✓ | ✗ |

## File Support

| Extension | Rendering |
|-----------|-----------|
| `.md`     | Markdown (CommonMark + tables + task lists + syntax highlighting) |
| `.yaml` / `.yml` | Pretty-printed YAML with syntax highlighting |

## Filename Sanitization

Source filenames are sanitized into URL-safe HTML names automatically:

| Rule | Example |
|------|---------|
| Lowercase | `README.md` → `readme.html` |
| Spaces → hyphens | `My Notes.md` → `my-notes.html` |
| Underscores → hyphens | `api_reference.md` → `api-reference.html` |
| Special chars removed | `Config (v2).md` → `config-v2.html` |
| Collapse multiple hyphens | `a - b.md` → `a-b.html` |
| Trim leading/trailing hyphens | `-hello-.md` → `hello.html` |

Only characters `[a-z0-9-]` are kept. The resulting name is also the URL path: `/my-notes` or `/my-notes.html`.

## Data Directories

| Path | Purpose |
|------|---------|
| `~/.opens/md-viewer-data/base/` | Extracted CSS/JS assets |
| `~/.opens/md-viewer-data/pages/` | Rendered HTML pages (serve mode) |
| `~/.opens/md-viewer-data/cache/mtime.json` | Mtime rebuild cache |
| `~/.opens/md-viewer-data/cache/asset-hash` | Binary asset fingerprint |

Assets are always re-extracted on startup. If the binary is updated, the asset hash changes and the mtime cache is cleared automatically — no manual `--reload` required.

## Upgrading / Data Migration

The data directory changed from `~/.opens/markdown-viewer/` to `~/.opens/md-viewer-data/` in this release.

The old directory is **not** touched or deleted by the new binary. Cached pages and mtime history are fully regenerable — simply run `md-viewer` once and the new directory will be created automatically.

If you want to preserve existing cached data, move it manually:

```bash
mv ~/.opens/markdown-viewer ~/.opens/md-viewer-data
```

## Security

- Binds exclusively to `127.0.0.1` (never `0.0.0.0`)
- Path traversal attempts return HTTP 400
- Directory listing is disabled
- All responses include `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, and a Content-Security-Policy header

## Development

```bash
# Run tests
cargo test

# Debug build (fast iteration)
cargo build

# Release build (optimised, ~1.5 MB)
cargo build --release
```

### Project Structure

```
src/
  main.rs       — CLI, mode dispatch, build orchestration
  server.rs     — HTTP server, routing, security
  builder.rs    — Markdown/YAML rendering, file scanning, TOC
  assets.rs     — Embedded asset extraction, hash detection
  templates.rs  — Template rendering, TOC HTML, file cards
  cache.rs      — Mtime-based rebuild cache
  sanitize.rs   — Filename sanitization
  port.rs       — Port availability and auto-increment
assets/
  style.css     — Docusaurus-inspired stylesheet
  app.js        — Theme, sidebar, TOC, search, scroll spy
  sidebar.js    — File tree component
  multi.html    — Folder mode page template
  single.html   — Single file page template
  prism.js/css  — Syntax highlighting (Prism.js 1.30.0)
  fuse.js       — Client-side fuzzy search (Fuse.js 6.6.2)
  [nav.js]      — Generated at runtime; contains FILE_TREE + SEARCH_INDEX
                  shared across all pages; written to base/ on every run
```
