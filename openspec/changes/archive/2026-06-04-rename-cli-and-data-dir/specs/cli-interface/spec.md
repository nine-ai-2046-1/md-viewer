## MODIFIED Requirements

### Requirement: CLI argument parsing
The system SHALL parse CLI arguments using clap with derive macros.

#### Scenario: Single file mode
- **WHEN** user runs `md-viewer --md file.md`
- **THEN** system processes single file

#### Scenario: Folder mode
- **WHEN** user runs `md-viewer --mds directory/`
- **THEN** system processes all markdown files in directory recursively

#### Scenario: Port specification
- **WHEN** user runs `md-viewer --md file.md --port 9141`
- **THEN** server binds to port 9141

#### Scenario: Reload flag
- **WHEN** user runs `md-viewer --md file.md --reload`
- **THEN** system checks mtime and rebuilds if changed

#### Scenario: Default port
- **WHEN** user runs `md-viewer --md file.md` without --port
- **THEN** system uses default port 9141

#### Scenario: Default reload
- **WHEN** user runs `md-viewer --md file.md` without --reload
- **THEN** system serves existing HTML without mtime checks

### Requirement: Output directory structure
The system SHALL organize output files in `~/.opens/md-viewer-data/`.

#### Scenario: Pages directory
- **WHEN** system generates HTML
- **THEN** files are saved to pages/ subdirectory

#### Scenario: Base directory
- **WHEN** system extracts assets
- **THEN** files are saved to base/ subdirectory

#### Scenario: Cache directory
- **WHEN** system caches mtimes
- **THEN** file is saved to cache/ subdirectory

#### Scenario: Directory creation
- **WHEN** output directories do not exist
- **THEN** system creates them recursively
