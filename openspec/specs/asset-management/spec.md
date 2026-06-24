## Purpose

The asset management module handles extraction, validation, and caching of embedded assets (CSS, JS, HTML templates) required for rendering markdown pages.

## Requirements

### Requirement: Embedded asset extraction
The system SHALL extract embedded assets to `~/.opens/md-viewer-data/base/` on first run.

#### Scenario: First run extraction
- **WHEN** base directory does not exist or is incomplete
- **THEN** system extracts all embedded assets to base/

#### Scenario: Existing valid assets
- **WHEN** base directory exists with all required files
- **THEN** system skips extraction

#### Scenario: Corrupted assets
- **WHEN** base directory exists but files are missing or corrupted
- **THEN** system re-extracts all assets

### Requirement: Asset validation
The system SHALL validate base assets before serving.

#### Scenario: All assets present
- **WHEN** base directory contains style.css, app.js, prism.css, prism.js, sidebar.js, fuse.js, single.html, multi.html
- **THEN** system considers assets valid

#### Scenario: Missing asset detected
- **WHEN** base directory is missing any required file
- **THEN** system re-extracts all assets

### Requirement: Mtime cache management
The system SHALL cache markdown file modification times for rebuild optimization.

#### Scenario: Cache creation
- **WHEN** system builds HTML from markdown
- **THEN** system stores source file mtime in cache/mtime.json

#### Scenario: Cache hit
- **WHEN** source file mtime matches cached mtime
- **THEN** system skips rebuild for that file

#### Scenario: Cache miss
- **WHEN** source file mtime differs from cached mtime
- **THEN** system rebuilds HTML and updates cache

### Requirement: Filename sanitization
The system SHALL sanitize markdown filenames for URL safety.

#### Scenario: Lowercase conversion
- **WHEN** markdown filename contains uppercase letters
- **THEN** HTML filename is lowercase

#### Scenario: Space to hyphen
- **WHEN** markdown filename contains spaces
- **THEN** HTML filename replaces spaces with hyphens

#### Scenario: Special character removal
- **WHEN** markdown filename contains special characters (except hyphens)
- **THEN** HTML filename removes them, keeping only a-z, 0-9, hyphens

#### Scenario: Multiple hyphens collapsed
- **WHEN** markdown filename results in multiple consecutive hyphens
- **THEN** HTML filename collapses them to single hyphen

#### Scenario: Leading/trailing hyphens trimmed
- **WHEN** markdown filename results in leading/trailing hyphens
- **THEN** HTML filename trims them

#### Scenario: Example transformations
- **WHEN** input is "My Notes.md"
- **THEN** output is "my-notes.html"

- **WHEN** input is "API Reference (v2).md"
- **THEN** output is "api-reference-v2.html"

- **WHEN** input is "hello_world.md"
- **THEN** output is "hello-world.html"
