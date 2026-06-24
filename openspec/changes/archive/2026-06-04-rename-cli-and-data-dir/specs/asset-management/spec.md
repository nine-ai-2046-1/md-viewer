## MODIFIED Requirements

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
