## 1. Implementation

- [x] 1.1 Add `rewrite_image_paths()` function in `src/builder.rs` using regex
- [x] 1.2 Call `rewrite_image_paths()` in `render_body()` after `rewrite_md_links()`
- [x] 1.3 Add unit tests for `rewrite_image_paths()` in `src/builder.rs`

## 2. Verification

- [x] 2.1 Run `cargo build` to verify compilation
- [x] 2.2 Run `cargo test` to verify all tests pass
- [x] 2.3 Manual test: `md-viewer --mds ~/Desktop/01` and verify images load in browser
