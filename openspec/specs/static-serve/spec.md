## Purpose

The static serve module provides HTTP server functionality to serve pre-built static sites without performing any build steps.

## Requirements

### Requirement: Serve mode starts HTTP server against a pre-built directory
The system SHALL, when `--serve <dir>` is the only mode flag, start a localhost-only HTTP server that serves files from `<dir>` without performing any build step.

#### Scenario: Serve a valid pre-built directory
- **WHEN** user runs `md-viewer --serve ./dist`
- **THEN** the system checks that `./dist/index.html` exists
- **THEN** the system binds to `127.0.0.1:<port>` (default 9141, auto-incremented if occupied)
- **THEN** the browser is opened at `http://127.0.0.1:<port>`
- **THEN** the server runs until the user interrupts (Ctrl+C)

#### Scenario: Serve respects --port flag
- **WHEN** user runs `md-viewer --serve ./dist --port 8080`
- **THEN** the server binds to `127.0.0.1:8080`
- **THEN** if port 8080 is occupied, the server auto-increments to the next available port

### Requirement: Serve mode rejects directory without index.html
The system SHALL refuse to start if `index.html` is not present in the target directory, to prevent silently serving a broken site.

#### Scenario: Missing index.html causes startup failure
- **WHEN** user runs `md-viewer --serve ./dist` and `./dist/index.html` does not exist
- **THEN** the system prints an error: "Error: ./dist/index.html not found. Run with --build first."
- **THEN** the process exits with code 1 without binding to any port

#### Scenario: Non-existent serve directory
- **WHEN** user runs `md-viewer --serve /nonexistent/path`
- **THEN** the system prints an error: "Error: Directory not found: /nonexistent/path"
- **THEN** the process exits with code 1

### Requirement: Serve mode applies existing security restrictions
The HTTP server started by `--serve` SHALL apply all security measures currently in place: localhost-only binding, path traversal prevention, security response headers, and directory listing rejection.

#### Scenario: Path traversal attempt is blocked
- **WHEN** a request is made with a path containing `..`
- **THEN** the server responds with HTTP 400

#### Scenario: Security headers are present on all responses
- **WHEN** any resource is served successfully
- **THEN** the response includes `X-Content-Type-Options: nosniff`, `X-Frame-Options: DENY`, and a Content-Security-Policy header

#### Scenario: Server only binds to localhost
- **WHEN** `--serve` starts the HTTP server
- **THEN** the server listens on `127.0.0.1` only, never `0.0.0.0` or `::`

### Requirement: Serve mode does not accept source or build flags
`--serve` SHALL be mutually exclusive with `--md`, `--mds`, and `--build`.

#### Scenario: Serve with --md rejected
- **WHEN** user provides both `--serve <dir>` and `--md <file>`
- **THEN** the system prints an error indicating the flags are mutually exclusive
- **THEN** the process exits with code 1

#### Scenario: Serve with --mds rejected
- **WHEN** user provides both `--serve <dir>` and `--mds <dir>`
- **THEN** the system prints an error indicating the flags are mutually exclusive
- **THEN** the process exits with code 1

### Requirement: Serve mode serves any file present in the directory
The server SHALL serve any file that exists under `<dir>`, subject to security checks, using correct MIME types.

#### Scenario: HTML page is served
- **WHEN** a GET request is made for `/readme` or `/readme.html`
- **THEN** the server responds with the contents of `<dir>/readme.html` and `Content-Type: text/html`

#### Scenario: Asset file is served
- **WHEN** a GET request is made for `/base/style.css`
- **THEN** the server responds with the contents of `<dir>/base/style.css` and `Content-Type: text/css`

#### Scenario: Missing file returns 404
- **WHEN** a GET request is made for a path that does not exist under `<dir>`
- **THEN** the server responds with HTTP 404
