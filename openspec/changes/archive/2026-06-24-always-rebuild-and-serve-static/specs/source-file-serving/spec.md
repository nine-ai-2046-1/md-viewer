## ADDED Requirements

### Requirement: Serve source directory files via HTTP
The system SHALL serve files from the `--mds` source directory via a `/source/*` HTTP route, allowing images, videos, PDFs, and other assets referenced in markdown to load in the browser.

#### Scenario: Serve image from source root
- **WHEN** client requests GET /source/photo.png
- **AND** the file photo.png exists in the source directory root
- **THEN** server returns the file with correct image MIME type

#### Scenario: Serve image from nested subdirectory
- **WHEN** client requests GET /source/sub/images/diagram.png
- **AND** the file exists at {source_dir}/sub/images/diagram.png
- **THEN** server returns the file with correct MIME type

#### Scenario: Serve video file
- **WHEN** client requests GET /source/clip.mp4
- **AND** the file exists in the source directory
- **THEN** server returns the file with Content-Type: video/mp4

#### Scenario: Path traversal blocked
- **WHEN** client requests GET /source/../../../etc/passwd
- **THEN** server returns 403 Forbidden

#### Scenario: Hidden file blocked
- **WHEN** client requests GET /source/.hidden/file
- **THEN** server returns 403 Forbidden

#### Scenario: Non-existent file returns 404
- **WHEN** client requests GET /source/missing.png
- **AND** the file does not exist in the source directory
- **THEN** server returns 404 Not Found

### Requirement: Source directory configured via ServerConfig
The system SHALL accept an optional `source_dir` field in `ServerConfig` that specifies the source directory for `/source/*` routing.

#### Scenario: Source dir provided
- **WHEN** ServerConfig includes a source_dir path
- **THEN** /source/* routes are active and serve files from that directory

#### Scenario: Source dir not provided
- **WHEN** ServerConfig source_dir is None
- **THEN** /source/* routes return 404 Not Found
