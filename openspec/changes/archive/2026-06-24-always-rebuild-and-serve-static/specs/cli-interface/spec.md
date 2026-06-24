## REMOVED Requirements

### Requirement: Reload flag
**Reason**: Always rebuild on every run makes the --reload flag unnecessary. Caching is completely removed.
**Migration**: Remove --reload flag from CLI. All pages are rebuilt on every run automatically.

### Requirement: Default reload
**Reason**: Always rebuild on every run. No caching means no "existing HTML" to serve.
**Migration**: System now always rebuilds. No action needed.

## MODIFIED Requirements

### Requirement: Output directory structure
The system SHALL organize output files in `~/.opens/md-viewer-data/`. The cache/ subdirectory is no longer used.

#### Scenario: Pages directory
- **WHEN** system generates HTML
- **THEN** files are saved to pages/ subdirectory

#### Scenario: Base directory
- **WHEN** system extracts assets
- **THEN** files are saved to base/ subdirectory

#### Scenario: Directory creation
- **WHEN** output directories do not exist
- **THEN** system creates them recursively
