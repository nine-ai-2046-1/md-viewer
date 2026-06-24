## 1. Remove Caching

- [x] 1.1 Delete `src/cache.rs` file
- [x] 1.2 Remove `mod cache;` from `src/main.rs`
- [x] 1.3 Remove `--reload` CLI flag from Cli struct in `src/main.rs`
- [x] 1.4 Remove mtime cache load/save logic from `run()` in `src/main.rs`
- [x] 1.5 Remove asset hash detection logic from `run()` in `src/main.rs`
- [x] 1.6 Update `run_single_mode()` to always rebuild (remove cache parameter and conditional)
- [x] 1.7 Update `run_folder_mode()` to always rebuild (remove cache parameter and conditional)
- [x] 1.8 Remove cache-related unit tests from `src/main.rs`

## 2. Add Source File Serving

- [x] 2.1 Add `source_dir: Option<PathBuf>` field to `ServerConfig` in `src/server.rs`
- [x] 2.2 Implement `serve_source_file()` function in `src/server.rs`
- [x] 2.3 Add `/source/*` route to `route()` function in `src/server.rs`
- [x] 2.4 Pass source_dir from `run_folder_mode()` to `ServerConfig` in `src/main.rs`
- [x] 2.5 Set source_dir to None for single-file mode and --serve mode in `src/main.rs`

## 3. Rewrite .md Hyperlinks

- [x] 3.1 Add `rewrite_md_links()` function in `src/builder.rs` using regex
- [x] 3.2 Call `rewrite_md_links()` in `render_body()` after `inject_heading_ids()` in `src/builder.rs`
- [x] 3.3 Add unit tests for `rewrite_md_links()` in `src/builder.rs`

## 4. Verification

- [x] 4.1 Run `cargo build` to verify compilation
- [x] 4.2 Run `cargo test` to verify all tests pass
- [x] 4.3 Manual test: `md-viewer --mds <folder-with-images>` and verify images load
- [x] 4.4 Manual test: click .md hyperlink and verify it resolves to .html page
