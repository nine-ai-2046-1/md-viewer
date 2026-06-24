## 1. Project Setup

- [x] 1.1 Create new Rust project with cargo init
- [x] 1.2 Add dependencies to Cargo.toml (clap, pulldown-cmark, tiny_http, include_dir, mime_guess, serde, serde_json, chrono)
- [x] 1.3 Create src/ directory structure (main.rs, builder.rs, server.rs, assets.rs, templates.rs, port.rs, sanitize.rs, cache.rs)
- [x] 1.4 Create assets/ directory with placeholder files

## 2. CLI Interface

- [x] 2.1 Implement clap-based CLI with --md, --mds, --port, --reload flags
- [x] 2.2 Implement mode exclusivity check (--md and --mds cannot be used together)
- [x] 2.3 Implement default values (--port 9141, --reload false)
- [x] 2.4 Implement error handling for missing mode flag

## 3. Filename Sanitization

- [x] 3.1 Implement sanitize.rs with lowercase conversion
- [x] 3.2 Implement space-to-hyphen conversion
- [x] 3.3 Implement special character removal (keep only a-z, 0-9, hyphens)
- [x] 3.4 Implement multiple hyphen collapsing
- [x] 3.5 Implement leading/trailing hyphen trimming
- [x] 3.6 Add unit tests for sanitize module

## 4. Asset Management

- [x] 4.1 Implement asset extraction using include_dir
- [x] 4.2 Implement base directory validation
- [x] 4.3 Implement asset re-extraction on corruption
- [x] 4.4 Create embedded CSS (style.css) with Docusaurus-inspired design
- [x] 4.5 Create embedded JS (app.js) with theme toggle, TOC, navigation
- [x] 4.6 Create embedded Prism.js (prism.css, prism.js) for syntax highlighting
- [x] 4.7 Create embedded sidebar.js for file tree component
- [x] 4.8 Create embedded fuse.js for client-side search
- [x] 4.9 Create embedded HTML templates (single.html, multi.html)

## 5. Mtime Cache

- [x] 5.1 Implement cache.rs with JSON serialization
- [x] 5.2 Implement mtime storage for source files
- [x] 5.3 Implement mtime comparison for rebuild detection
- [x] 5.4 Implement --reload flag logic
- [x] 5.5 Add unit tests for cache module

## 6. Markdown Conversion

- [x] 6.1 Implement pulldown-cmark parser integration
- [x] 6.2 Implement HTML generation with proper doctype
- [x] 6.3 Implement syntax highlighting with Prism.js
- [x] 6.4 Implement table support
- [x] 6.5 Implement task list support
- [x] 6.6 Implement TOC generation from headings
- [x] 6.7 Add unit tests for builder module

## 7. Template System

- [x] 7.1 Implement template rendering with variable substitution
- [x] 7.2 Implement single file template (single.html)
- [x] 7.3 Implement folder template (multi.html)
- [x] 7.4 Implement index page generation for folder mode
- [x] 7.5 Implement theme support (data-theme attribute)
- [x] 7.6 Implement responsive design (mobile/desktop)

## 8. Folder Scanning

- [x] 8.1 Implement recursive directory scanning
- [x] 8.2 Implement .md file filtering
- [x] 8.3 Implement file tree generation for sidebar
- [x] 8.4 Implement empty directory error handling
- [x] 8.5 Implement non-existent directory error handling

## 9. HTTP Server

- [x] 9.1 Implement tiny_http server setup
- [x] 9.2 Implement localhost-only binding (127.0.0.1)
- [x] 9.3 Implement route handling (/, /base/*, /{filename})
- [x] 9.4 Implement security headers (X-Content-Type-Options, X-Frame-Options, CSP)
- [x] 9.5 Implement path traversal prevention
- [x] 9.6 Implement directory listing blocking
- [x] 9.7 Implement MIME type detection for assets

## 10. Port Finding

- [x] 10.1 Implement port availability check
- [x] 10.2 Implement auto port finding (increment on EADDRINUSE)
- [x] 10.3 Implement max attempt limit (20)
- [x] 10.4 Implement error handling for all ports occupied

## 11. Browser Auto-open

- [x] 11.1 Implement platform detection (macOS/Linux)
- [x] 11.2 Implement macOS browser open (open command)
- [x] 11.3 Implement Linux browser open (xdg-open command)
- [x] 11.4 Implement error handling for browser open failure

## 12. Client Search

- [x] 12.1 Implement fuse.js integration
- [x] 12.2 Implement search index generation (filename, content, headings)
- [x] 12.3 Implement search input UI
- [x] 12.4 Implement search results dropdown
- [x] 12.5 Implement fuzzy search with ranking
- [x] 12.6 Implement no results message

## 13. Integration Testing

- [x] 13.1 Test single file mode end-to-end
- [x] 13.2 Test folder mode end-to-end
- [x] 13.3 Test reload flag behavior
- [x] 13.4 Test port auto-finding
- [x] 13.5 Test security (path traversal, directory listing) — unit tests in server.rs
- [x] 13.6 Test theme toggle and persistence
- [x] 13.7 Test sidebar collapse and persistence
- [x] 13.8 Test search functionality

## 14. Release Preparation

- [x] 14.1 Build release binary with LTO
- [x] 14.2 Verify binary size (1.5MB — within budget)
- [x] 14.3 Create README.md with usage instructions
- [x] 14.4 Test on clean system (no Rust installed)
