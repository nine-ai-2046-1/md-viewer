## ADDED Requirements

### Requirement: CLI argument parsing
The system SHALL parse CLI arguments using clap with derive macros.

#### Scenario: Single file mode
- **WHEN** user runs `markdown-viewer --md file.md`
- **THEN** system processes single file

#### Scenario: Folder mode
- **WHEN** user runs `markdown-viewer --mds directory/`
- **THEN** system processes all markdown files in directory recursively

#### Scenario: Port specification
- **WHEN** user runs `markdown-viewer --md file.md --port 9141`
- **THEN** server binds to port 9141

#### Scenario: Reload flag
- **WHEN** user runs `markdown-viewer --md file.md --reload`
- **THEN** system checks mtime and rebuilds if changed

#### Scenario: Default port
- **WHEN** user runs `markdown-viewer --md file.md` without --port
- **THEN** system uses default port 9141

#### Scenario: Default reload
- **WHEN** user runs `markdown-viewer --md file.md` without --reload
- **THEN** system serves existing HTML without mtime checks

### Requirement: Mode exclusivity
The system SHALL enforce mutual exclusivity between --md and --mds flags.

#### Scenario: Both flags provided
- **WHEN** user runs `markdown-viewer --md file.md --mds directory/`
- **THEN** system exits with error message indicating flags are mutually exclusive

#### Scenario: No mode flag
- **WHEN** user runs `markdown-viewer` without --md or --mds
- **THEN** system exits with help message

### Requirement: Recursive directory scan
The system SHALL recursively scan directories for markdown files in folder mode.

#### Scenario: Single directory
- **WHEN** directory contains markdown files
- **THEN** system processes all .md files

#### Scenario: Nested directories
- **WHEN** directory contains subdirectories with markdown files
- **THEN** system processes all .md files recursively

#### Scenario: Empty directory
- **WHEN** directory contains no markdown files
- **THEN** system exits with error message

#### Scenario: Non-existent directory
- **WHEN** user specifies non-existent directory
- **THEN** system exits with error message

### Requirement: Output directory structure
The system SHALL organize output files in ~/.opens/markdown-viewer/.

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

### Requirement: Error handling
The system SHALL provide clear error messages for common failure scenarios.

#### Scenario: File not found
- **WHEN** user specifies non-existent markdown file
- **THEN** system exits with error message "File not found: {path}"

#### Scenario: Permission denied
- **WHEN** system cannot read source file
- **THEN** system exits with error message "Permission denied: {path}"

#### Scenario: Port in use
- **WHEN** all ports in range are occupied
- **THEN** system exits with error message listing attempted ports

#### Scenario: Base extraction failed
- **WHEN** system cannot write to base directory
- **THEN** system exits with error message explaining the issue
