## ADDED Requirements

### Requirement: Localhost-only binding
The system SHALL bind HTTP server to 127.0.0.1 only, rejecting any other network interface.

#### Scenario: Server binds to localhost
- **WHEN** server starts
- **THEN** it listens on 127.0.0.1:{port} only

#### Scenario: Reject 0.0.0.0 binding
- **WHEN** server attempts to bind to 0.0.0.0
- **THEN** system refuses and logs error

#### Scenario: Reject IPv6 binding
- **WHEN** server attempts to bind to [::]
- **THEN** system refuses and logs error

### Requirement: Auto port finding
The system SHALL automatically find available port if specified port is occupied.

#### Scenario: Port available
- **WHEN** user specifies port and it is available
- **THEN** server binds to that port

#### Scenario: Port occupied
- **WHEN** user specifies port and it is occupied
- **THEN** system tries next port (increment by 1) up to 20 attempts

#### Scenario: All ports occupied
- **WHEN** system tries 20 consecutive ports and all are occupied
- **THEN** system exits with error message listing attempted ports

### Requirement: Static file serving
The system SHALL serve HTML files and base assets without directory browsing.

#### Scenario: Serve index page
- **WHEN** client requests GET /
- **THEN** server returns pages/index.html (folder mode) or pages/{name}.html (single mode)

#### Scenario: Serve HTML file
- **WHEN** client requests GET /{filename}
- **THEN** server returns pages/{filename}.html with text/html content type

#### Scenario: Serve base assets
- **WHEN** client requests GET /base/{asset}
- **THEN** server returns base/{asset} with correct MIME type

#### Scenario: Directory listing blocked
- **WHEN** client requests GET /pages/
- **THEN** server returns 403 Forbidden

#### Scenario: Path traversal blocked
- **WHEN** client requests GET /../etc/passwd
- **THEN** server returns 403 Forbidden

#### Scenario: Unknown path
- **WHEN** client requests GET /unknown
- **THEN** server returns 404 Not Found

### Requirement: Security headers
The system SHALL include security headers in all responses.

#### Scenario: Response headers present
- **WHEN** server sends any response
- **THEN** headers include X-Content-Type-Options: nosniff, X-Frame-Options: DENY, Content-Security-Policy: default-src 'self'

### Requirement: Browser auto-open
The system SHALL automatically open browser when server starts.

#### Scenario: Server starts successfully
- **WHEN** server binds to port successfully
- **THEN** system opens http://127.0.0.1:{port} in default browser

#### Scenario: macOS browser open
- **WHEN** system is macOS
- **THEN** uses `open` command to open browser

#### Scenario: Linux browser open
- **WHEN** system is Linux
- **THEN** uses `xdg-open` command to open browser
