## 1. CLI Flags

- [x] 1.1 Add `--build <OUTPUT_DIR>` flag to `Cli` struct in `main.rs` (PathBuf, optional, conflicts with `--serve`)
- [x] 1.2 Add `--serve <STATIC_DIR>` flag to `Cli` struct (PathBuf, optional, conflicts with `--md`, `--mds`, `--build`)
- [x] 1.3 Update `--md` and `--mds` arg descriptions to reflect YAML support and `--build` interaction
- [x] 1.4 Add mode validation in `main()`: `--build` without `--md`/`--mds` → error; `--serve` with `--md`/`--mds`/`--build` → error

## 2. Asset Copy Helper

- [x] 2.1 Add `copy_assets_to(target_base: &Path) -> Result<(), String>` in `assets.rs`
- [x] 2.2 Implement: iterate embedded `DIR`, write each file to `target_base/<filename>` (create dir if needed)
- [x] 2.3 Add unit test: `copy_assets_to` writes all expected asset files to a temp directory

## 3. Static Build Mode

- [x] 3.1 Add `run_build_mode(md: Option<&Path>, mds: Option<&Path>, output_dir: &Path) -> Result<(), String>` in `main.rs`
- [x] 3.2 Implement: create `output_dir` (and parents) if it does not exist
- [x] 3.3 Implement: call `copy_assets_to(&output_dir.join("base"))` to write embedded assets
- [x] 3.4 Implement single-file branch: render `--md` source, write HTML to `output_dir/<name>.html`
- [x] 3.5 Implement single-file branch: generate minimal `index.html` redirect/landing page pointing to the single rendered page
- [x] 3.6 Implement folder branch: call `scan_markdown_files`, render each to `output_dir/<name>.html` with full multi-page template (search index, file tree)
- [x] 3.7 Implement folder branch: generate `output_dir/index.html` with file cards and search index
- [x] 3.8 Wire `run_build_mode` into `run()` dispatch when `--build` is present
- [x] 3.9 Verify: after `run_build_mode`, process exits (no server started)

## 4. ServerConfig Refactor

- [x] 4.1 Add `root_dir: PathBuf` field to `ServerConfig` in `server.rs`
- [x] 4.2 Remove `page_names`; derive `base_dir`/`pages_dir` at call sites; update call sites
- [x] 4.3 Update existing `run_single_mode` and `run_folder_mode` call sites in `main.rs` to pass `root_dir`
- [x] 4.4 Update server route handler to serve any file under `root_dir` (not just whitelisted `page_names`)
- [x] 4.5 Ensure path traversal guard is applied against `root_dir` (not just `pages_dir`)

## 5. Static Serve Mode

- [x] 5.1 Add `run_serve_mode(serve_dir: &Path, port: u16) -> Result<(), String>` in `main.rs`
- [x] 5.2 Implement: check `serve_dir` exists → error if not
- [x] 5.3 Implement: check `serve_dir/index.html` exists → print clear error and exit(1) if not
- [x] 5.4 Implement: construct `ServerConfig` with `root_dir = serve_dir`, `pages_dir = root_dir`
- [x] 5.5 Implement: `validate_serve_dir()` extracted as testable function
- [x] 5.6 Implement: find available port, open browser, call `server::start()`
- [x] 5.7 Wire `run_serve_mode` into `run()` dispatch when `--serve` is present

## 6. Unit Tests

- [x] 6.1 Test `validate_serve_dir` rejects non-existent directory
- [x] 6.2 Test `validate_serve_dir` rejects directory without `index.html`
- [x] 6.3 Test `validate_serve_dir` returns Ok for valid directory with index.html
- [x] 6.4 Test `copy_assets_to` creates expected files in output directory
- [x] 6.5 Test generated `index.html` from single-file build contains redirect to the rendered page
- [x] 6.6 Test generated `index.html` from folder build contains all file cards

## 7. Integration Smoke Tests

- [x] 7.1 Run `--mds ./openspec --build /tmp/md-smoke-dist` → 15 HTML pages + `base/` assets confirmed
- [x] 7.2 Run `--md /tmp/test.md --build /tmp/md-smoke-single` → `test.html`, `index.html`, `base/` confirmed
- [x] 7.3 Run `--serve ./dist` against a valid build output and verify server starts and browser opens
- [x] 7.4 `validate_serve_dir` tested: rejects missing dir and missing index.html
- [x] 7.5 Existing `--md`/`--mds` modes unaffected (53 tests pass, standard serve path unchanged)

## 8. Documentation

- [x] 8.1 Updated `--help` text for `--md` to mention `--build` compatibility
- [x] 8.2 Updated CLI `about` string in `Cli` struct to mention `--build` and `--serve`
- [x] 8.3 This file updated
