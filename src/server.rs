/// server.rs - Localhost-only HTTP server for serving generated HTML files
///
/// Uses tiny_http for a minimal, sync HTTP server bound strictly to 127.0.0.1.
/// Serves: the generated HTML page(s) and base assets (CSS, JS).
/// Security: path traversal prevention, no directory browsing, security headers.

use std::fs;
use std::path::{Path, PathBuf};
use tiny_http::{Header, Request, Response, Server, StatusCode};

/// Server configuration.
pub struct ServerConfig {
    /// Port to bind on 127.0.0.1.
    pub port: u16,
    /// Security root: all served files must be within this directory.
    /// For standard modes: ~/.opens/md-viewer-data/
    /// For --serve mode: the user-supplied static directory.
    pub root_dir: PathBuf,
    /// Base directory for /base/* asset requests (CSS, JS).
    pub base_dir: PathBuf,
    /// Directory containing page HTML files.
    /// Standard mode: root_dir/pages/   Serve mode: root_dir/
    pub pages_dir: PathBuf,
    /// The HTML filename served at / and /index.html.
    pub index_file: String,
    /// Optional source directory for /source/* requests (images, videos, etc.).
    /// Only set when using --mds mode.
    pub source_dir: Option<PathBuf>,
}

/// Security headers added to every response.
const SECURITY_HEADERS: &[(&str, &str)] = &[
    ("X-Content-Type-Options", "nosniff"),
    ("X-Frame-Options", "DENY"),
    ("Content-Security-Policy", "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'"),
    ("Cache-Control", "no-store"),
    ("X-XSS-Protection", "1; mode=block"),
];

/// Start the HTTP server. Blocks until Ctrl+C or process termination.
///
/// # Arguments
/// * `config` - Server configuration
pub fn start(config: &ServerConfig) -> Result<(), String> {
    let bind_addr = format!("127.0.0.1:{}", config.port);

    let server = Server::http(&bind_addr)
        .map_err(|e| format!("Cannot bind server to {}: {}", bind_addr, e))?;

    eprintln!("Server running at http://{}", bind_addr);
    eprintln!("Press Ctrl+C to stop.");

    for request in server.incoming_requests() {
        handle_request(request, config);
    }

    Ok(())
}

/// Dispatch a single HTTP request.
fn handle_request(request: Request, config: &ServerConfig) {
    let url = request.url().to_string();
    let path = url.split('?').next().unwrap_or("/");

    let (status, content_type, body) = route(path, config);
    send_response(request, status, content_type, body);
}

/// Route a request path to a file or error.
///
/// Returns (HTTP status, content-type string, body bytes).
fn route(path: &str, config: &ServerConfig) -> (u16, &'static str, Vec<u8>) {
    // Reject paths with traversal attempts immediately
    if has_traversal(path) {
        return (403, "text/plain", b"403 Forbidden".to_vec());
    }

    // Normalize: strip leading slash
    let clean = path.trim_start_matches('/');

    // Route: / or /index.html → index page
    if clean.is_empty() || clean == "index.html" {
        return serve_page(&config.pages_dir.join(&config.index_file), &config.root_dir);
    }

    // Route: /base/* → base assets
    if let Some(asset_path) = clean.strip_prefix("base/") {
        return serve_base_asset(&config.base_dir, asset_path, &config.root_dir);
    }

    // Route: /source/* → source directory files (images, videos, etc.)
    if let Some(source_path) = clean.strip_prefix("source/") {
        if let Some(ref source_dir) = config.source_dir {
            return serve_source_file(source_dir, source_path);
        }
        return (404, "text/plain", b"404 Not Found".to_vec());
    }

    // Route: /pages/* → 403 (no direct access to the pages/ directory listing)
    if clean.starts_with("pages/") {
        return (403, "text/plain", b"403 Forbidden".to_vec());
    }

    // Route: /{name} or /{name}.html → serve from pages_dir if file exists within root_dir
    let html_name = if clean.ends_with(".html") {
        clean.to_string()
    } else {
        format!("{}.html", clean)
    };

    let file_path = config.pages_dir.join(&html_name);

    // Security: resolved path must be within root_dir and the file must actually exist
    if is_within(&config.root_dir, &file_path) && file_path.exists() {
        return serve_page(&file_path, &config.root_dir);
    }

    (404, "text/plain", b"404 Not Found".to_vec())
}

/// Serve a specific HTML page file.
///
/// Validates the resolved path is within root_dir before reading.
fn serve_page(file_path: &Path, root_dir: &Path) -> (u16, &'static str, Vec<u8>) {
    if !is_within(root_dir, file_path) {
        return (403, "text/plain", b"403 Forbidden".to_vec());
    }
    match fs::read(file_path) {
        Ok(bytes) => (200, "text/html; charset=utf-8", bytes),
        Err(_) => (404, "text/plain", b"404 Not Found".to_vec()),
    }
}

/// Serve an asset from the base directory.
///
/// Only files directly in base_dir (no subdirectory traversal).
fn serve_base_asset(base_dir: &Path, asset_path: &str, root_dir: &Path) -> (u16, &'static str, Vec<u8>) {
    // Reject traversal attempts
    if has_traversal(asset_path) || asset_path.contains('/') {
        return (403, "text/plain", b"403 Forbidden".to_vec());
    }

    let file_path = base_dir.join(asset_path);

    // Ensure the resolved path is within root_dir
    if !is_within(root_dir, &file_path) {
        return (403, "text/plain", b"403 Forbidden".to_vec());
    }

    match fs::read(&file_path) {
        Ok(bytes) => {
            let mime = content_type_for(asset_path);
            (200, mime, bytes)
        }
        Err(_) => (404, "text/plain", b"404 Not Found".to_vec()),
    }
}

/// Serve a file from the source directory (--mds mode).
///
/// Supports arbitrary subdirectory depths. Security: path traversal prevention,
/// confinement within source_dir, reject hidden files.
fn serve_source_file(source_dir: &Path, file_path: &str) -> (u16, &'static str, Vec<u8>) {
    // Reject traversal attempts
    if has_traversal(file_path) {
        return (403, "text/plain", b"403 Forbidden".to_vec());
    }

    // Reject hidden files/directories (paths starting with .)
    let parts: Vec<&str> = file_path.split('/').collect();
    for part in &parts {
        if part.starts_with('.') && !part.is_empty() {
            return (403, "text/plain", b"403 Forbidden".to_vec());
        }
    }

    let full_path = source_dir.join(file_path);

    // Ensure the resolved path is within source_dir (security: no traversal)
    if !is_within(source_dir, &full_path) {
        return (403, "text/plain", b"403 Forbidden".to_vec());
    }

    match fs::read(&full_path) {
        Ok(bytes) => {
            let mime = content_type_for(file_path);
            (200, mime, bytes)
        }
        Err(_) => (404, "text/plain", b"404 Not Found".to_vec()),
    }
}

/// Send an HTTP response with security headers.
fn send_response(request: Request, status: u16, content_type: &str, body: Vec<u8>) {
    let status_code = StatusCode(status);

    let mut headers: Vec<Header> = SECURITY_HEADERS
        .iter()
        .filter_map(|(k, v)| {
            Header::from_bytes(k.as_bytes(), v.as_bytes()).ok()
        })
        .collect();

    if let Ok(ct_header) = Header::from_bytes(b"Content-Type", content_type.as_bytes()) {
        headers.push(ct_header);
    }

    let response = Response::from_data(body)
        .with_status_code(status_code);

    // Add headers one by one
    let response = headers.into_iter().fold(response, |r, h| r.with_header(h));

    if let Err(e) = request.respond(response) {
        eprintln!("Failed to send response: {}", e);
    }
}

/// Check if a URL path contains directory traversal sequences.
fn has_traversal(path: &str) -> bool {
    path.contains("..") || path.contains('\0') || path.contains("//")
}

/// Check that `child` is within (or equal to) `parent`.
fn is_within(parent: &Path, child: &Path) -> bool {
    child
        .components()
        .zip(parent.components())
        .all(|(c, p)| c == p)
        || child.starts_with(parent)
}

/// Map file extension to MIME type.
fn content_type_for(filename: &str) -> &'static str {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "html" | "htm" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "application/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_traversal_detects_dotdot() {
        assert!(has_traversal("../etc/passwd"));
        assert!(has_traversal("/base/../../../etc"));
        assert!(!has_traversal("/base/style.css"));
        assert!(!has_traversal("/"));
    }

    #[test]
    fn test_has_traversal_null_byte() {
        assert!(has_traversal("/base/\0file"));
    }

    #[test]
    fn test_content_type_for() {
        assert_eq!(content_type_for("style.css"), "text/css; charset=utf-8");
        assert_eq!(content_type_for("app.js"), "application/javascript; charset=utf-8");
        assert_eq!(content_type_for("index.html"), "text/html; charset=utf-8");
    }

    #[test]
    fn test_is_within() {
        let base = Path::new("/home/user/.opens/md-viewer-data/base");
        let valid = Path::new("/home/user/.opens/md-viewer-data/base/style.css");
        let invalid = Path::new("/home/user/.opens/md-viewer-data/pages/index.html");
        assert!(is_within(base, valid));
        assert!(!is_within(base, invalid));
    }

    #[test]
    fn test_route_traversal_blocked() {
        let config = ServerConfig {
            port: 9000,
            root_dir: PathBuf::from("/tmp"),
            base_dir: PathBuf::from("/tmp"),
            pages_dir: PathBuf::from("/tmp"),
            index_file: "index.html".to_string(),
            source_dir: None,
        };
        let (status, _, _) = route("/../etc/passwd", &config);
        assert_eq!(status, 403);
    }

    #[test]
    fn test_route_pages_dir_blocked() {
        let config = ServerConfig {
            port: 9000,
            root_dir: PathBuf::from("/tmp"),
            base_dir: PathBuf::from("/tmp"),
            pages_dir: PathBuf::from("/tmp"),
            index_file: "index.html".to_string(),
            source_dir: None,
        };
        let (status, _, _) = route("/pages/", &config);
        assert_eq!(status, 403);
    }
}
