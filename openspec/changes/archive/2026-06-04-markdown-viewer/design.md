## Context

This is a new Rust CLI project that converts markdown files to beautiful, interactive HTML pages and serves them via a localhost-only HTTP server. The tool targets developers who need instant markdown preview without installing Node.js or other runtimes.

Current state: Empty workspace, fresh project.

Stakeholders: Individual developers who work with markdown documentation.

## Goals / Non-Goals

**Goals:**
- Self-contained binary that works without external dependencies
- Docusaurus-inspired UI with dark/light theme, collapsible sidebar, syntax highlighting
- Client-side search across filenames and content
- Localhost-only security with no directory browsing
- Auto port finding and browser open
- Mtime-based rebuild caching

**Non-Goals:**
- Multi-user or network-accessible server
- Custom template/theme support (embedded only in v1)
- Real-time file watching (use --reload flag instead)
- Plugin system or extensibility
- Windows primary support (macOS/Linux focus)

## Decisions

### 1. Language: Rust

**Decision:** Use Rust for the CLI binary.

**Rationale:** Self-contained binary, fast startup, no runtime dependencies, good ecosystem for CLI tools.

**Alternatives considered:**
- Go: Also self-contained, but Rust has better compile-time asset embedding
- Node.js: Requires runtime, not self-contained

### 2. HTTP Server: tiny_http

**Decision:** Use tiny_http for the HTTP server.

**Rationale:** Minimal dependency, sync (no async runtime overhead), sufficient for static file serving.

**Alternatives considered:**
- axum: Overkill, requires tokio runtime
- warp: Filter-based, more complex than needed
- std::net::TcpListener: Zero dependencies but requires manual HTTP parsing

### 3. Markdown Parser: pulldown-cmark

**Decision:** Use pulldown-cmark for markdown-to-HTML conversion.

**Rationale:** Most popular Rust markdown parser, CommonMark compliant, streaming parser, good performance.

**Alternatives considered:**
- comrak: GitHub-flavored Markdown support, but more complex
- markdown: Lightweight but limited features

### 4. Asset Embedding: include_dir

**Decision:** Use include_dir crate to embed assets at compile time.

**Rationale:** Single binary, no external files needed at runtime, assets extracted to ~/.opens/markdown-viewer/base/ on first run.

**Alternatives considered:**
- include_str!: Would need to embed each file separately
- Manual embedding: More control but more code

### 5. UI Framework: Vanilla HTML/CSS/JS

**Decision:** Use vanilla HTML/CSS/JS for the frontend (Docusaurus-inspired).

**Rationale:** No build pipeline needed, small file sizes, easy to embed, no React/Vue overhead.

**Alternatives considered:**
- React/Vue: Would require build tooling, larger bundle size
- Tailwind CSS: Would require build pipeline

### 6. Syntax Highlighting: Prism.js 2.0

**Decision:** Use Prism.js 2.0 stable for syntax highlighting.

**Rationale:** Mature, supports many languages, small footprint, no dependencies.

**Alternatives considered:**
- Highlight.js: Similar features but larger
- Tree-sitter: More powerful but complex integration

### 7. Search: fuse.js

**Decision:** Use fuse.js for client-side search.

**Rationale:** Lightweight, fuzzy search, no server-side indexing needed, searches filename + content.

**Alternatives considered:**
- Lunr.js: More complex setup
- FlexSearch: Different API style

### 8. Filename Sanitization

**Decision:** Convert filenames to lowercase, replace spaces with hyphens, keep only a-z0-9 and hyphens.

**Rationale:** URL-safe, consistent naming, avoids platform-specific issues.

**Rule:** `[^a-z0-9-]` → remove, collapse multiple hyphens, trim leading/trailing hyphens.

### 9. Port Finding

**Decision:** Auto-find next available port starting from user-specified or default 9141.

**Rationale:** Avoid port conflicts, better UX than failing on port in use.

**Algorithm:** Try bind, if EADDRINUSE increment port (max 20 attempts).

### 10. Base Path

**Decision:** Use `$HOME/.opens/markdown-viewer/base/` as default base path.

**Rationale:** Consistent with user's existing .opens directory structure.

## Risks / Trade-offs

**[Risk] Binary size increase** → Embedded assets add ~90KB, total binary ~2-3MB. Acceptable for CLI tool.

**[Risk] No real-time reload** → Users must restart or use --reload flag. Trade-off for simplicity.

**[Risk] Limited customization** → Embedded templates only. Acceptable for v1, can add customization later.

**[Risk] Platform-specific issues** → macOS/Linux focus. Windows may have path handling differences. Mitigate with std::path.

**[Risk] Security: localhost only** → No remote access. By design, but users may want LAN access later.

**[Trade-off] Sync vs Async server** → Chose sync (tiny_http) for simplicity. May limit concurrent connections, but acceptable for single-user local viewing.

**[Trade-off] No directory browsing** → Security feature, but users can't browse pages/ directory. Mitigated by index page in folder mode.

## Migration Plan

N/A - new project, no migration needed.

## Open Questions

None - all design decisions confirmed during exploration phase.
