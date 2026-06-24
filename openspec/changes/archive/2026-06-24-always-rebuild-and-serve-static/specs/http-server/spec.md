## MODIFIED Requirements

### Requirement: Static file serving
The system SHALL serve HTML files, base assets, and source directory files without directory browsing.

#### Scenario: Serve index page
- **WHEN** client requests GET /
- **THEN** server returns pages/index.html (folder mode) or pages/{name}.html (single mode)

#### Scenario: Serve HTML file
- **WHEN** client requests GET /{filename}
- **THEN** server returns pages/{filename}.html with text/html content type

#### Scenario: Serve base assets
- **WHEN** client requests GET /base/{asset}
- **THEN** server returns base/{asset} with correct MIME type

#### Scenario: Serve source files
- **WHEN** client requests GET /source/{path}
- **AND** source_dir is configured in ServerConfig
- **THEN** server returns the file from source_dir/{path} with correct MIME type

#### Scenario: Source dir not configured
- **WHEN** client requests GET /source/{path}
- **AND** source_dir is None
- **THEN** server returns 404 Not Found

#### Scenario: Directory listing blocked
- **WHEN** client requests GET /pages/
- **THEN** server returns 403 Forbidden

#### Scenario: Path traversal blocked
- **WHEN** client requests GET /../etc/passwd
- **THEN** server returns 403 Forbidden

#### Scenario: Unknown path
- **WHEN** client requests GET /unknown
- **THEN** server returns 404 Not Found

## ADDED Requirements

### Requirement: ServerConfig includes source directory
The system SHALL include an optional `source_dir` field in `ServerConfig` for serving source directory files.

#### Scenario: Source dir present
- **WHEN** ServerConfig has source_dir set to Some(path)
- **THEN** /source/* routes serve files from that directory

#### Scenario: Source dir absent
- **WHEN** ServerConfig has source_dir set to None
- **THEN** /source/* routes return 404
