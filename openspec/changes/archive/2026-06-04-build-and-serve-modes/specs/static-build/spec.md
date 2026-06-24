## ADDED Requirements

### Requirement: Build mode produces a self-contained static site
The system SHALL, when `--build <output_dir>` is provided alongside `--md` or `--mds`, render all source files to HTML and copy all embedded assets into `<output_dir>`, then exit without starting an HTTP server.

#### Scenario: Build folder mode to output directory
- **WHEN** user runs `markdown-viewer --mds ./docs --build ./dist`
- **THEN** `./dist/` is created if it does not exist
- **THEN** each markdown and YAML file is rendered to `./dist/<name>.html`
- **THEN** embedded assets are written to `./dist/base/` (style.css, app.js, prism.js, prism.css, fuse.js, sidebar.js)
- **THEN** `./dist/index.html` is generated with file cards and search index
- **THEN** the process exits with code 0

#### Scenario: Build single file mode to output directory
- **WHEN** user runs `markdown-viewer --md README.md --build ./dist`
- **THEN** `./dist/readme.html` is generated
- **THEN** embedded assets are written to `./dist/base/`
- **THEN** `./dist/index.html` is generated as a redirect/landing page pointing to `readme.html`
- **THEN** the process exits with code 0

#### Scenario: Build output directory is created if absent
- **WHEN** `<output_dir>` does not exist
- **THEN** the system creates it (including any missing parent directories)
- **THEN** the build proceeds normally

#### Scenario: Build rejects missing source flag
- **WHEN** user runs `markdown-viewer --build ./dist` without `--md` or `--mds`
- **THEN** the system prints an error: "Error: --build requires --md or --mds"
- **THEN** the process exits with code 1

### Requirement: Build output is consumable by serve mode
The directory produced by `--build` SHALL have the structure expected by `--serve`, so that `markdown-viewer --serve <output_dir>` works immediately after a build.

#### Scenario: Build then serve roundtrip
- **WHEN** user runs `markdown-viewer --mds ./docs --build ./dist`
- **THEN** `./dist/index.html` exists
- **WHEN** user then runs `markdown-viewer --serve ./dist`
- **THEN** the server starts successfully and serves the site

### Requirement: Build flag conflicts with serve flag
The `--build` flag SHALL be mutually exclusive with `--serve`.

#### Scenario: Build and serve flags used together
- **WHEN** user provides both `--build <dir>` and `--serve <dir>`
- **THEN** the system prints an error indicating the flags are mutually exclusive
- **THEN** the process exits with code 1

### Requirement: Build respects --reload flag
When `--reload` is provided alongside `--build`, the system SHALL apply mtime-based cache checks, rebuilding only files that have changed since the last build.

#### Scenario: Unchanged file skipped with --reload
- **WHEN** `--build` is run twice with `--reload` and source files have not changed
- **THEN** unchanged files are not re-rendered on the second run
- **THEN** "Using existing <name>.html" is printed for each skipped file
