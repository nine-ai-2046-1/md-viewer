## Why

Developers need a fast, secure way to view markdown files as beautiful, interactive HTML pages locally. Existing solutions require installing Node.js, npm packages, or expose security risks through network access. A self-contained Rust CLI that works out-of-the-box provides instant markdown viewing with Docusaurus-inspired UI, localhost-only security, and zero external dependencies.

## What Changes

- New Rust CLI tool `markdown-viewer` with two modes:
  - Single file mode (`--md`): Convert and serve one markdown file
  - Folder mode (`--mds`): Recursively scan directory, generate index page, serve all files
- Self-contained binary with embedded assets (CSS, JS, templates)
- Docusaurus-inspired UI with dark/light theme, collapsible sidebar, syntax highlighting
- Client-side search using fuse.js (searches filename + content)
- Localhost-only HTTP server with auto port finding
- Automatic browser open on server start
- Mtime-based rebuild caching for fast subsequent runs
- Security: no directory browsing, path traversal prevention, response headers

## Capabilities

### New Capabilities

- `markdown-conversion`: Convert markdown files to HTML using pulldown-cmark with syntax highlighting
- `http-server`: Localhost-only HTTP server with static file serving and security controls
- `template-system`: Docusaurus-inspired HTML templates with theme toggle, TOC, sidebar
- `client-search`: Fuse.js-based client-side search across filenames and content
- `asset-management`: Embedded asset extraction and caching in ~/.opens/markdown-viewer/
- `cli-interface`: Clap-based CLI with --md, --mds, --port, --reload flags

### Modified Capabilities

<!-- None - this is a new project -->

## Impact

- New Rust project with dependencies: clap, pulldown-cmark, tiny_http, include_dir, mime_guess, serde, serde_json, chrono
- Assets directory: ~/.opens/markdown-viewer/ (base/, pages/, cache/)
- Binary size: ~2-3MB release with LTO
- Supported platforms: macOS, Linux (Windows possible but not primary target)
- Security: localhost binding only, no external network access
