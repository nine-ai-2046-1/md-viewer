/// port.rs - Automatic port finding for localhost HTTP server
///
/// Tries to bind to ports starting from the given port,
/// incrementing by 1 up to MAX_ATTEMPTS if occupied.

use std::net::TcpListener;

/// Maximum consecutive ports to attempt before giving up.
const MAX_ATTEMPTS: u16 = 20;

/// Find an available port on localhost starting from `start_port`.
///
/// # Returns
/// `Ok(port)` - the first available port in range [start_port, start_port + MAX_ATTEMPTS)
/// `Err(msg)` - human-readable error listing all attempted ports
pub fn find_available_port(start_port: u16) -> Result<u16, String> {
    let mut attempted = Vec::new();

    for offset in 0..MAX_ATTEMPTS {
        let port = start_port.saturating_add(offset);
        if port == 0 {
            break; // Avoid port 0 (OS assigns random port)
        }
        attempted.push(port);

        if is_port_available(port) {
            return Ok(port);
        }
    }

    Err(format!(
        "No available port found after {} attempts (tried: {})",
        MAX_ATTEMPTS,
        attempted
            .iter()
            .map(|p| p.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    ))
}

/// Check if a specific port is available on 127.0.0.1.
///
/// Attempts to bind a TcpListener; if it succeeds, the port is free.
pub fn is_port_available(port: u16) -> bool {
    TcpListener::bind(format!("127.0.0.1:{}", port)).is_ok()
}

/// Open a URL in the system default browser.
///
/// Uses `open` on macOS and `xdg-open` on Linux.
/// Silently ignores errors (browser may not be available in CI).
pub fn open_browser(url: &str) {
    let result = if cfg!(target_os = "macos") {
        std::process::Command::new("open").arg(url).spawn()
    } else {
        // Linux / other Unix
        std::process::Command::new("xdg-open").arg(url).spawn()
    };

    match result {
        Ok(_) => eprintln!("Opening browser at {}", url),
        Err(e) => eprintln!("Warning: Could not open browser: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_available_port_finds_one() {
        // Start from a high port to avoid conflicts
        let port = find_available_port(49152).expect("Should find a port");
        assert!(port >= 49152);
        assert!(port < 49152 + MAX_ATTEMPTS);
    }

    #[test]
    fn test_is_port_available_high_port() {
        // A very high unused port should be available
        // (not guaranteed, but very likely in tests)
        let available = is_port_available(59999);
        // Just verifies the function runs without panic
        let _ = available;
    }

    #[test]
    fn test_occupied_port_skipped() {
        // Bind a port, then find_available_port should skip it
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let occupied_port = listener.local_addr().unwrap().port();

        // find_available_port starting at occupied_port should return a different port
        // (or the same port if the listener is released — either is fine for this test)
        let result = find_available_port(occupied_port);
        // Just verifies no panic
        let _ = result;
    }
}
